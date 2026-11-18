use base64::Engine;
use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Map, Value};

use crate::error::{ApiError, ApiResult};
use crate::models::{ServerRow, UserRow};
use crate::plugin::PluginContext;
use crate::response::ok;
use crate::state::{require_client_user, SharedState};
use crate::util;

const VALID_TYPES: &[&str] = &[
    "shadowsocks",
    "vmess",
    "vless",
    "trojan",
    "hysteria",
    "tuic",
    "anytls",
    "socks",
    "naive",
    "http",
    "mieru",
    "sudoku",
    "shadowquic",
    "trusttunnel",
    "masque",
];

pub async fn build_servers(state: &SharedState, user: &UserRow) -> ApiResult<Vec<Value>> {
    let rows: Vec<ServerRow> = sqlx::query_as(
        "SELECT * FROM v2_server WHERE `show` = 1 AND (enabled IS NULL OR enabled = 1) ORDER BY sort ASC, id ASC",
    )
    .fetch_all(&state.db)
    .await?;

    let mut by_id: HashMap<u64, ServerRow> = rows.iter().cloned().map(|s| (s.id, s)).collect();
    let now = util::now();
    let mut out = Vec::new();

    for row in rows {
        if let Some(group_id) = user.group_id {
            if !row.group_ids().iter().any(|g| *g == group_id) {
                continue;
            }
        } else if !row.group_ids().is_empty() {
            continue;
        }

        if let Some(limit) = row.transfer_enable {
            if limit > 0 && row.u + row.d >= limit {
                continue;
            }
        }

        // Virtual node inherits its parent's connectivity settings.
        let effective = if row.r#type == "virtual" {
            match row.parent_id.and_then(|pid| by_id.get(&pid).cloned()) {
                Some(parent) => {
                    let mut merged = parent.clone();
                    merged.id = row.id;
                    merged.name = row.name.clone();
                    merged.show = row.show;
                    merged.sort = row.sort;
                    merged.parent_id = row.parent_id;
                    merged.group_ids = row.group_ids.clone();
                    merged.r#type = parent.r#type.clone();
                    merged
                }
                None => continue,
            }
        } else {
            row.clone()
        };

        let mut port = effective.port.clone();
        let mut ports: Option<String> = None;
        if port.contains('-') {
            let parts: Vec<i64> = port
                .split('-')
                .filter_map(|p| p.parse().ok())
                .collect();
            if parts.len() == 2 && parts[1] >= parts[0] {
                let chosen =
                    parts[0] + (rand::random::<u64>() % ((parts[1] - parts[0] + 1) as u64)) as i64;
                ports = Some(port.clone());
                port = chosen.to_string();
            }
        }

        let password = server_password(&effective, user);
        let mut settings = effective.settings();
        normalize_reality(&mut settings);

        let mut item = Map::new();
        item.insert("id".into(), json!(effective.id));
        item.insert("type".into(), json!(effective.r#type));
        item.insert("name".into(), json!(effective.name));
        item.insert("host".into(), json!(effective.host));
        item.insert("port".into(), json!(port));
        if let Some(range) = ports {
            item.insert("ports".into(), json!(range));
        }
        item.insert("server_port".into(), json!(effective.server_port));
        item.insert("password".into(), json!(password));
        item.insert("uuid".into(), json!(user.uuid));
        item.insert("rate".into(), json!(effective.current_rate()));
        item.insert("group_ids".into(), json!(effective.group_ids()));
        item.insert("route_ids".into(), json!(effective.route_ids()));
        item.insert("tags".into(), effective.tags_value());
        item.insert("protocol_settings".into(), settings);
        item.insert("server_key".into(), json!(server_key(&effective)));
        item.insert("parent_id".into(), json!(effective.parent_id));
        item.insert("created_at".into(), json!(effective.created_ts()));
        item.insert("u".into(), json!(effective.u));
        item.insert("d".into(), json!(effective.d));
        item.insert("transfer_enable".into(), json!(effective.transfer_enable));
        item.insert("enabled".into(), json!(effective.enabled.unwrap_or(1) != 0));
        item.insert("show".into(), json!(effective.show != 0));
        item.insert("is_online".into(), json!(1));
        item.insert(
            "available_status".into(),
            json!(2),
        );
        item.insert("last_check_at".into(), json!(now));
        item.insert("last_push_at".into(), json!(now));
        item.insert("online".into(), json!(0));
        out.push(Value::Object(item));
    }

    Ok(out)
}

fn normalize_reality(settings: &mut Value) {
    let Some(obj) = settings.as_object_mut() else {
        return;
    };
    let Some(reality) = obj.get_mut("reality_settings").and_then(|v| v.as_object_mut()) else {
        return;
    };
    for field in ["public_key", "private_key"] {
        if let Some(Value::String(key)) = reality.get_mut(field) {
            *key = normalize_reality_key(key);
        }
    }
}

fn normalize_reality_key(key: &str) -> String {
    let trimmed = key.trim();
    match base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(trimmed) {
        Ok(bytes) if bytes.len() == 32 => trimmed.to_string(),
        _ => match base64::engine::general_purpose::STANDARD.decode(trimmed) {
            Ok(bytes) if bytes.len() == 32 => {
                base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
            }
            _ => trimmed.to_string(),
        },
    }
}

pub fn server_password(server: &ServerRow, user: &UserRow) -> String {
    let settings = server.settings();
    match server.r#type.as_str() {
        "shadowsocks" => {
            let cipher = settings.get("cipher").and_then(|v| v.as_str()).unwrap_or("");
            let size = match cipher {
                "2022-blake3-aes-128-gcm" => Some(16),
                "2022-blake3-aes-256-gcm" | "2022-blake3-chacha20-poly1305" => Some(32),
                _ => None,
            };
            match size {
                Some(size) => {
                    let server_key = server_key_with_len(server.created_ts(), size);
                    let user_key = base64::engine::general_purpose::STANDARD
                        .encode(&user.uuid.as_bytes()[..user.uuid.len().min(size)]);
                    format!("{server_key}:{user_key}")
                }
                None => user.uuid.clone(),
            }
        }
        "masque" => user
            .usque_device_public
            .clone()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| user.uuid.clone()),
        _ => user.uuid.clone(),
    }
}

