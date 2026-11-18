use async_trait::async_trait;
use base64::Engine;
use serde_json::{json, Map, Value};

use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};

pub struct SurgePlugin;
pub struct SurfboardPlugin;
pub struct LoonPlugin;
pub struct QuantumultXPlugin;
pub struct StashPlugin;
pub struct ShadowrocketPlugin;

fn servers(params: &Value) -> Vec<Value> {
    params
        .get("servers")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
}

fn string_field(value: &Value, key: &str) -> String {
    match value.get(key) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => (if *b { "1" } else { "0" }).into(),
        _ => String::new(),
    }
}

fn bool_field(value: &Value, key: &str) -> bool {
    match value.get(key) {
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
        Some(Value::String(s)) => matches!(s.as_str(), "1" | "true" | "yes"),
        _ => false,
    }
}

fn settings(server: &Value) -> Value {
    server
        .get("protocol_settings")
        .cloned()
        .unwrap_or_else(|| json!({}))
}

fn server_port(server: &Value) -> String {
    match server.get("port") {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        _ => "0".into(),
    }
}

fn app_name(params: &Value) -> String {
    params
        .get("settings")
        .and_then(|v| v.get("app_name"))
        .and_then(|v| v.as_str())
        .unwrap_or("Fboard")
        .to_string()
}

