use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Map, Value};
use tokio::sync::mpsc;

use crate::api::server;
use crate::error::ApiError;
use crate::models::ServerRow;
use crate::state::{AppState, SharedState};
use crate::util;
use crate::ws_redis::RedisBridge;

type Sender = mpsc::UnboundedSender<Value>;

/// Active WebSocket connections and ephemeral device/op state.
#[derive(Default)]
pub struct WsRegistry {
    nodes: DashMap<u64, Sender>,
    machines: DashMap<u64, Sender>,
    machine_nodes: DashMap<u64, Vec<u64>>,
    /// node_id -> (user_id -> IPs)
    devices: DashMap<u64, HashMap<i64, Vec<String>>>,
    acks: DashMap<String, Value>,
    logs: DashMap<u64, Value>,
    bridge: OnceLock<Arc<RedisBridge>>,
    pub remote_nodes: DashMap<u64, i64>,
    pub remote_machines: DashMap<u64, i64>,
    pub remote_machine_nodes: DashMap<u64, Vec<u64>>,
    pub remote_devices: DashMap<u64, HashMap<i64, Vec<String>>>,
    pub remote_acks: DashMap<String, Value>,
    pub remote_logs: DashMap<u64, Value>,
}

impl WsRegistry {
    pub fn set_bridge(&self, bridge: Arc<RedisBridge>) {
        let _ = self.bridge.set(bridge);
    }

    pub fn publish(&self, payload: Value) {
        if let Some(bridge) = self.bridge.get() {
            bridge.publish(payload);
        }
    }

    fn remote_alive(map: &DashMap<u64, i64>, id: u64) -> bool {
        map.get(&id)
            .map(|expiry| *expiry > util::now())
            .unwrap_or(false)
    }

    pub fn node_is_remote(&self, node_id: u64) -> bool {
        !self.nodes.contains_key(&node_id) && Self::remote_alive(&self.remote_nodes, node_id)
    }

    pub fn is_connected(&self, node_id: u64) -> bool {
        self.nodes.contains_key(&node_id) || Self::remote_alive(&self.remote_nodes, node_id)
    }

    pub fn machine_connected(&self, machine_id: u64) -> bool {
        self.machines.contains_key(&machine_id)
            || Self::remote_alive(&self.remote_machines, machine_id)
    }

    pub fn deliver_node_local(&self, node_id: u64, event: &str, data: Value) -> bool {
        let Some(sender) = self.nodes.get(&node_id).map(|v| v.clone()) else {
            return false;
        };
        sender
            .send(json!({"event": event, "data": data, "timestamp": util::now()}))
            .is_ok()
    }

    pub fn deliver_machine_local(&self, machine_id: u64, event: &str, data: Value) -> bool {
        let Some(sender) = self.machines.get(&machine_id).map(|v| v.clone()) else {
            return false;
        };
        sender
            .send(json!({"event": event, "data": data, "timestamp": util::now()}))
            .is_ok()
    }

    pub fn send_node(&self, node_id: u64, event: &str, data: Value) -> bool {
        if let Some(sender) = self.nodes.get(&node_id).map(|v| v.clone()) {
            if sender
                .send(json!({"event": event, "data": data.clone(), "timestamp": util::now()}))
                .is_ok()
            {
                return true;
            }
        }
        if self.bridge.get().is_some() {
            self.publish(json!({"kind": "node_event", "id": node_id, "event": event, "data": data}));
            true
        } else {
            false
        }
    }

    pub fn send_machine(&self, machine_id: u64, event: &str, data: Value) -> bool {
        if let Some(sender) = self.machines.get(&machine_id).map(|v| v.clone()) {
            if sender
                .send(json!({"event": event, "data": data.clone(), "timestamp": util::now()}))
                .is_ok()
            {
                return true;
            }
        }
        if self.bridge.get().is_some() {
            self.publish(json!({"kind": "machine_event", "id": machine_id, "event": event, "data": data}));
            true
        } else {
            false
        }
    }

    pub fn machine_node_ids(&self, machine_id: u64) -> Vec<u64> {
        self.machine_nodes
            .get(&machine_id)
            .map(|v| v.clone())
            .or_else(|| self.remote_machine_nodes.get(&machine_id).map(|v| v.clone()))
            .unwrap_or_default()
    }