fn server_key(server: &ServerRow) -> String {
    server_key_with_len(server.created_ts(), 16)
}

fn server_key_with_len(timestamp: i64, len: usize) -> String {
    let digest = util::md5_hex(&timestamp.to_string());
    base64::engine::general_purpose::STANDARD.encode(&digest.as_bytes()[..len.min(digest.len())])
}

fn normalize_server_for_plugin(mut value: Value) -> Value {
    if let Some(obj) = value.as_object_mut() {
        // Legacy output plugins expect `port` as string or int; both serialize
        // identically in URI building.
        obj.entry("port").or_insert(json!("0"));
    }
    value
}

fn parse_requested_types(types: Option<&str>) -> Vec<String> {
    match types {
        None | Some("") | Some("all") => VALID_TYPES.iter().map(|s| s.to_string()).collect(),
        Some(raw) => raw
            .split(['|', ',', '｜'])
            .map(|s| s.trim().to_lowercase())
            .filter(|s| VALID_TYPES.contains(&s.as_str()))
            .collect(),
    }
}

fn filter_servers(
    servers: Vec<Value>,
    allowed: &[String],
    keywords: &[String],
) -> Vec<Value> {
    servers
        .into_iter()
        .filter(|server| {
            let ty = server.get("type").and_then(|v| v.as_str()).unwrap_or("");
            if !allowed.is_empty() && !allowed.iter().any(|t| t == ty) {
                return false;
            }
            if keywords.is_empty() {
                return true;
            }
            let name = server
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_lowercase();
            let tags: Vec<String> = server
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|t| t.as_str().map(|s| s.to_lowercase()))
                        .collect()
                })
                .unwrap_or_default();
            keywords.iter().any(|kw| {
                name.contains(kw) || tags.iter().any(|tag| tag.contains(kw))
            })
        })
        .collect()
}

pub async fn client_subscribe(
    State(state): State<SharedState>,
    Path(token): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let user = require_client_user(&state, Some(&token)).await?;
    subscribe_response(&state, &user, &query, &headers).await
}

pub async fn client_subscribe_query(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let user = require_client_user(&state, query.get("token").map(|s| s.as_str())).await?;
    subscribe_response(&state, &user, &query, &headers).await
}