fn subscribe_link(params: &Value) -> String {
    let settings = params.get("settings").cloned().unwrap_or_else(|| json!({}));
    let base = settings
        .get("app_url")
        .and_then(|v| v.as_str())
        .filter(|v| !v.is_empty())
        .unwrap_or("")
        .trim_end_matches('/')
        .to_string();
    let path = settings
        .get("subscribe_path")
        .and_then(|v| v.as_str())
        .unwrap_or("s");
    let token = params
        .get("user")
        .and_then(|v| v.get("token"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    format!("{base}/{path}/{token}")
}

fn subscribe_info(params: &Value, app: &str) -> String {
    let user = params.get("user").cloned().unwrap_or_else(|| json!({}));
    let u = user.get("u").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let d = user.get("d").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let total = user
        .get("transfer_enable")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let gb = |v: f64| -> f64 { (v / 1024.0 / 1024.0 / 1024.0 * 100.0).round() / 100.0 };
    let upload = gb(u);
    let download = gb(d);
    let total_gb = gb(total);
    let unused = (total_gb - upload - download).max(0.0);
    let expire = user
        .get("expired_at")
        .and_then(|v| v.as_i64())
        .and_then(|v| chrono::DateTime::from_timestamp(v, 0))
        .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "长期有效".into());
    format!(
        "title={app}订阅信息, content=上传流量：{upload}GB\\n下载流量：{download}GB\\n剩余流量：{unused}GB\\n套餐流量：{total_gb}GB\\n到期时间：{expire}"
    )
}

fn tls_settings(settings: &Value) -> Value {
    settings
        .get("tls_settings")
        .cloned()
        .unwrap_or_else(|| json!({}))
}

fn server_name(settings: &Value) -> Option<String> {
    settings
        .pointer("/tls_settings/server_name")
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
        .or_else(|| {
            settings
                .pointer("/tls/server_name")
                .and_then(|v| v.as_str())
                .map(|v| v.to_string())
        })
}

fn ws_host(settings: &Value) -> Option<String> {
    settings
        .pointer("/network_settings/headers/Host")
        .and_then(|v| v.as_str())
        .or_else(|| {
            settings
                .pointer("/network_settings/headers/host")
                .and_then(|v| v.as_str())
        })
        .map(|v| v.to_string())
}

fn ws_path(settings: &Value) -> Option<String> {
    settings
        .pointer("/network_settings/path")
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
}

fn grpc_service(settings: &Value) -> Option<String> {
    settings
        .pointer("/network_settings/serviceName")
        .and_then(|v| v.as_str())
        .or_else(|| {
            settings
                .pointer("/network_settings/service_name")
                .and_then(|v| v.as_str())
        })
        .map(|v| v.to_string())
}

fn comma(parts: Vec<String>) -> String {
    let filtered: Vec<String> = parts.into_iter().filter(|v| !v.is_empty()).collect();
    format!("{}\r\n", filtered.join(","))
}

fn surge_proxy(server: &Value, spaced: bool) -> Option<String> {
    let ty = string_field(server, "type");
    let name = string_field(server, "name");
    let host = string_field(server, "host");
    let port = server_port(server);
    let password = string_field(server, "password");
    let uuid = string_field(server, "uuid");
    let settings = settings(server);
    let sep = if spaced { " = " } else { "=" };

    match ty.as_str() {
        "shadowsocks" => {
            let mut parts = vec![
                format!("{name}{sep}ss"),
                host,
                port,
                format!(
                    "encrypt-method={}",
                    string_field(&settings, "cipher")
                ),
                format!("password={password}"),
                "tfo=true".into(),
                "udp-relay=true".into(),
            ];
            if let Some(plugin) = settings.get("plugin").and_then(|v| v.as_str()) {
                if plugin == "obfs" {
                    let opts = settings.get("plugin_opts").and_then(|v| v.as_str()).unwrap_or("");
                    for pair in opts.split(';').filter(|v| !v.is_empty()) {
                        if let Some((key, value)) = pair.split_once('=') {
                            match key.trim() {
                                "obfs" => parts.push(format!("obfs={value}")),
                                "obfs-host" => parts.push(format!("obfs-host={value}")),
                                "path" => parts.push(format!("obfs-uri={value}")),
                                _ => {}
                            }
                        }
                    }
                }
            }
            Some(comma(parts))
        }
        "vmess" => {
            let mut parts = vec![
                format!("{name}{sep}vmess"),
                host,
                port,
                format!("username={uuid}"),
                "vmess-aead=true".into(),
                "tfo=true".into(),
                "udp-relay=true".into(),
            ];
            if bool_field(&settings, "tls") {
                parts.push("tls=true".into());
                let tls = tls_settings(&settings);
                parts.push(format!(
                    "skip-cert-verify={}",
                    bool_field(&tls, "allow_insecure")
                ));
                if let Some(sni) = server_name(&settings) {
                    parts.push(format!("sni={sni}"));
                }
            }
            match settings.get("network").and_then(|v| v.as_str()).unwrap_or("tcp") {
                "ws" | "httpupgrade" => {
                    parts.push("ws=true".into());
                    if let Some(path) = ws_path(&settings) {
                        parts.push(format!("ws-path={path}"));
                    }
                    if let Some(host) = ws_host(&settings) {
                        parts.push(format!("ws-headers=Host:{host}"));
                    }
                }
                "grpc" => {
                    if let Some(service) = grpc_service(&settings) {
                        parts.push(format!("grpc-service-name={service}"));
                    }
                }
                _ => {}
            }
            Some(comma(parts))
        }
        "trojan" => {
            let mut parts = vec![
                format!("{name}{sep}trojan"),
                host,
                port,
                format!("password={password}"),
                "tfo=true".into(),
                "udp-relay=true".into(),
            ];
            if let Some(sni) = server_name(&settings) {
                parts.push(format!("sni={sni}"));
            }
            if bool_field(&tls_settings(&settings), "allow_insecure") {
                parts.push("skip-cert-verify=true".into());
            }
            Some(comma(parts))
        }
        "hysteria" => {
            let version = settings
                .get("version")
                .and_then(|v| v.as_i64())
                .unwrap_or(1);
            if version != 2 {
                return None;
            }
            let mut parts = vec![
                format!("{name}{sep}hysteria2"),
                host,
                port,
                format!("password={password}"),
                "udp-relay=true".into(),
            ];
            if let Some(sni) = server_name(&settings) {
                parts.push(format!("sni={sni}"));
            }
            if bool_field(&tls_settings(&settings), "allow_insecure") {
                parts.push("skip-cert-verify=true".into());
            }
            Some(comma(parts))
        }
        "anytls" => {
            let mut parts = vec![
                format!("{name}{sep}anytls"),
                host,
                port,
                format!("password={password}"),
                "udp-relay=true".into(),
            ];
            if let Some(sni) = server_name(&settings) {
                parts.push(format!("sni={sni}"));
            }
            if bool_field(&tls_settings(&settings), "allow_insecure") {
                parts.push("skip-cert-verify=true".into());
            }
            Some(comma(parts))
        }
        "socks" => {
            let tls = bool_field(&settings, "tls");
            let mut parts = vec![
                format!("{name}{sep}{}", if tls { "socks5-tls" } else { "socks5" }),
                host,
                port,
                password.clone(),
                password,
            ];
            if tls {
                if let Some(sni) = server_name(&settings) {
                    parts.push(format!("sni={sni}"));
                }
                if bool_field(&tls_settings(&settings), "allow_insecure") {
                    parts.push("skip-cert-verify=true".into());
                }
            }
            parts.push("udp-relay=true".into());
            Some(comma(parts))
        }
        "http" => {
            let tls = bool_field(&settings, "tls");
            let mut parts = vec![
                format!("{name}{sep}{}", if tls { "https" } else { "http" }),
                host,
                port,
                password.clone(),
                password,
            ];
            if tls {
                if let Some(sni) = server_name(&settings) {
                    parts.push(format!("sni={sni}"));
                }
            }
            Some(comma(parts))
        }
        _ => None,
    }
}

fn surge_like(params: &Value, template_name: &str, default_template: &str, spaced: bool) -> String {
    let app = app_name(params);
    let templates = params.get("templates").cloned().unwrap_or_else(|| json!({}));
    let mut config = templates
        .get(template_name)
        .and_then(|v| v.as_str())
        .filter(|v| !v.trim().is_empty())
        .unwrap_or(default_template)
        .to_string();

    let mut proxies = String::new();
    let mut names: Vec<String> = Vec::new();
    for server in servers(params) {
        if let Some(proxy) = surge_proxy(&server, spaced) {
            names.push(string_field(&server, "name"));
            proxies.push_str(&proxy);
        }
    }
    let host = params
        .get("host")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    config = config.replace("$subs_link", &subscribe_link(params));
    config = config.replace("$subs_domain", host);
    config = config.replace("$proxies", proxies.trim_end());
    config = config.replace("$proxy_group", &names.join(", "));
    config = config.replace("$subscribe_info", &subscribe_info(params, &app));
    config
}

fn loon_proxy(server: &Value) -> Option<String> {
    let ty = string_field(server, "type");
    let name = string_field(server, "name");
    let host = string_field(server, "host");
    let port = server_port(server);
    let password = string_field(server, "password");
    let uuid = string_field(server, "uuid");
    let settings = settings(server);

    match ty.as_str() {
        "shadowsocks" => {
            let mut parts = vec![
                format!("{name}=Shadowsocks"),
                host,
                port,
                string_field(&settings, "cipher"),
                password,
                "fast-open=false".into(),
                "udp=true".into(),
            ];
            if let Some(plugin) = settings.get("plugin").and_then(|v| v.as_str()) {
                if plugin == "obfs" {
                    let opts = settings.get("plugin_opts").and_then(|v| v.as_str()).unwrap_or("");
                    for pair in opts.split(';').filter(|v| !v.is_empty()) {
                        if let Some((key, value)) = pair.split_once('=') {
                            match key.trim() {
                                "obfs" => parts.push(format!("obfs-name={value}")),
                                "obfs-host" => parts.push(format!("obfs-host={value}")),
                                "path" => parts.push(format!("obfs-uri={value}")),
                                _ => {}
                            }
                        }
                    }
                }
            }
            Some(comma(parts))
        }
        "vmess" => {
            let mut parts = vec![
                format!("{name}=vmess"),
                host,
                port,
                "auto".into(),
                uuid,
                "fast-open=false".into(),
                "udp=true".into(),
                "alterId=0".into(),
            ];
            if bool_field(&settings, "tls") {
                parts.push("over-tls=true".into());
                parts.push(format!(
                    "skip-cert-verify={}",
                    bool_field(&tls_settings(&settings), "allow_insecure")
                ));
                if let Some(sni) = server_name(&settings) {
                    parts.push(format!("tls-name={sni}"));
                }
            }
            match settings.get("network").and_then(|v| v.as_str()).unwrap_or("tcp") {
                "ws" | "httpupgrade" => {
                    parts.push("transport=ws".into());
                    if let Some(path) = ws_path(&settings) {
                        parts.push(format!("path={path}"));
                    }
                    if let Some(host) = ws_host(&settings) {
                        parts.push(format!("host={host}"));
                    }
                }
                "grpc" => {
                    parts.push("transport=grpc".into());
                    if let Some(service) = grpc_service(&settings) {
                        parts.push(format!("grpc-service-name={service}"));
                    }
                }
                _ => parts.push("transport=tcp".into()),
            }
            Some(comma(parts))
        }
        "trojan" => {
            let mut parts = vec![format!("{name}=trojan"), host, port, password];
            if let Some(sni) = server_name(&settings) {
                parts.push(format!("tls-name={sni}"));
            }
            parts.push(format!(
                "skip-cert-verify={}",
                bool_field(&tls_settings(&settings), "allow_insecure")
            ));
            match settings.get("network").and_then(|v| v.as_str()).unwrap_or("tcp") {
                "ws" => {
                    parts.push("transport=ws".into());
                    if let Some(path) = ws_path(&settings) {
                        parts.push(format!("path={path}"));
                    }
                    if let Some(host) = ws_host(&settings) {
                        parts.push(format!("host={host}"));
                    }
                }
                "grpc" => {
                    parts.push("transport=grpc".into());
                    if let Some(service) = grpc_service(&settings) {
                        parts.push(format!("grpc-service-name={service}"));
                    }
                }
                _ => {}
            }
            Some(comma(parts))
        }
        "vless" => {
            let mut parts = vec![
                format!("{name}=VLESS"),
                host,
                port,
                uuid,
                "alterId=0".into(),
                "udp=true".into(),
            ];
            if let Some(flow) = settings.get("flow").and_then(|v| v.as_str()) {
                if !flow.is_empty() {
                    parts.push(format!("flow={flow}"));
                }
            }
            let tls_mode = settings.get("tls").and_then(|v| v.as_i64()).unwrap_or(0);
            if tls_mode > 0 {
                parts.push("over-tls=true".into());
                if let Some(sni) = server_name(&settings) {
                    parts.push(format!("sni={sni}"));
                }
            }
            if let Some(service) = grpc_service(&settings) {
                parts.push("transport=grpc".into());
                parts.push(format!("grpc-service-name={service}"));
            }
            Some(comma(parts))
        }
        "hysteria" => {
            let version = settings.get("version").and_then(|v| v.as_i64()).unwrap_or(1);
            if version != 2 {
                return None;
            }
            let mut parts = vec![
                format!("{name}=Hysteria2"),
                host,
                port,
                password,
                server_name(&settings).unwrap_or_else(|| "(null)".into()),
                "udp=true".into(),
            ];
            if bool_field(&tls_settings(&settings), "allow_insecure") {
                parts.push("skip-cert-verify=true".into());
            }
            Some(comma(parts))
        }
        "anytls" => {
            let mut parts = vec![
                format!("{name}=anytls"),
                host,
                port,
                password,
                "udp=true".into(),
            ];
            if let Some(sni) = server_name(&settings) {
                parts.push(format!("sni={sni}"));
            }
            Some(comma(parts))
        }
        _ => None,
    }
}

fn loon_render(params: &Value) -> String {
    let mut out = String::new();
    for server in servers(params) {
        if let Some(proxy) = loon_proxy(&server) {
            out.push_str(&proxy);
        }
    }
    out
}

fn qx_addr(host: &str) -> String {
    if host.contains(':') && !host.starts_with('[') {
        format!("[{host}]")
    } else {
        host.to_string()
    }
}

fn qx_common(mut parts: Vec<String>, server: &Value, ty: &str) -> Vec<String> {
    parts.push("fast-open=true".into());
    if ty != "http" {
        parts.push("udp-relay=true".into());
    }
    parts.push(format!("tag={}", string_field(server, "name")));
    parts
}

fn qx_proxy(server: &Value) -> Option<String> {
    let ty = string_field(server, "type");
    let host = qx_addr(&string_field(server, "host"));
    let port = server_port(server);
    let password = string_field(server, "password");
    let uuid = string_field(server, "uuid");
    let settings = settings(server);
    let tls = settings.get("tls").and_then(|v| v.as_i64()).unwrap_or(0);
    let network = settings.get("network").and_then(|v| v.as_str()).unwrap_or("tcp");

    let mut parts = match ty.as_str() {
        "shadowsocks" => vec![
            format!("shadowsocks={host}:{port}"),
            format!("method={}", string_field(&settings, "cipher")),
            format!("password={password}"),
        ],
        "vmess" => vec![
            format!("vmess={host}:{port}"),
            "method=auto".into(),
            format!("password={uuid}"),
        ],
        "vless" => {
            let mut parts = vec![
                format!("vless={host}:{port}"),
                "method=none".into(),
                format!("password={uuid}"),
            ];
            if let Some(flow) = settings.get("flow").and_then(|v| v.as_str()) {
                if !flow.is_empty() {
                    parts.push(format!("vless-flow={flow}"));
                }
            }
            parts
        }
        "trojan" => vec![
            format!("trojan={host}:{port}"),
            format!("password={password}"),
            "over-tls=true".into(),
        ],
        "anytls" => {
            let mut parts = vec![
                format!("anytls={host}:{port}"),
                format!("password={password}"),
                "over-tls=true".into(),
            ];
            if bool_field(&tls_settings(&settings), "allow_insecure") {
                parts.push("tls-verification=false".into());
            } else {
                parts.push("tls-verification=true".into());
            }
            if let Some(sni) = server_name(&settings) {
                parts.push(format!("tls-host={sni}"));
            }
            parts
        }
        "socks" => vec![
            format!("socks5={host}:{port}"),
            format!("username={password}"),
            format!("password={password}"),
        ],
        "http" => vec![
            format!("http={host}:{port}"),
            format!("username={password}"),
            format!("password={password}"),
        ],
        _ => return None,
    };

    if matches!(ty.as_str(), "vmess" | "vless" | "trojan" | "socks" | "http") {
        match network {
            "ws" | "httpupgrade" => {
                parts.push(if tls > 0 { "obfs=wss".into() } else { "obfs=ws".into() });
                if let Some(path) = ws_path(&settings) {
                    parts.push(format!("obfs-uri={path}"));
                }
                if let Some(host) = ws_host(&settings) {
                    parts.push(format!("obfs-host={host}"));
                }
            }
            _ => {}
        }
        if tls > 0 {
            parts.push("tls-verification=true".into());
            if let Some(sni) = server_name(&settings) {
                parts.push(format!("tls-host={sni}"));
            }
        }
    }
    if ty == "trojan" {
        if let Some(sni) = server_name(&settings) {
            parts.push(format!("tls-host={sni}"));
        }
    }

    parts = qx_common(parts, server, &ty);
    Some(comma(parts))
}

fn qx_render(params: &Value) -> String {
    let mut raw = String::new();
    for server in servers(params) {
        if let Some(proxy) = qx_proxy(&server) {
            raw.push_str(&proxy);
        }
    }
    base64::engine::general_purpose::STANDARD.encode(raw.as_bytes())
}

#[async_trait]
impl NativePlugin for SurgePlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "surge".into(),
            name: "Surge".into(),
            kind: PluginKind::Output,
            version: "1.0.0".into(),
            description: "Surge 配置".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# Surge\n\n使用 `v2_subscribe_templates` 中的 `surge` 模板，缺省使用内置模板。".into(),
        }
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        if method != "subscribe.render" {
            anyhow::bail!("unsupported method: {method}");
        }
        let body = surge_like(
            params,
            "surge",
            include_str!("../../../../resources/rules/default.surge.conf"),
            true,
        );
        Ok(json!({"body": body, "content_type": "application/octet-stream"}))
    }
}

