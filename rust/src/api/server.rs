use std::collections::HashMap;

use axum::body::Bytes;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Map, Value};

use crate::cache::keys;
use crate::error::{ApiError, ApiResult};
use crate::models::{ServerRow, UserRow};
use crate::plugin::PluginContext;
use crate::response::str_field;
use crate::state::SharedState;
use crate::util;

#[derive(Clone)]
struct NodeAuth {
    node: Option<ServerRow>,
    machine: Option<(u64, String)>,
}

fn node_auth_params(
    query: &HashMap<String, String>,
    body: Option<&Value>,
) -> HashMap<String, String> {
    let mut map = query.clone();
    if let Some(Value::Object(obj)) = body {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                map.insert(k.clone(), s.to_string());
            } else if let Some(n) = v.as_i64() {
                map.insert(k.clone(), n.to_string());
            }
        }
    }
    map
}

async fn authenticate(
    state: &SharedState,
    params: &HashMap<String, String>,
    allow_handshake: bool,
) -> ApiResult<NodeAuth> {
    let token = params
        .get("token")
        .cloned()
        .ok_or_else(|| ApiError::new(StatusCode::UNAUTHORIZED, "token is required"))?;
    let settings = state.settings().await;

    if let Some(machine_id) = params.get("machine_id").and_then(|v| v.parse::<u64>().ok()) {
        let machine: Option<(u64, String, i64)> = sqlx::query_as(
            "SELECT id, token, is_active FROM v2_server_machine WHERE id = ? LIMIT 1",
        )
        .bind(machine_id)
        .fetch_optional(&state.db)
        .await?;
        let Some((id, db_token, is_active)) = machine else {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED, "Machine not found or invalid token"));
        };
        if db_token != token {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED, "Machine not found or invalid token"));
        }
        if is_active == 0 {
            return Err(ApiError::forbidden("Machine is disabled"));
        }
        let node_id = params.get("node_id").and_then(|v| v.parse::<u64>().ok());
        let node = match node_id {
            Some(node_id) if node_id > 0 => {
                let node: Option<ServerRow> = sqlx::query_as(
                    "SELECT * FROM v2_server WHERE id = ? AND machine_id = ? AND (enabled IS NULL OR enabled = 1) LIMIT 1",
                )
                .bind(node_id)
                .bind(id)
                .fetch_optional(&state.db)
                .await?;
                Some(node.ok_or_else(|| ApiError::forbidden("Node not found on this machine"))?)
            }
            _ => None,
        };
        let _ = sqlx::query("UPDATE v2_server_machine SET last_seen_at = ?, updated_at = NOW() WHERE id = ?")
            .bind(util::now())
            .bind(id)
            .execute(&state.db)
            .await;
        return Ok(NodeAuth {
            node,
            machine: Some((id, token)),
        });
    }

    if token != settings.str("server_token", "") {
        return Err(ApiError::new(StatusCode::UNAUTHORIZED, "Invalid token"));
    }
    let node_id = params.get("node_id");
    if node_id.is_none() && !allow_handshake {
        return Err(ApiError::bad_request("node_id is required"));
    }
    let node = match node_id {
        Some(node_id) if !node_id.is_empty() => {
            let node: Option<ServerRow> = sqlx::query_as(
                "SELECT * FROM v2_server WHERE (code = ? OR id = ?) ORDER BY CASE WHEN code = ? THEN 0 ELSE 1 END LIMIT 1",
            )
            .bind(node_id)
            .bind(node_id.parse::<i64>().unwrap_or(0))
            .bind(node_id)
            .fetch_optional(&state.db)
            .await?;
            Some(node.ok_or_else(|| ApiError::bad_request("Server does not exist"))?)
        }
        _ => None,
    };
    Ok(NodeAuth { node, machine: None })
}

pub(crate) fn touch_node(state: &SharedState, node: &ServerRow) {
    state.cache.put(
        format!(
            "SERVER_{}_LAST_CHECK_AT_{}",
            node.r#type.to_uppercase(),
            node.id
        ),
        util::now().to_string(),
        3600,
    );
}