async fn subscribe_response(
    state: &SharedState,
    user: &UserRow,
    query: &HashMap<String, String>,
    headers: &HeaderMap,
) -> ApiResult<Response> {
    if !user.is_available() {
        return Ok((StatusCode::FORBIDDEN, "").into_response());
    }

    let mut servers = build_servers(state, user).await?;
    let allowed = parse_requested_types(query.get("types").map(|s| s.as_str()));
    let keywords: Vec<String> = query
        .get("filter")
        .filter(|f| f.chars().count() <= 20)
        .map(|f| {
            f.split(['|', ',', '｜'])
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();
    let total = servers.len();
    servers = filter_servers(servers, &allowed, &keywords);
    let rejected = total.saturating_sub(servers.len());

    let settings = state.settings().await;
    if settings.bool("show_info_to_server_enable", false) && !servers.is_empty() {
        let used = user.u + user.d;
        let remaining = util::traffic_convert((user.transfer_enable - used).max(0) as f64);
        let expired = user
            .expired_at
            .map(|ts| {
                chrono::DateTime::from_timestamp(ts, 0)
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "长期有效".into())
            })
            .unwrap_or_else(|| "长期有效".into());
        let base = servers[0].clone();
        for name in [
            format!("剩余流量：{remaining}"),
            format!("套餐到期：{expired}"),
        ] {
            let mut info = base.clone();
            if let Some(obj) = info.as_object_mut() {
                obj.insert("name".into(), json!(name));
            }
            servers.insert(0, info);
        }
    }
    if rejected > 0 && !servers.is_empty() {
        let mut info = servers[0].clone();
        if let Some(obj) = info.as_object_mut() {
            obj.insert("name".into(), json!(format!("过滤掉{rejected}条线路")));
        }
        servers.insert(0, info);
    }

    let flag = query
        .get("flag")
        .cloned()
        .or_else(|| {
            headers
                .get(axum::http::header::USER_AGENT)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        })
        .unwrap_or_default()
        .to_lowercase();

    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = PluginContext::new(&state.db, &settings, &app_url, json!({}));
    let enabled = state.plugin.enabled_map(&state.db).await;
    let plugin_result = state
        .plugin
        .render_subscription(
            json!({
                "user": user.public_json(),
                "servers": servers.iter().cloned().map(normalize_server_for_plugin).collect::<Vec<_>>(),
                "flag": flag,
                "host": headers
                    .get(axum::http::header::HOST)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or(""),
                "client": client_info(&flag),
                "templates": load_templates(state).await,
                "settings": settings.public_map(),
            }),
            &ctx,
            &enabled,
        )
        .await;

    if let Some(result) = plugin_result {
        if let Some(body) = result.get("body").and_then(|v| v.as_str()) {
            let content_type = result
                .get("content_type")
                .and_then(|v| v.as_str())
                .unwrap_or("text/plain; charset=utf-8");
            let mut response = (StatusCode::OK, body.to_string()).into_response();
            response.headers_mut().insert(
                axum::http::header::CONTENT_TYPE,
                content_type.parse().unwrap(),
            );
            attach_subscription_headers(&mut response, user, state);
            return Ok(response);
        }
    }

    // Built-in fallback: general (base64) URI list.
    let links = servers
        .iter()
        .map(build_link)
        .collect::<Vec<_>>()
        .join("");
    let body = base64::engine::general_purpose::STANDARD.encode(links.as_bytes());
    let mut response = (StatusCode::OK, body).into_response();
    response.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse().unwrap(),
    );
    attach_subscription_headers(&mut response, user, state);
    Ok(response)
}

fn attach_subscription_headers(response: &mut Response, user: &UserRow, state: &SharedState) {
    let expire = user.expired_at.unwrap_or(0).max(0);
    let info = format!(
        "upload={}; download={}; total={}; expire={}",
        user.u, user.d, user.transfer_enable, expire
    );
    if let Ok(v) = info.parse() {
        response.headers_mut().insert("subscription-userinfo", v);
    }
    if let Ok(v) = "24".parse() {
        response.headers_mut().insert("profile-update-interval", v);
    }
    if let Ok(v) = state.cfg.app_url.parse() {
        response.headers_mut().insert("profile-web-page-url", v);
    }
}