#[async_trait]
impl NativePlugin for SurfboardPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "surfboard".into(),
            name: "Surfboard".into(),
            kind: PluginKind::Output,
            version: "1.0.0".into(),
            description: "Surfboard 配置".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# Surfboard\n\n使用 `v2_subscribe_templates` 中的 `surfboard` 模板，缺省使用内置模板。".into(),
        }
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        if method != "subscribe.render" {
            anyhow::bail!("unsupported method: {method}");
        }
        let body = surge_like(
            params,
            "surfboard",
            include_str!("../../../../resources/rules/default.surfboard.conf"),
            false,
        );
        Ok(json!({"body": body, "content_type": "application/octet-stream"}))
    }
}

#[async_trait]
impl NativePlugin for LoonPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "loon".into(),
            name: "Loon".into(),
            kind: PluginKind::Output,
            version: "1.0.0".into(),
            description: "Loon 节点列表".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# Loon\n\n输出 Loon 节点配置行。".into(),
        }
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        if method != "subscribe.render" {
            anyhow::bail!("unsupported method: {method}");
        }
        Ok(json!({"body": loon_render(params), "content_type": "text/plain; charset=utf-8"}))
    }
}

#[async_trait]
impl NativePlugin for QuantumultXPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "quantumultx".into(),
            name: "Quantumult X".into(),
            kind: PluginKind::Output,
            version: "1.0.0".into(),
            description: "Quantumult X 节点列表".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# Quantumult X\n\n输出 Base64 编码的 Quantumult X 节点配置。".into(),
        }
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        if method != "subscribe.render" {
            anyhow::bail!("unsupported method: {method}");
        }
        Ok(json!({"body": qx_render(params), "content_type": "text/plain; charset=utf-8"}))
    }
}