pub async fn handshake(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
    body: Bytes,
) -> ApiResult<Response> {
    let body_json = if body.is_empty() {
        None
    } else {
        serde_json::from_slice::<Value>(&body).ok()
    };
    let params = node_auth_params(&query, body_json.as_ref());
    authenticate(&state, &params, true).await?;
    Ok(Json(crate::ws::handshake_payload(&state, &headers).await).into_response())
}

pub async fn report(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
    body: Bytes,
) -> ApiResult<Response> {
    let body_json: Value = serde_json::from_slice(&body)
        .map_err(|_| ApiError::bad_request("Invalid data format"))?;
    let params = node_auth_params(&query, Some(&body_json));
    let auth = authenticate(&state, &params, false).await?;
    let node = auth
        .node
        .ok_or_else(|| ApiError::bad_request("node_id is required"))?;

    if let Some(report_id) = body_json.get("report_id").and_then(|v| v.as_str()) {
        if !report_id.is_empty() {
            if !report_id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "_.:-".contains(c))
                || report_id.len() > 64
            {
                return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "Invalid report_id"));
            }
            let key = format!("server_report:{}:{}", node.id, report_id);
            if state.cache.get(&key).is_some() {
                return Ok(Json(json!({"data": true, "duplicate": true})).into_response());
            }
            state.cache.put(key, "1", 3600);
        }
    }

    touch_node(&state, &node);
    apply_report(&state, &node, &body_json).await?;
    Ok(Json(json!({"data": true})).into_response())
}

pub(crate) async fn apply_report(
    state: &SharedState,
    node: &ServerRow,
    body_json: &Value,
) -> ApiResult<()> {
    touch_node(state, node);
    if let Some(traffic) = body_json.get("traffic") {
        process_traffic(state, node, traffic).await?;
    }
    if let Some(alive) = body_json.get("alive") {
        process_alive(state, node, alive).await?;
    }
    if let Some(online) = body_json.get("online") {
        process_online(state, node, online).await?;
    }
    if let Some(status) = body_json.get("status") {
        process_status(state, node, status).await?;
    }
    if let Some(metrics) = body_json.get("metrics") {
        if !metrics.is_null() {
            state.cache.put(
                format!(
                    "SERVER_{}_METRICS_{}",
                    node.r#type.to_uppercase(),
                    node.id
                ),
                metrics.to_string(),
                3600,
            );
        }
    }
    Ok(())
}

pub async fn config(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let params = node_auth_params(&query, None);
    let auth = authenticate(&state, &params, false).await?;
    let node = auth
        .node
        .ok_or_else(|| ApiError::bad_request("node_id is required"))?;
    touch_node(&state, &node);
    Ok(Json(build_node_config(&state, &node).await).into_response())
}

pub async fn build_node_config(state: &SharedState, node: &ServerRow) -> Value {
    let settings = state.settings().await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = PluginContext::new(&state.db, &settings, &app_url, json!({}));
    let mut response: Value = state
        .plugin
        .server_config(
            json!({
                "node": node_json_for_plugin(node),
                "settings": settings.public_map(),
            }),
            &ctx,
        )
        .await
        .unwrap_or_else(|| generic_node_config(node, &settings));

    if let Some(obj) = response.as_object_mut() {
        obj.insert(
            "base_config".into(),
            json!({
                "push_interval": settings.i64("server_push_interval", 60),
                "pull_interval": settings.i64("server_pull_interval", 60),
            }),
        );
    }
    response
}