async fn load_templates(state: &SharedState) -> Value {
    let rows: Vec<(String, Option<String>)> =
        sqlx::query_as("SELECT name, content FROM v2_subscribe_templates")
            .fetch_all(&state.db)
            .await
            .unwrap_or_default();
    let mut map = Map::new();
    for (name, content) in rows {
        if let Some(content) = content {
            if !content.is_empty() {
                map.insert(name, json!(content));
            }
        }
    }
    // Match the Laravel migration: bundled defaults are used until an admin
    // saves a custom template.
    let defaults = [
        ("clash", include_str!("../../../resources/rules/default.clash.yaml")),
        ("clashmeta", include_str!("../../../resources/rules/default.clash.yaml")),
        ("stash", include_str!("../../../resources/rules/default.clash.yaml")),
        ("singbox", include_str!("../../../resources/rules/default.sing-box.json")),
        ("surge", include_str!("../../../resources/rules/default.surge.conf")),
        ("surfboard", include_str!("../../../resources/rules/default.surfboard.conf")),
    ];
    for (name, content) in defaults {
        map.entry(name.to_string()).or_insert_with(|| json!(content));
    }
    Value::Object(map)
}

fn client_info(flag: &str) -> Value {
    let known = [
        "clashmeta", "clash", "singbox", "sing-box", "loon", "quantumultx", "shadowrocket",
        "stash", "surfboard", "surge", "v2rayn", "v2rayng", "passwall", "ssrplus", "sagernet",
        "inhive", "general", "shadowsocks",
    ];
    let mut name = "";
    for candidate in known.iter().rev() {
        if flag.contains(candidate) {
            name = candidate;
            break;
        }
    }
    json!({
        "name": if name.is_empty() { Value::Null } else { Value::String(name.into()) },
        "version": Value::Null,
        "flag": flag,
    })
}

fn bool_field_simple(value: &Value, path: &str) -> bool {
    value
        .pointer(path)
        .map(|v| match v {
            Value::Bool(b) => *b,
            Value::Number(n) => n.as_i64().unwrap_or(0) != 0,
            Value::String(s) => matches!(s.as_str(), "1" | "true" | "yes"),
            _ => false,
        })
        .unwrap_or(false)
}