    pub fn connected_node_ids(&self) -> Vec<u64> {
        let now = util::now();
        let mut ids: Vec<u64> = self.nodes.iter().map(|v| *v.key()).collect();
        for entry in self.remote_nodes.iter() {
            if *entry.value() > now {
                ids.push(*entry.key());
            }
        }
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    pub fn connected_machine_ids(&self) -> Vec<u64> {
        let now = util::now();
        let mut ids: Vec<u64> = self.machines.iter().map(|v| *v.key()).collect();
        for entry in self.remote_machines.iter() {
            if *entry.value() > now {
                ids.push(*entry.key());
            }
        }
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    pub fn set_node_devices(&self, node_id: u64, devices: HashMap<i64, Vec<String>>) {
        self.devices.insert(node_id, devices.clone());
        if self.bridge.get().is_some() {
            let map: Map<String, Value> = devices
                .iter()
                .map(|(user, ips)| (user.to_string(), json!(ips)))
                .collect();
            self.publish(json!({"kind": "devices", "id": node_id, "devices": Value::Object(map)}));
        }
    }

    pub fn remove_node_devices(&self, node_id: u64) {
        self.devices.remove(&node_id);
        if self.bridge.get().is_some() {
            self.publish(json!({"kind": "devices_remove", "id": node_id}));
        }
    }

    pub fn active_devices(&self) -> HashMap<i64, Vec<String>> {
        let mut merged: HashMap<i64, HashSet<String>> = HashMap::new();
        for entry in self.devices.iter() {
            for (user_id, ips) in entry.value() {
                let slot = merged.entry(*user_id).or_default();
                for ip in ips {
                    slot.insert(ip.clone());
                }
            }
        }
        for entry in self.remote_devices.iter() {
            for (user_id, ips) in entry.value() {
                let slot = merged.entry(*user_id).or_default();
                for ip in ips {
                    slot.insert(ip.clone());
                }
            }
        }
        merged
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect()
    }

    pub fn node_devices(&self, node_id: u64) -> HashMap<i64, Vec<String>> {
        self.devices
            .get(&node_id)
            .map(|v| v.clone())
            .or_else(|| self.remote_devices.get(&node_id).map(|v| v.clone()))
            .unwrap_or_default()
    }

    pub fn store_ack(&self, machine_id: u64, op: &str, payload: Value) {
        self.acks.insert(format!("{machine_id}:{op}"), payload.clone());
        if self.bridge.get().is_some() {
            self.publish(json!({"kind": "ack", "id": machine_id, "op": op, "payload": payload}));
        }
    }

    pub fn get_ack(&self, machine_id: u64, op: &str) -> Option<Value> {
        let key = format!("{machine_id}:{op}");
        self.acks
            .get(&key)
            .map(|v| v.clone())
            .or_else(|| self.remote_acks.get(&key).map(|v| v.clone()))
    }

    pub fn acks_for(&self, machine_id: u64) -> Map<String, Value> {
        let prefix = format!("{machine_id}:");
        let mut out = Map::new();
        for entry in self.acks.iter() {
            if let Some(op) = entry.key().strip_prefix(&prefix) {
                out.insert(op.to_string(), entry.value().clone());
            }
        }
        for entry in self.remote_acks.iter() {
            if let Some(op) = entry.key().strip_prefix(&prefix) {
                out.insert(op.to_string(), entry.value().clone());
            }
        }
        out
    }

    pub fn store_logs(&self, machine_id: u64, payload: Value) {
        self.logs.insert(machine_id, payload.clone());
        if self.bridge.get().is_some() {
            self.publish(json!({"kind": "logs", "id": machine_id, "payload": payload}));
        }
    }

    pub fn get_logs(&self, machine_id: u64) -> Option<Value> {
        self.logs
            .get(&machine_id)
            .map(|v| v.clone())
            .or_else(|| self.remote_logs.get(&machine_id).map(|v| v.clone()))
    }
}

pub async fn ws_handler(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Query(query): Query<HashMap<String, String>>,
    upgrade: WebSocketUpgrade,
) -> Response {
    let token = util::bearer_token(
        headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok()),
    )
    .or_else(|| query.get("token").cloned())
    .unwrap_or_default();

    let settings = state.settings().await;
    if !settings.bool("server_ws_enable", true) {
        return (StatusCode::SERVICE_UNAVAILABLE, "websocket disabled").into_response();
    }

    let machine_id = query
        .get("machine_id")
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v > 0);

