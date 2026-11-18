use std::sync::Arc;
use std::time::Duration;

use futures_util::StreamExt;
use serde_json::{json, Value};

use crate::state::SharedState;
use crate::util;

const CHANNEL: &str = "fboard:ws";
const REMOTE_TTL: i64 = 180;

/// Optional Redis bridge for multi-instance WebSocket fan-out. When Redis is
/// not configured the panel keeps its in-process registry behaviour.
pub struct RedisBridge {
    client: redis::Client,
    manager: redis::aio::ConnectionManager,
    instance_id: String,
}

impl RedisBridge {
    pub async fn connect(url: &str) -> anyhow::Result<Arc<Self>> {
        let client = redis::Client::open(url)?;
        let manager = client.get_connection_manager().await?;
        Ok(Arc::new(Self {
            client,
            manager,
            instance_id: util::guid(),
        }))
    }

    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    /// Fire-and-forget publish; callers are synchronous WS/registry helpers.
    pub fn publish(&self, payload: Value) {
        let mut manager = self.manager.clone();
        let instance_id = self.instance_id.clone();
        let mut payload = payload;
        if let Some(object) = payload.as_object_mut() {
            object.insert("origin".into(), json!(instance_id));
        }
        tokio::spawn(async move {
            let text = payload.to_string();
            let result: redis::RedisResult<i64> = redis::cmd("PUBLISH")
                .arg(CHANNEL)
                .arg(text)
                .query_async(&mut manager)
                .await;
            if let Err(err) = result {
                tracing::warn!(error = %err, "redis publish failed");
            }
        });
    }

    pub async fn run(self: Arc<Self>, state: SharedState) {
        loop {
            match self.client.get_async_pubsub().await {
                Ok(mut pubsub) => {
                    if let Err(err) = pubsub.subscribe(CHANNEL).await {
                        tracing::warn!(error = %err, "redis subscribe failed");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                    tracing::info!("redis WebSocket bridge subscribed");
                    let mut messages = pubsub.on_message();
                    while let Some(message) = messages.next().await {
                        let payload = message.get_payload::<String>().unwrap_or_default();
                        let Ok(value) = serde_json::from_str::<Value>(&payload) else {
                            continue;
                        };
                        if value.get("origin").and_then(|v| v.as_str())
                            == Some(self.instance_id.as_str())
                        {
                            continue;
                        }
                        handle(&state, value).await;
                    }
                }
                Err(err) => {
                    tracing::warn!(error = %err, "redis pubsub connect failed");
                }
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}

async fn handle(state: &SharedState, payload: Value) {
    let kind = payload.get("kind").and_then(|v| v.as_str()).unwrap_or("");
    let id = payload.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
    let now = util::now();
    match kind {
        "node_online" => {
            state.ws.remote_nodes.insert(id, now + REMOTE_TTL);
        }
        "node_alive" => {
            state.ws.remote_nodes.insert(id, now + REMOTE_TTL);
        }
        "node_offline" => {
            state.ws.remote_nodes.remove(&id);
            state.ws.remote_devices.remove(&id);
        }
        "machine_alive" => {
            state.ws.remote_machines.insert(id, now + REMOTE_TTL);
            if let Some(nodes) = state.ws.remote_machine_nodes.get(&id).map(|v| v.clone()) {
                for node in nodes {
                    state.ws.remote_nodes.insert(node, now + REMOTE_TTL);
                }
            }
        }
        "machine_online" => {
            state.ws.remote_machines.insert(id, now + REMOTE_TTL);
            if let Some(nodes) = payload.get("node_ids").and_then(|v| v.as_array()) {
                let ids: Vec<u64> = nodes.iter().filter_map(|v| v.as_u64()).collect();
                for node in &ids {
                    state.ws.remote_nodes.insert(*node, now + REMOTE_TTL);
                }
                state.ws.remote_machine_nodes.insert(id, ids);
            }
        }
        "machine_offline" => {
            state.ws.remote_machines.remove(&id);
            if let Some((_, nodes)) = state.ws.remote_machine_nodes.remove(&id) {
                for node in nodes {
                    state.ws.remote_nodes.remove(&node);
                    state.ws.remote_devices.remove(&node);
                }
            }
        }
        "node_event" => {
            state.ws.remote_nodes.insert(id, now + REMOTE_TTL);
            let event = payload.get("event").and_then(|v| v.as_str()).unwrap_or("");
            let data = payload.get("data").cloned().unwrap_or(Value::Null);
            state.ws.deliver_node_local(id, event, data);
        }
        "machine_event" => {
            state.ws.remote_machines.insert(id, now + REMOTE_TTL);
            let event = payload.get("event").and_then(|v| v.as_str()).unwrap_or("");
            let data = payload.get("data").cloned().unwrap_or(Value::Null);
            state.ws.deliver_machine_local(id, event, data);
        }
        "sync_node" => {
            crate::ws::push_node_sync_local(state, id).await;
        }
        "devices" => {
            let devices = payload
                .get("devices")
                .and_then(|v| v.as_object())
                .map(|map| {
                    map.iter()
                        .filter_map(|(user, ips)| {
                            let user = user.parse::<i64>().ok()?;
                            let ips = ips
                                .as_array()?
                                .iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect::<Vec<_>>();
                            Some((user, ips))
                        })
                        .collect()
                })
                .unwrap_or_default();
            state.ws.remote_devices.insert(id, devices);
        }
        "devices_remove" => {
            state.ws.remote_devices.remove(&id);
        }
        "ack" => {
            let op = payload.get("op").and_then(|v| v.as_str()).unwrap_or("");
            let data = payload.get("payload").cloned().unwrap_or(Value::Null);
            state.ws.remote_acks.insert(format!("{id}:{op}"), data);
        }
        "logs" => {
            let data = payload.get("payload").cloned().unwrap_or(Value::Null);
            state.ws.remote_logs.insert(id, data);
        }
        _ => {}
    }
}