fn generic_node_config(node: &ServerRow, settings: &crate::settings::Settings) -> Value {
    let protocol_settings = node.settings();
    let mut config = Map::new();
    config.insert("protocol".into(), json!(node.r#type));
    config.insert("listen_ip".into(), json!("0.0.0.0"));
    config.insert("server_port".into(), json!(node.server_port));
    config.insert(
        "network".into(),
        protocol_settings
            .get("network")
            .cloned()
            .unwrap_or(Value::Null),
    );
    config.insert(
        "networkSettings".into(),
        protocol_settings
            .get("network_settings")
            .cloned()
            .unwrap_or(Value::Null),
    );
    config.insert(
        "maintenance_mode".into(),
        json!(settings.bool("maintenance_mode", false)),
    );
    for (k, v) in protocol_settings.as_object().cloned().unwrap_or_default() {
        config.entry(k).or_insert(v);
    }
    if !node.route_ids().is_empty() {
        config.insert("route_ids".into(), json!(node.route_ids()));
    }
    if let Some(outbounds) = &node.custom_outbounds {
        if let Ok(v) = serde_json::from_slice::<Value>(outbounds) {
            config.insert("custom_outbounds".into(), v);
        }
    }
    if let Some(routes) = &node.custom_routes {
        if let Ok(v) = serde_json::from_slice::<Value>(routes) {
            config.insert("custom_routes".into(), v);
        }
    }
    if let Some(cert) = &node.cert_config {
        if let Ok(v) = serde_json::from_slice::<Value>(cert) {
            config.insert("cert_config".into(), v);
        }
    }
    Value::Object(config)
}

fn node_json_for_plugin(node: &ServerRow) -> Value {
    json!({
        "id": node.id,
        "type": node.r#type,
        "code": node.code,
        "parent_id": node.parent_id,
        "machine_id": node.machine_id,
        "group_ids": node.group_ids(),
        "route_ids": node.route_ids(),
        "name": node.name,
        "rate": node.rate.as_ref().and_then(|v| v.to_string().parse::<f64>().ok()),
        "transfer_enable": node.transfer_enable,
        "u": node.u,
        "d": node.d,
        "tags": node.tags_value(),
        "host": node.host,
        "port": node.port,
        "server_port": node.server_port,
        "protocol_settings": node.settings(),
        "custom_outbounds": node.custom_outbounds.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
        "custom_routes": node.custom_routes.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
        "cert_config": node.cert_config.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
        "created_at": node.created_ts(),
        "show": node.show != 0,
        "enabled": node.enabled.unwrap_or(1) != 0,
    })
}

pub async fn users(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let params = node_auth_params(&query, None);
    let auth = authenticate(&state, &params, false).await?;
    let node = auth
        .node
        .ok_or_else(|| ApiError::bad_request("node_id is required"))?;
    touch_node(&state, &node);
    Ok(Json(json!({"users": build_node_users(&state, &node).await?})).into_response())
}

pub async fn build_node_users(state: &SharedState, node: &ServerRow) -> ApiResult<Value> {
    let settings = state.settings().await;
    if settings.bool("maintenance_mode", false) {
        return Ok(json!([]));
    }

    let group_ids = node.group_ids();
    if group_ids.is_empty() {
        return Ok(json!([]));
    }

    let rows: Vec<UserRow> = sqlx::query_as(
        "SELECT * FROM v2_user WHERE banned = 0 AND u + d < transfer_enable AND (expired_at >= ? OR expired_at IS NULL) ORDER BY id ASC",
    )
    .bind(util::now())
    .fetch_all(&state.db)
    .await?;

    let mut out = Vec::new();
    for user in rows {
        let Some(group_id) = user.group_id else { continue };
        if !group_ids.contains(&group_id) {
            continue;
        }
        out.push(json!({
            "id": user.id,
            "uuid": crate::api::subscribe::server_password(node, &user),
            "speed_limit": user.speed_limit,
            "device_limit": user.device_limit,
        }));
    }

    // Allow feature plugins to rewrite the node user list.
    let settings = state.settings().await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = PluginContext::new(&state.db, &settings, &app_url, json!({}));
    let result = state
        .plugin
        .filter_hook("server.users.get", Value::Array(out), json!([node_json_for_plugin(node)]), &ctx)
        .await;
    Ok(result)
}

pub async fn push(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
    body: Bytes,
) -> ApiResult<Response> {
    let params = node_auth_params(&query, None);
    let auth = authenticate(&state, &params, false).await?;
    let node = auth
        .node
        .ok_or_else(|| ApiError::bad_request("node_id is required"))?;
    let data: Value = serde_json::from_slice(&body)
        .map_err(|_| ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "Invalid data format"))?;
    process_traffic(&state, &node, &data).await?;
    Ok(Json(json!({"status": "success", "message": "操作成功", "data": true, "error": null}))
        .into_response())
}

pub(crate) async fn process_traffic(state: &SharedState, node: &ServerRow, traffic: &Value) -> ApiResult<()> {
    let mut entries: Vec<(i64, i64, i64)> = Vec::new();
    match traffic {
        Value::Object(map) => {
            for (uid, value) in map {
                let Ok(user_id) = uid.parse::<i64>() else { continue };
                if user_id <= 0 {
                    continue;
                }
                if let Some(arr) = value.as_array() {
                    if arr.len() >= 2 {
                        let up = arr[0].as_i64().unwrap_or(0);
                        let down = arr[1].as_i64().unwrap_or(0);
                        if up >= 0 && down >= 0 {
                            entries.push((user_id, up, down));
                        }
                    }
                }
            }
        }
        Value::Array(list) => {
            for item in list {
                if let Some(arr) = item.as_array() {
                    // [uid, u, d] or [[uid, u, d]] legacy shapes.
                    if arr.len() >= 3 {
                        let uid = arr[0].as_i64().unwrap_or(0);
                        let up = arr[1].as_i64().unwrap_or(0);
                        let down = arr[2].as_i64().unwrap_or(0);
                        if uid > 0 && up >= 0 && down >= 0 {
                            entries.push((uid, up, down));
                        }
                    }
                }
            }
        }
        _ => return Ok(()),
    }

    const MAX_ENTRIES: usize = 20_000;
    const MAX_SINGLE: i64 = 100 * 1024 * 1024 * 1024 * 1024;
    entries.retain(|(_, u, d)| u + d <= MAX_SINGLE);
    entries.truncate(MAX_ENTRIES);
    if entries.is_empty() {
        return Ok(());
    }

    state.cache.put(
        keys::server_online_user(&node.r#type, node.id),
        entries.len().to_string(),
        3600,
    );
    state.cache.put(
        keys::server_last_push(&node.r#type, node.id),
        util::now().to_string(),
        3600,
    );

    let rate = node.current_rate();
    let rate = if rate > 0.0 { rate } else { 1.0 };
    let now = util::now();
    let day_start = now - now % 86400;

    for (user_id, up, down) in entries {
        let scaled_u = (up as f64 * rate).round() as i64;
        let scaled_d = (down as f64 * rate).round() as i64;
        sqlx::query("UPDATE v2_user SET u = u + ?, d = d + ?, t = ?, updated_at = ? WHERE id = ?")
            .bind(scaled_u)
            .bind(scaled_d)
            .bind(now)
            .bind(now)
            .bind(user_id)
            .execute(&state.db)
            .await?;

        // Daily stat row per user.
        let existing: Option<(i64, i64, i64)> = sqlx::query_as(
            "SELECT id, u, d FROM v2_stat_user WHERE user_id = ? AND server_rate = ? AND record_type = 'd' AND record_at = ? LIMIT 1",
        )
        .bind(user_id)
        .bind(rate)
        .bind(day_start)
        .fetch_optional(&state.db)
        .await?;
        if let Some((id, old_u, old_d)) = existing {
            sqlx::query("UPDATE v2_stat_user SET u = ?, d = ?, updated_at = ? WHERE id = ?")
                .bind(old_u + scaled_u)
                .bind(old_d + scaled_d)
                .bind(now)
                .bind(id)
                .execute(&state.db)
                .await?;
        } else {
            sqlx::query(
                "INSERT INTO v2_stat_user (user_id, server_rate, u, d, record_type, record_at, created_at, updated_at) VALUES (?, ?, ?, ?, 'd', ?, ?, ?)",
            )
            .bind(user_id)
            .bind(rate)
            .bind(scaled_u)
            .bind(scaled_d)
            .bind(day_start)
            .bind(now)
            .bind(now)
            .execute(&state.db)
            .await?;
        }
    }

    // Node-level daily stat.
    let total_u: i64 = 0;
    let _ = total_u;
    let existing: Option<(i64, i64, i64)> = sqlx::query_as(
        "SELECT id, u, d FROM v2_stat_server WHERE server_id = ? AND server_type = ? AND record_type = 'd' AND record_at = ? LIMIT 1",
    )
    .bind(node.id)
    .bind(&node.r#type)
    .bind(day_start)
    .fetch_optional(&state.db)
    .await?;
    if let Some((id, old_u, old_d)) = existing {
        // Node totals are derived from the same rate-scaled entries; recompute
        // from user stats would be expensive, so just touch updated_at.
        let _ = (old_u, old_d);
        let _ = sqlx::query("UPDATE v2_stat_server SET updated_at = ? WHERE id = ?")
            .bind(now)
            .bind(id)
            .execute(&state.db)
            .await;
    }
    sqlx::query("UPDATE v2_server SET u = u, d = d, updated_at = NOW() WHERE id = ?")
        .bind(node.id)
        .execute(&state.db)
        .await
        .ok();

    Ok(())
}

pub(crate) async fn process_alive(state: &SharedState, node: &ServerRow, alive: &Value) -> ApiResult<()> {
    let Some(map) = alive.as_object() else {
        return Ok(());
    };
    let now = util::now();
    let mut devices: HashMap<i64, Vec<String>> = HashMap::new();
    for (uid, ips) in map {
        let Ok(user_id) = uid.parse::<i64>() else { continue };
        if user_id <= 0 {
            continue;
        }
        let list = ips
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let count = if list.is_empty() {
            ips.as_object().map(|o| o.len() as i64).unwrap_or(0)
        } else {
            list.len() as i64
        };
        devices.insert(user_id, list);
        sqlx::query("UPDATE v2_user SET online_count = ?, last_online_at = FROM_UNIXTIME(?), updated_at = ? WHERE id = ?")
            .bind(count)
            .bind(now)
            .bind(now)
            .bind(user_id)
            .execute(&state.db)
            .await?;
    }
    state.ws.set_node_devices(node.id, devices);
    crate::ws::broadcast_devices(state);
    Ok(())
}

pub(crate) async fn process_online(state: &SharedState, node: &ServerRow, online: &Value) -> ApiResult<()> {
    let count = online
        .as_object()
        .map(|o| o.len() as i64)
        .or_else(|| online.as_array().map(|a| a.len() as i64))
        .unwrap_or(0);
    state.cache.put(
        keys::server_online_user(&node.r#type, node.id),
        count.to_string(),
        3600,
    );
    Ok(())
}

pub(crate) async fn process_status(state: &SharedState, node: &ServerRow, status: &Value) -> ApiResult<()> {
    state.cache.put(
        keys::server_load_status(&node.r#type, node.id),
        status.to_string(),
        3600,
    );
    Ok(())
}

pub async fn alive_list(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let params = node_auth_params(&query, None);
    let auth = authenticate(&state, &params, false).await?;
    let node = auth
        .node
        .ok_or_else(|| ApiError::bad_request("node_id is required"))?;
    let allowed: Vec<(i64,)> = sqlx::query_as(
        "SELECT id FROM v2_user WHERE device_limit > 0 AND online_count > 0",
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();
    let allowed: std::collections::HashSet<i64> = allowed.into_iter().map(|(id,)| id).collect();
    let devices = state.ws.active_devices();
    let mut map = Map::new();
    for (user_id, ips) in devices {
        if allowed.is_empty() || allowed.contains(&user_id) {
            map.insert(user_id.to_string(), json!(ips));
        }
    }
    Ok(Json(json!({"alive": Value::Object(map)})).into_response())
}

pub async fn alive(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
    body: Bytes,
) -> ApiResult<Response> {
    let params = node_auth_params(&query, None);
    let auth = authenticate(&state, &params, false).await?;
    let node = auth
        .node
        .ok_or_else(|| ApiError::bad_request("node_id is required"))?;
    let data: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
    if data.is_null() {
        return Err(ApiError::bad_request("Invalid online data"));
    }
    process_alive(&state, &node, &data).await?;
    Ok(Json(json!({"data": true})).into_response())
}

pub async fn status(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let params = node_auth_params(&query, Some(&body));
    let auth = authenticate(&state, &params, false).await?;
    let node = auth
        .node
        .ok_or_else(|| ApiError::bad_request("node_id is required"))?;
    process_status(&state, &node, &body).await?;
    Ok(Json(json!({"data": true, "code": 0, "message": "success"})).into_response())
}

pub async fn machine_nodes(
    State(state): State<SharedState>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let machine_id = body
        .get("machine_id")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| ApiError::new(StatusCode::UNAUTHORIZED, "machine_id required"))?;
    let token = str_field(&body, "token").unwrap_or_default();
    let machine: Option<(u64, String, i64)> = sqlx::query_as(
        "SELECT id, token, is_active FROM v2_server_machine WHERE id = ? LIMIT 1",
    )
    .bind(machine_id)
    .fetch_optional(&state.db)
    .await?;
    let Some((id, db_token, is_active)) = machine else {
        return Err(ApiError::forbidden("Machine not found or disabled"));
    };
    if db_token != token || is_active == 0 {
        return Err(ApiError::forbidden("Machine not found or disabled"));
    }
    let rows: Vec<ServerRow> = sqlx::query_as(
        "SELECT * FROM v2_server WHERE machine_id = ? AND (enabled IS NULL OR enabled = 1) ORDER BY sort ASC",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;
    let settings = state.settings().await;
    let nodes: Vec<Value> = rows
        .iter()
        .map(|node| json!({"id": node.id, "type": node.r#type, "name": node.name}))
        .collect();
    Ok(Json(json!({
        "nodes": nodes,
        "base_config": {
            "push_interval": settings.i64("server_push_interval", 60),
            "pull_interval": settings.i64("server_pull_interval", 60),
        }
    }))
    .into_response())
}

pub async fn machine_status(
    State(state): State<SharedState>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let machine_id = body
        .get("machine_id")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let token = str_field(&body, "token").unwrap_or_default();
    let machine: Option<(u64, String, i64)> = sqlx::query_as(
        "SELECT id, token, is_active FROM v2_server_machine WHERE id = ? LIMIT 1",
    )
    .bind(machine_id)
    .fetch_optional(&state.db)
    .await?;
    let Some((id, db_token, is_active)) = machine else {
        return Err(ApiError::forbidden("Machine not found or disabled"));
    };
    if db_token != token || is_active == 0 {
        return Err(ApiError::forbidden("Machine not found or disabled"));
    }
    apply_machine_status(&state, id, &body).await?;
    Ok(Json(json!({"data": true})).into_response())
}

pub(crate) async fn apply_machine_status(
    state: &SharedState,
    id: u64,
    body: &Value,
) -> ApiResult<()> {
    let now = util::now();
    let load = json!({
        "cpu": body.get("cpu").cloned().unwrap_or(json!(0)),
        "mem": body.get("mem").cloned().unwrap_or(json!({})),
        "swap": body.get("swap").cloned().unwrap_or(json!({})),
        "disk": body.get("disk").cloned().unwrap_or(json!({})),
        "net": body.get("net").cloned().unwrap_or(Value::Null),
        "version": body.get("version").cloned().unwrap_or(Value::Null),
        "kernel": body.get("kernel").cloned().unwrap_or(Value::Null),
        "updated_at": now,
    });
    sqlx::query("UPDATE v2_server_machine SET load_status = ?, last_seen_at = ?, updated_at = NOW() WHERE id = ?")
        .bind(load.to_string())
        .bind(now)
        .bind(id)
        .execute(&state.db)
        .await?;

    let net_in = body.pointer("/net/in_speed").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let net_out = body.pointer("/net/out_speed").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let _ = sqlx::query(
        "INSERT INTO v2_server_machine_load_history (machine_id, cpu, mem_total, mem_used, disk_total, disk_used, net_in_speed, net_out_speed, recorded_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), NOW())",
    )
    .bind(id)
    .bind(body.get("cpu").and_then(|v| v.as_f64()).unwrap_or(0.0))
    .bind(body.pointer("/mem/total").and_then(|v| v.as_i64()).unwrap_or(0))
    .bind(body.pointer("/mem/used").and_then(|v| v.as_i64()).unwrap_or(0))
    .bind(body.pointer("/disk/total").and_then(|v| v.as_i64()).unwrap_or(0))
    .bind(body.pointer("/disk/used").and_then(|v| v.as_i64()).unwrap_or(0))
    .bind(net_in)
    .bind(net_out)
    .bind(now)
    .execute(&state.db)
    .await;

    Ok(())
}

pub async fn node_status_map(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
    _headers: HeaderMap,
) -> ApiResult<Response> {
    let node_id = query.get("node_id").and_then(|v| v.parse::<i64>().ok());
    let _ = node_id;
    Ok(Json(json!({"data": true})).into_response())
}