    if let Some(machine_id) = machine_id {
        let machine: Option<(u64, String, i64)> = match sqlx::query_as(
            "SELECT id, token, is_active FROM v2_server_machine WHERE id = ? LIMIT 1",
        )
        .bind(machine_id)
        .fetch_optional(&state.db)
        .await
        {
            Ok(v) => v,
            Err(err) => {
                tracing::error!(error = %err, "ws machine lookup failed");
                return (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response();
            }
        };
        let Some((id, db_token, active)) = machine else {
            return (StatusCode::UNAUTHORIZED, "invalid machine credentials").into_response();
        };
        if db_token != token || active == 0 {
            return (StatusCode::UNAUTHORIZED, "invalid machine credentials").into_response();
        }
        let nodes: Vec<ServerRow> = sqlx::query_as(
            "SELECT * FROM v2_server WHERE machine_id = ? AND (enabled IS NULL OR enabled = 1) ORDER BY sort ASC",
        )
        .bind(id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

        return upgrade
            .on_upgrade(move |socket| async move {
                connection_machine(state, socket, id, nodes).await;
            })
            .into_response();
    }

    // Single-node auth.
    let node_id = query
        .get("node_id")
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v > 0);
    let Some(node_id) = node_id else {
        return (StatusCode::UNAUTHORIZED, "node_id is required").into_response();
    };
    if token != settings.str("server_token", "") {
        return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
    }
    let node: Option<ServerRow> = match sqlx::query_as("SELECT * FROM v2_server WHERE id = ?")
        .bind(node_id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(v) => v,
        Err(_) => None,
    };
    let Some(node) = node else {
        return (StatusCode::NOT_FOUND, "node not found").into_response();
    };

    upgrade
        .on_upgrade(move |socket| async move {
            connection_node(state, socket, node).await;
        })
        .into_response()
}

async fn connection_node(state: SharedState, socket: WebSocket, node: ServerRow) {
    let node_id = node.id;
    let (tx, mut rx) = mpsc::unbounded_channel::<Value>();
    state.ws.nodes.insert(node_id, tx.clone());
    state.ws.publish(json!({"kind": "node_online", "id": node_id}));
    state.cache.put(
        format!("node_ws_alive:{node_id}"),
        "1",
        86400,
    );

    let mut socket = socket;
    let send = |value: Value| {
        let tx = tx.clone();
        async move {
            let _ = tx.send(value);
        }
    };

    // auth.success then full sync (matches Workerman contract).
    let _ = send(json!({
        "event": "auth.success",
        "data": {"node_id": node_id}
    }))
    .await;
    push_full_sync(&state, node_id, &node, &tx).await;

    run_socket(state.clone(), socket, rx, Some(node_id), None).await;
    state.ws.nodes.remove(&node_id);
    state.ws.publish(json!({"kind": "node_offline", "id": node_id}));
    state.ws.remove_node_devices(node_id);
    state.cache.put(
        format!("node_ws_alive:{node_id}"),
        "0",
        60,
    );
}

async fn connection_machine(
    state: SharedState,
    socket: WebSocket,
    machine_id: u64,
    nodes: Vec<ServerRow>,
) {
    let (tx, mut rx) = mpsc::unbounded_channel::<Value>();
    state.ws.machines.insert(machine_id, tx.clone());
    let node_ids: Vec<u64> = nodes.iter().map(|n| n.id).collect();
    state.ws.publish(json!({
        "kind": "machine_online",
        "id": machine_id,
        "node_ids": node_ids
    }));
    state
        .ws
        .machine_nodes
        .insert(machine_id, node_ids.clone());
    for id in &node_ids {
        state.ws.nodes.insert(*id, tx.clone());
        state
            .cache
            .put(format!("node_ws_alive:{id}"), "1", 86400);
    }
    let settings = state.settings().await;
    let _ = tx.send(json!({
        "event": "auth.success",
        "data": {
            "machine_id": machine_id,
            "node_ids": node_ids,
            "nodes": nodes.iter().map(|n| json!({"id": n.id, "type": n.r#type, "name": n.name})).collect::<Vec<_>>(),
            "base_config": {
                "push_interval": settings.i64("server_push_interval", 60),
                "pull_interval": settings.i64("server_pull_interval", 60),
            }
        }
    }));

    for node in &nodes {
        push_full_sync(&state, node.id, node, &tx).await;
    }

    run_socket(state.clone(), socket, rx, None, Some(machine_id)).await;

    state.ws.machines.remove(&machine_id);
    state.ws.machine_nodes.remove(&machine_id);
    state.ws.publish(json!({"kind": "machine_offline", "id": machine_id}));
    for id in node_ids {
        state.ws.nodes.remove(&id);
        state.ws.publish(json!({"kind": "node_offline", "id": id}));
        state.ws.remove_node_devices(id);
        state
            .cache
            .put(format!("node_ws_alive:{id}"), "0", 60);
    }
}

async fn push_full_sync(state: &SharedState, node_id: u64, node: &ServerRow, tx: &Sender) {
    let config = server::build_node_config(state, node).await;
    let users = server::build_node_users(state, node).await.unwrap_or_else(|_| json!([]));
    let _ = tx.send(json!({
        "event": "sync.config",
        "data": {"config": config, "node_id": node_id, "timestamp": util::now()}
    }));
    let _ = tx.send(json!({
        "event": "sync.users",
        "data": {"users": users, "node_id": node_id, "timestamp": util::now()}
    }));
}

async fn run_socket(
    state: SharedState,
    socket: WebSocket,
    rx: mpsc::UnboundedReceiver<Value>,
    node_id: Option<u64>,
    machine_id: Option<u64>,
) {
    let (mut sink, mut stream) = socket.split();
    let mut rx = rx;
    let mut ping = tokio::time::interval(Duration::from_secs(55));
    ping.tick().await;

    loop {
        tokio::select! {
            outbound = rx.recv() => {
                match outbound {
                    Some(value) => {
                        if sink
                            .send(Message::Text(value.to_string().into()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = ping.tick() => {
                if sink.send(Message::Text(json!({"event": "ping"}).to_string().into())).await.is_err() {
                    break;
                }
                if let Some(id) = node_id {
                    state.ws.publish(json!({"kind": "node_alive", "id": id}));
                }
                if let Some(id) = machine_id {
                    state.ws.publish(json!({"kind": "machine_alive", "id": id}));
                }
            }
            incoming = stream.next() => {
                let Some(Ok(message)) = incoming else { break };
                match message {
                    Message::Text(text) => {
                        if let Ok(value) = serde_json::from_str::<Value>(&text) {
                            if handle_event(&state, value, node_id, machine_id).await.is_err() {
                                break;
                            }
                        }
                    }
                    Message::Binary(bytes) => {
                        if let Ok(value) = serde_json::from_slice::<Value>(&bytes) {
                            if handle_event(&state, value, node_id, machine_id).await.is_err() {
                                break;
                            }
                        }
                    }
                    Message::Close(_) => break,
                    Message::Ping(_) | Message::Pong(_) => {}
                }
            }
        }
    }
}

async fn handle_event(
    state: &SharedState,
    message: Value,
    node_id: Option<u64>,
    machine_id: Option<u64>,
) -> Result<(), ()> {
    let event = message
        .get("event")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let data = message.get("data").cloned().unwrap_or(Value::Null);

    let resolved_node = node_id.or_else(|| {
        data.get("node_id")
            .and_then(|v| v.as_u64())
            .or_else(|| machine_id.and_then(|m| state.ws.machine_node_ids(m).first().copied()))
    });

    match event.as_str() {
        "pong" => {
            if let Some(id) = resolved_node {
                state.cache.put(format!("node_ws_alive:{id}"), "1", 86400);
            }
        }
        "node.status" => {
            let Some(id) = resolved_node else { return Ok(()) };
            let node: Option<ServerRow> = sqlx::query_as("SELECT * FROM v2_server WHERE id = ?")
                .bind(id)
                .fetch_optional(&state.db)
                .await
                .ok()
                .flatten();
            if let Some(node) = node {
                if let Err(err) = server::apply_report(state, &node, &data).await {
                    tracing::warn!(error = %err, node = id, "ws node.status failed");
                }
            }
        }
        "machine.status" => {
            let Some(id) = machine_id else { return Ok(()) };
            if let Err(err) = server::apply_machine_status(state, id, &data).await {
                tracing::warn!(error = %err, machine = id, "ws machine.status failed");
            }
        }
        "report.devices" => {
            let Some(id) = resolved_node else { return Ok(()) };
            let payload = data.get("devices").cloned().unwrap_or(data.clone());
            let mut devices: HashMap<i64, Vec<String>> = HashMap::new();
            if let Some(map) = payload.as_object() {
                for (uid, ips) in map {
                    let Ok(user_id) = uid.parse::<i64>() else { continue };
                    if user_id <= 0 {
                        continue;
                    }
                    let list = ips
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
                    devices.insert(user_id, list);
                }
            }
            state.ws.set_node_devices(id, devices.clone());
            update_online_counts(state, &devices).await;
            broadcast_devices(state);
        }
        "request.devices" => {
            let Some(id) = resolved_node else { return Ok(()) };
            let devices = state.ws.active_devices();
            state
                .ws
                .send_node(id, "sync.devices", json!({"users": devices}));
        }
        "request.sync" => {
            let Some(id) = resolved_node else { return Ok(()) };
            let node: Option<ServerRow> = sqlx::query_as("SELECT * FROM v2_server WHERE id = ?")
                .bind(id)
                .fetch_optional(&state.db)
                .await
                .ok()
                .flatten();
            if let Some(node) = node {
                if let Some(tx) = state.ws.nodes.get(&id).map(|v| v.clone()) {
                    push_full_sync(state, id, &node, &tx).await;
                }
            }
        }
        "report.logs" => {
            if let Some(id) = machine_id {
                let lines = data
                    .get("lines")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .take(1000)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                state.ws.store_logs(
                    id,
                    json!({
                        "lines": lines,
                        "req_id": data.get("req_id").cloned().unwrap_or(Value::Null),
                        "updated_at": util::now(),
                        "count": lines.len(),
                    }),
                );
            }
        }
        "op.ack" => {
            if let Some(id) = machine_id {
                let op = data.get("op").and_then(|v| v.as_str()).unwrap_or("");
                state.ws.store_ack(
                    id,
                    op,
                    json!({
                        "op": op,
                        "status": data.get("status").cloned().unwrap_or(Value::Null),
                        "detail": data.get("detail").cloned().unwrap_or(Value::Null),
                        "ts": data.get("ts").cloned().unwrap_or(json!(util::now())),
                    }),
                );
            }
        }
        "ping" => {
            // Node initiated ping: answer on the same connection is not
            // available here; the periodic server ping covers liveness.
        }
        _ => {}
    }
    Ok(())
}

async fn update_online_counts(state: &SharedState, devices: &HashMap<i64, Vec<String>>) {
    let now = util::now();
    for (user_id, ips) in devices {
        let _ = sqlx::query(
            "UPDATE v2_user SET online_count = ?, last_online_at = FROM_UNIXTIME(?), updated_at = ? WHERE id = ?",
        )
        .bind(ips.len() as i64)
        .bind(now)
        .bind(now)
        .bind(user_id)
        .execute(&state.db)
        .await;
    }
}

/// Push the fleet-wide device snapshot to every connected node.
pub fn broadcast_devices(state: &SharedState) {
    let devices = state.ws.active_devices();
    let payload = json!({"users": devices});
    for node_id in state.ws.connected_node_ids() {
        state.ws.send_node(node_id, "sync.devices", payload.clone());
    }
}

/// Push config/users to a node only when it is connected to this instance.
pub async fn push_node_sync_local(state: &SharedState, node_id: u64) -> bool {
    let Some(tx) = state.ws.nodes.get(&node_id).map(|v| v.clone()) else {
        return false;
    };
    let node: Option<ServerRow> = sqlx::query_as("SELECT * FROM v2_server WHERE id = ?")
        .bind(node_id)
        .fetch_optional(&state.db)
        .await
        .ok()
        .flatten();
    if let Some(node) = node {
        push_full_sync(state, node_id, &node, &tx).await;
        true
    } else {
        false
    }
}

/// Push updated config/users to a node after an admin change.
pub async fn push_node_sync(state: &SharedState, node_id: u64) {
    if push_node_sync_local(state, node_id).await {
        return;
    }
    if state.ws.node_is_remote(node_id) {
        state
            .ws
            .publish(json!({"kind": "sync_node", "id": node_id}));
    }
}

/// Send `sync.nodes` to a machine after its node set changes.
pub fn push_machine_nodes(state: &SharedState, machine_id: u64, nodes: &[ServerRow]) {
    if !state.ws.machine_connected(machine_id) {
        return;
    }
    state.ws.send_machine(
        machine_id,
        "sync.nodes",
        json!({
            "nodes": nodes.iter().map(|n| json!({"id": n.id, "type": n.r#type, "name": n.name})).collect::<Vec<_>>()
        }),
    );
}

/// Remote ops helper used by admin endpoints.
pub fn send_machine_op(
    state: &SharedState,
    machine_id: u64,
    event: &str,
    data: Value,
) -> Result<(), ApiError> {
    if state.ws.send_machine(machine_id, event, data) {
        Ok(())
    } else {
        Err(ApiError::bad_request("机器未通过 WebSocket 连接"))
    }
}

pub async fn handshake_payload(state: &SharedState, headers: &HeaderMap) -> Value {
    let settings = state.settings().await;
    if !settings.bool("server_ws_enable", true) {
        return json!({"websocket": {"enabled": false}});
    }
    let custom = settings.str("server_ws_url", "");
    let url = if custom.trim().is_empty() {
        let host = headers
            .get(axum::http::header::HOST)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("127.0.0.1");
        let scheme = headers
            .get("x-forwarded-proto")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("http");
        let ws_scheme = if scheme == "https" { "wss" } else { "ws" };
        format!("{ws_scheme}://{host}/ws")
    } else {
        custom.trim_end_matches('/').to_string()
    };
    json!({"websocket": {"enabled": true, "ws_url": url}})
}