pub(crate) fn build_link(server: &Value) -> String {
    let ty = server.get("type").and_then(|v| v.as_str()).unwrap_or("");
    let name = server.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let host = server.get("host").and_then(|v| v.as_str()).unwrap_or("");
    let port = server
        .get("port")
        .map(|v| v.as_str().map(|s| s.to_string()).unwrap_or_else(|| v.to_string()))
        .unwrap_or_else(|| "0".into());
    let password = server
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let settings = server
        .get("protocol_settings")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let addr = util::wrap_ipv6(host);
    let encoded_name = urlencoding::encode(name);

    match ty {
        "shadowsocks" => {
            let cipher = settings.get("cipher").and_then(|v| v.as_str()).unwrap_or("aes-256-gcm");
            let userinfo = base64::engine::general_purpose::URL_SAFE_NO_PAD
                .encode(format!("{cipher}:{password}").as_bytes());
            let mut link = format!("ss://{userinfo}@{addr}:{port}");
            if let (Some(plugin), Some(opts)) = (
                settings.get("plugin").and_then(|v| v.as_str()),
                settings.get("plugin_opts").and_then(|v| v.as_str()),
            ) {
                if !plugin.is_empty() {
                    link.push_str(&format!(
                        "/?plugin={}",
                        urlencoding::encode(&format!("{plugin};{opts}"))
                    ));
                }
            }
            format!("{link}#{encoded_name}\r\n")
        }
        "vmess" => {
            let network = settings
                .get("network")
                .and_then(|v| v.as_str())
                .unwrap_or("tcp");
            let tls = settings.get("tls").and_then(|v| v.as_i64()).unwrap_or(0) != 0;
            let ws_host = settings
                .pointer("/network_settings/headers/Host")
                .and_then(|v| v.as_str())
                .or_else(|| {
                    settings
                        .pointer("/network_settings/headers/host")
                        .and_then(|v| v.as_str())
                })
                .unwrap_or("");
            let ws_path = settings
                .pointer("/network_settings/path")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let sni = settings
                .pointer("/tls_settings/server_name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let host_header = if ws_host.is_empty() { sni } else { ws_host };
            let header_type = settings
                .pointer("/network_settings/header/type")
                .and_then(|v| v.as_str())
                .unwrap_or("none");
            let mut config = json!({
                "v": "2",
                "ps": name,
                "add": host,
                "port": port,
                "id": password,
                "aid": "0",
                "net": if network == "httpupgrade" { "ws" } else { network },
                "type": header_type,
                "host": host_header,
                "path": ws_path,
                "tls": if tls { "tls" } else { "" },
            });
            if tls {
                config["sni"] = json!(sni);
                if let Some(alpn) = settings
                    .pointer("/tls_settings/client_alpn")
                    .or_else(|| settings.pointer("/tls_settings/alpn"))
                {
                    config["alpn"] = alpn.clone();
                }
            }
            if network == "grpc" {
                config["path"] = settings
                    .pointer("/network_settings/serviceName")
                    .cloned()
                    .unwrap_or_else(|| json!(""));
            }
            format!(
                "vmess://{}\r\n",
                base64::engine::general_purpose::STANDARD.encode(config.to_string().as_bytes())
            )
        }
        "vless" => {
            let mut params: Vec<(String, String)> = vec![
                ("encryption".into(), "none".into()),
                (
                    "type".into(),
                    settings
                        .get("network")
                        .and_then(|v| v.as_str())
                        .unwrap_or("tcp")
                        .into(),
                ),
            ];
            let tls = settings.get("tls").and_then(|v| v.as_i64()).unwrap_or(0);
            let mut security = "none";
            if tls == 1 {
                security = "tls";
                if let Some(sni) = settings
                    .pointer("/tls_settings/server_name")
                    .and_then(|v| v.as_str())
                {
                    params.push(("sni".into(), sni.into()));
                }
            } else if tls == 2 {
                security = "reality";
                if let Some(pbk) = settings
                    .pointer("/reality_settings/public_key")
                    .and_then(|v| v.as_str())
                {
                    params.push(("pbk".into(), pbk.into()));
                }
                if let Some(sid) = settings
                    .pointer("/reality_settings/short_id")
                    .and_then(|v| v.as_str())
                {
                    params.push(("sid".into(), sid.into()));
                }
                if let Some(sni) = settings
                    .pointer("/reality_settings/server_name")
                    .and_then(|v| v.as_str())
                {
                    params.push(("sni".into(), sni.into()));
                }
                params.push(("fp".into(), "chrome".into()));
            }
            if let Some(flow) = settings.get("flow").and_then(|v| v.as_str()) {
                if !flow.is_empty() {
                    params.push(("flow".into(), flow.into()));
                }
            }
            let network = settings
                .get("network")
                .and_then(|v| v.as_str())
                .unwrap_or("tcp");
            if let Some(path) = settings
                .pointer("/network_settings/path")
                .and_then(|v| v.as_str())
            {
                params.push(("path".into(), path.into()));
            }
            if let Some(host) = settings
                .pointer("/network_settings/headers/Host")
                .and_then(|v| v.as_str())
                .or_else(|| {
                    settings
                        .pointer("/network_settings/headers/host")
                        .and_then(|v| v.as_str())
                })
            {
                params.push(("host".into(), host.into()));
            }
            if network == "grpc" {
                if let Some(service) = settings
                    .pointer("/network_settings/serviceName")
                    .and_then(|v| v.as_str())
                {
                    params.push(("serviceName".into(), service.into()));
                }
            }
            if let Some(alpn) = settings
                .pointer("/tls_settings/client_alpn")
                .or_else(|| settings.pointer("/tls_settings/alpn"))
            {
                let value = alpn
                    .as_str()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| alpn.to_string());
                params.push(("alpn".into(), value));
            }
            params.push(("security".into(), security.into()));
            let query = params
                .iter()
                .map(|(k, v)| format!("{k}={}", urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!("vless://{password}@{addr}:{port}?{query}#{encoded_name}\r\n")
        }
        "trojan" => {
            let mut params: Vec<(String, String)> = Vec::new();
            if let Some(sni) = settings
                .pointer("/tls_settings/server_name")
                .and_then(|v| v.as_str())
            {
                params.push(("sni".into(), sni.into()));
            }
            if let Some(network) = settings.get("network").and_then(|v| v.as_str()) {
                if !network.is_empty() && network != "tcp" {
                    params.push(("type".into(), network.into()));
                }
            }
            if let Some(path) = settings
                .pointer("/network_settings/path")
                .and_then(|v| v.as_str())
            {
                params.push(("path".into(), path.into()));
            }
            if let Some(host) = settings
                .pointer("/network_settings/headers/Host")
                .and_then(|v| v.as_str())
                .or_else(|| {
                    settings
                        .pointer("/network_settings/headers/host")
                        .and_then(|v| v.as_str())
                })
            {
                params.push(("host".into(), host.into()));
            }
            if bool_field_simple(&settings, "/tls_settings/allow_insecure") {
                params.push(("allowInsecure".into(), "1".into()));
            }
            let query = params
                .iter()
                .map(|(k, v)| format!("{k}={}", urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!("trojan://{password}@{addr}:{port}?{query}#{encoded_name}\r\n")
        }
        "hysteria" => {
            let version = settings.get("version").and_then(|v| v.as_i64()).unwrap_or(2);
            let mut params: Vec<(String, String)> = Vec::new();
            if let Some(sni) = settings
                .pointer("/tls/server_name")
                .and_then(|v| v.as_str())
            {
                params.push(("sni".into(), sni.into()));
            }
            if version == 2 {
                if let Some(range) = server.get("ports").and_then(|v| v.as_str()) {
                    params.push(("mport".into(), range.into()));
                }
                let query = params
                    .iter()
                    .map(|(k, v)| format!("{k}={}", urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("&");
                format!("hysteria2://{password}@{addr}:{port}?{query}#{encoded_name}\r\n")
            } else {
                params.push(("protocol".into(), "udp".into()));
                params.push(("auth".into(), password.clone()));
                let query = params
                    .iter()
                    .map(|(k, v)| format!("{k}={}", urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("&");
                format!("hysteria://{addr}:{port}?{query}#{encoded_name}\r\n")
            }
        }
        "tuic" => {
            let mut params: Vec<(String, String)> = Vec::new();
            if let Some(sni) = settings
                .pointer("/tls/server_name")
                .and_then(|v| v.as_str())
            {
                params.push(("sni".into(), sni.into()));
            }
            let query = params
                .iter()
                .map(|(k, v)| format!("{k}={}", urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!("tuic://{password}:{password}@{addr}:{port}?{query}#{encoded_name}\r\n")
        }
        "anytls" => {
            let mut params: Vec<(String, String)> = Vec::new();
            if let Some(sni) = settings
                .pointer("/tls/server_name")
                .and_then(|v| v.as_str())
            {
                params.push(("sni".into(), sni.into()));
            }
            let query = params
                .iter()
                .map(|(k, v)| format!("{k}={}", urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!(
                "anytls://{}@{addr}:{port}/?{query}#{encoded_name}\r\n",
                urlencoding::encode(&password)
            )
        }
        "socks" => {
            let creds = base64::engine::general_purpose::STANDARD
                .encode(format!("{password}:{password}").as_bytes());
            format!("socks://{creds}@{addr}:{port}#{encoded_name}\r\n")
        }
        "http" => {
            let creds = base64::engine::general_purpose::STANDARD
                .encode(format!("{password}:{password}").as_bytes());
            format!("http://{creds}@{addr}:{port}#{encoded_name}\r\n")
        }
        _ => String::new(),
    }
}

pub async fn app_config(State(state): State<SharedState>) -> ApiResult<Response> {
    let s = state.settings().await;
    Ok(ok(json!({
        "app_name": s.str("app_name", "Fboard"),
        "app_url": s.str("app_url", ""),
        "subscribe_path": s.subscribe_path(),
        "logo": s.str("logo", ""),
        "version": env!("CARGO_PKG_VERSION"),
    })))
}

pub async fn app_version() -> ApiResult<Response> {
    Ok(Json(json!({
        "status": "success",
        "message": "操作成功",
        "data": {
            "version": env!("CARGO_PKG_VERSION"),
            "channel": "rust"
        },
        "error": null
    }))
    .into_response())
        .map_err(|e: ApiError| e)
}

pub async fn subscribe_url(state: &SharedState, user: &UserRow) -> String {
    let settings = state.settings().await;
    let base = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    format!(
        "{}/{}/{}",
        base.trim_end_matches('/'),
        settings.subscribe_path(),
        user.token
    )
}