#[async_trait]
impl NativePlugin for StashPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "stash".into(),
            name: "Stash".into(),
            kind: PluginKind::Output,
            version: "1.0.0".into(),
            description: "Stash（Clash 兼容）配置".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# Stash\n\n复用 Clash YAML 配置。".into(),
        }
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        if method != "subscribe.render" {
            anyhow::bail!("unsupported method: {method}");
        }
        let body = super::output::clash_render(params)
            .ok_or_else(|| anyhow::anyhow!("stash render failed"))?;
        Ok(json!({"body": body, "content_type": "text/yaml; charset=utf-8"}))
    }
}

#[async_trait]
impl NativePlugin for ShadowrocketPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "shadowrocket".into(),
            name: "Shadowrocket".into(),
            kind: PluginKind::Output,
            version: "1.0.0".into(),
            description: "Shadowrocket 订阅".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# Shadowrocket\n\n输出 Base64 节点链接，Shadowrocket 可直接识别。".into(),
        }
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        if method != "subscribe.render" {
            anyhow::bail!("unsupported method: {method}");
        }
        let links = servers(params)
            .iter()
            .map(crate::api::subscribe::build_link)
            .collect::<Vec<_>>()
            .join("");
        let body = base64::engine::general_purpose::STANDARD.encode(links.as_bytes());
        Ok(json!({"body": body, "content_type": "text/plain; charset=utf-8"}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> Value {
        json!({
            "user": {"u": 1073741824i64, "d": 0, "transfer_enable": 107374182400i64, "expired_at": null, "token": "tok"},
            "settings": {"app_name": "TestPanel", "app_url": "https://panel.example.com", "subscribe_path": "s"},
            "host": "panel.example.com",
            "templates": {},
            "servers": [
                {"type": "shadowsocks", "name": "SS", "host": "ss.example.com", "port": 8388, "password": "pw", "protocol_settings": {"cipher": "aes-256-gcm"}},
                {"type": "vmess", "name": "VM", "host": "vm.example.com", "port": 443, "uuid": "uuid", "protocol_settings": {"tls": 1, "network": "ws", "tls_settings": {"server_name": "vm.example.com"}, "network_settings": {"path": "/ws", "headers": {"Host": "vm.example.com"}}}},
                {"type": "trojan", "name": "TJ", "host": "tj.example.com", "port": 443, "password": "pw", "protocol_settings": {"tls_settings": {"server_name": "tj.example.com"}}}
            ]
        })
    }

    #[test]
    fn surge_contains_template_sections_and_proxies() {
        let body = surge_like(&params(), "surge", include_str!("../../../../resources/rules/default.surge.conf"), true);
        assert!(body.contains("SS = ss"));
        assert!(body.contains("VM = vmess"));
        assert!(body.contains("TJ = trojan"));
        assert!(body.contains("[Proxy]"));
        assert!(body.contains("https://panel.example.com/s/tok"));
    }

    #[test]
    fn loon_and_qx_render_nodes() {
        let loon = loon_render(&params());
        assert!(loon.contains("SS=Shadowsocks"));
        assert!(loon.contains("VM=vmess"));
        assert!(loon.contains("TJ=trojan"));
        let qx = qx_render(&params());
        let raw = String::from_utf8(
            base64::engine::general_purpose::STANDARD.decode(qx).unwrap(),
        )
        .unwrap();
        assert!(raw.contains("shadowsocks=ss.example.com:8388"));
        assert!(raw.contains("vmess=vm.example.com:443"));
        assert!(raw.contains("trojan=tj.example.com:443"));
    }
}
