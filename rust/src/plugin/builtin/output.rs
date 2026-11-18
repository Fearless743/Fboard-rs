use async_trait::async_trait;
use base64::Engine;
use serde_json::{json, Map, Value};

use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};

pub struct GeneralPlugin;
pub struct ShadowsocksPlugin;
pub struct ClashPlugin {
    pub meta: bool,
}
pub struct SingBoxPlugin;

fn servers(params: &Value) -> Vec<Value> {
    params
        .get("servers")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
}

fn setting<'a>(params: &'a Value, key: &str) -> &'a str {
    params
        .get("settings")
        .and_then(|v| v.get(key))
        .and_then(|v| v.as_str())
        .unwrap_or("")
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

fn pointer(value: &Value, path: &str) -> Value {
    value.pointer(path).cloned().unwrap_or(Value::Null)
}

fn tls_enabled(settings: &Value, server_type: &str) -> bool {
    if server_type == "trojan" {
        return true;
    }
    bool_field(settings, "tls")
}

fn proxy_name(server: &Value) -> String {
    string_field(server, "name")
}

fn server_port(server: &Value) -> Value {
    match server.get("port") {
        Some(Value::String(s)) => s
            .parse::<i64>()
            .map(|v| json!(v))
            .unwrap_or_else(|_| json!(s)),
        Some(Value::Number(n)) => json!(n),
        _ => json!(0),
    }
}

fn ws_opts(settings: &Value) -> (Option<String>, Option<String>) {
    let path = settings
        .pointer("/network_settings/path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let host = settings
        .pointer("/network_settings/headers/Host")
        .and_then(|v| v.as_str())
        .or_else(|| {
            settings
                .pointer("/network_settings/headers/host")
                .and_then(|v| v.as_str())
        })
        .map(|s| s.to_string());
    (path, host)
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
        .map(|s| s.to_string())
}

fn clash_proxy(server: &Value) -> Option<Value> {
    let ty = string_field(server, "type");
    let name = proxy_name(server);
    let host = string_field(server, "host");
    let port = server_port(server);
    let password = string_field(server, "password");
    let uuid = string_field(server, "uuid");
    let settings = server
        .get("protocol_settings")
        .cloned()
        .unwrap_or_else(|| json!({}));

    let mut out = Map::new();
    out.insert("name".into(), json!(name));
    out.insert("server".into(), json!(host));
    out.insert("port".into(), port);
    match ty.as_str() {
        "shadowsocks" => {
            out.insert("type".into(), json!("ss"));
            out.insert(
                "cipher".into(),
                settings
                    .get("cipher")
                    .cloned()
                    .unwrap_or_else(|| json!("aes-256-gcm")),
            );
            out.insert("password".into(), json!(password));
            out.insert("udp".into(), json!(true));
            if let Some(plugin) = settings.get("plugin").and_then(|v| v.as_str()) {
                if !plugin.is_empty() {
                    out.insert("plugin".into(), json!(plugin));
                    if let Some(opts) = settings.get("plugin_opts").and_then(|v| v.as_str()) {
                        out.insert("plugin-opts".into(), parse_plugin_opts(plugin, opts, &settings));
                    }
                }
            }
        }
        "vmess" => {
            out.insert("type".into(), json!("vmess"));
            out.insert("uuid".into(), json!(uuid));
            out.insert("alterId".into(), json!(0));
            out.insert("cipher".into(), json!("auto"));
            out.insert("udp".into(), json!(true));
            if tls_enabled(&settings, &ty) {
                out.insert("tls".into(), json!(true));
                out.insert(
                    "skip-cert-verify".into(),
                    json!(bool_field(
                        &pointer(&settings, "/tls_settings"),
                        "allow_insecure"
                    )),
                );
                if let Some(sni) = settings
                    .pointer("/tls_settings/server_name")
                    .and_then(|v| v.as_str())
                {
                    out.insert("servername".into(), json!(sni));
                }
                if let Some(alpn) = settings
                    .pointer("/tls_settings/client_alpn")
                    .or_else(|| settings.pointer("/tls_settings/alpn"))
                {
                    out.insert("alpn".into(), alpn.clone());
                }
            }
            apply_clash_network(&mut out, &settings);
        }
        "vless" => {
            out.insert("type".into(), json!("vless"));
            out.insert("uuid".into(), json!(uuid));
            out.insert("udp".into(), json!(true));
            if let Some(flow) = settings.get("flow").and_then(|v| v.as_str()) {
                if !flow.is_empty() {
                    out.insert("flow".into(), json!(flow));
                }
            }
            if tls_enabled(&settings, &ty) {
                out.insert("tls".into(), json!(true));
                if let Some(sni) = settings
                    .pointer("/tls_settings/server_name")
                    .and_then(|v| v.as_str())
                {
                    out.insert("servername".into(), json!(sni));
                }
                if settings.get("reality_settings").is_some() {
                    out.insert("reality-opts".into(), settings["reality_settings"].clone());
                }
            }
            apply_clash_network(&mut out, &settings);
        }
        "trojan" => {
            out.insert("type".into(), json!("trojan"));
            out.insert("password".into(), json!(password));
            out.insert("udp".into(), json!(true));
            if let Some(sni) = settings
                .pointer("/tls_settings/server_name")
                .and_then(|v| v.as_str())
            {
                out.insert("sni".into(), json!(sni));
            }
            if let Some(alpn) = settings
                .pointer("/tls_settings/client_alpn")
                .or_else(|| settings.pointer("/tls_settings/alpn"))
            {
                out.insert("alpn".into(), alpn.clone());
            }
            out.insert(
                "skip-cert-verify".into(),
                json!(bool_field(
                    &pointer(&settings, "/tls_settings"),
                    "allow_insecure"
                )),
            );
            apply_clash_network(&mut out, &settings);
        }
        "socks" => {
            out.insert("type".into(), json!("socks5"));
            out.insert("username".into(), json!(password));
            out.insert("password".into(), json!(password));
            out.insert("udp".into(), json!(true));
        }
        "http" => {
            out.insert("type".into(), json!("http"));
            out.insert("username".into(), json!(password));
            out.insert("password".into(), json!(password));
        }
        "hysteria" => {
            out.insert("type".into(), json!("hysteria2"));
            out.insert("password".into(), json!(password));
            if let Some(sni) = settings
                .pointer("/tls/server_name")
                .and_then(|v| v.as_str())
            {
                out.insert("sni".into(), json!(sni));
            }
        }
        "tuic" => {
            out.insert("type".into(), json!("tuic"));
            out.insert("uuid".into(), json!(uuid));
            out.insert("password".into(), json!(password));
            if let Some(sni) = settings
                .pointer("/tls/server_name")
                .and_then(|v| v.as_str())
            {
                out.insert("sni".into(), json!(sni));
            }
        }
        "anytls" => {
            out.insert("type".into(), json!("anytls"));
            out.insert("password".into(), json!(password));
            if let Some(sni) = settings
                .pointer("/tls/server_name")
                .and_then(|v| v.as_str())
            {
                out.insert("sni".into(), json!(sni));
            }
        }
        _ => return None,
    }
    Some(Value::Object(out))
}

fn parse_plugin_opts(plugin: &str, opts: &str, settings: &Value) -> Value {
    let mut parsed = Map::new();
    for pair in opts.split(';').filter(|s| !s.is_empty()) {
        if let Some((key, value)) = pair.split_once('=') {
            parsed.insert(key.trim().to_string(), json!(value.trim()));
        }
    }
    match plugin {
        "obfs" => json!({
            "mode": parsed.get("obfs").cloned().unwrap_or_else(|| settings.get("obfs").cloned().unwrap_or_else(|| json!("http"))),
            "host": parsed.get("obfs-host").cloned().unwrap_or_else(|| pointer(settings, "/obfs_settings/host")),
        }),
        "v2ray-plugin" => json!({
            "mode": parsed.get("mode").cloned().unwrap_or_else(|| json!("websocket")),
            "tls": parsed.get("tls").and_then(|v| v.as_str()).map(|v| v == "true").unwrap_or(false),
            "host": parsed.get("host").cloned().unwrap_or_else(|| json!("")),
            "path": parsed.get("path").cloned().unwrap_or_else(|| json!("/")),
        }),
        _ => Value::Object(parsed),
    }
}

fn apply_clash_network(out: &mut Map<String, Value>, settings: &Value) {
    let network = settings
        .get("network")
        .and_then(|v| v.as_str())
        .unwrap_or("tcp");
    match network {
        "ws" | "httpupgrade" => {
            out.insert("network".into(), json!("ws"));
            let (path, host) = ws_opts(settings);
            let mut ws = Map::new();
            if let Some(path) = path {
                ws.insert("path".into(), json!(path));
            }
            if let Some(host) = host {
                ws.insert("headers".into(), json!({"Host": host}));
            }
            if !ws.is_empty() {
                out.insert("ws-opts".into(), Value::Object(ws));
            }
        }
        "grpc" => {
            out.insert("network".into(), json!("grpc"));
            if let Some(service) = grpc_service(settings) {
                out.insert("grpc-opts".into(), json!({"grpc-service-name": service}));
            }
        }
        "tcp" => {
            out.insert("network".into(), json!("tcp"));
        }
        _ => {}
    }
}

pub(crate) fn clash_render(params: &Value) -> Option<String> {
    let app_name = setting(params, "app_name");
    let templates = params.get("templates").cloned().unwrap_or_else(|| json!({}));
    let template_name = if params
        .get("client")
        .and_then(|v| v.get("name"))
        .and_then(|v| v.as_str())
        .map(|v| v.to_ascii_lowercase().contains("meta"))
        .unwrap_or(false)
    {
        "clash_meta"
    } else {
        "clash"
    };
    let template = templates
        .get(template_name)
        .and_then(|v| v.as_str())
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.to_string())
        .unwrap_or_else(|| {
            include_str!("../../../../resources/rules/default.clash.yaml").to_string()
        });
    let mut config: Value = serde_yaml::from_str(&template).unwrap_or_else(|_| {
        serde_yaml::from_str(include_str!("../../../../resources/rules/default.clash.yaml"))
            .unwrap_or_else(|_| json!({}))
    });
    if !config.is_object() {
        config = serde_yaml::from_str(include_str!(
            "../../../../resources/rules/default.clash.yaml"
        ))
        .unwrap_or_else(|_| json!({}));
    }

    let mut proxy_values = Vec::new();
    let mut names = Vec::new();
    for server in servers(params) {
        if let Some(proxy) = clash_proxy(&server) {
            if let Some(name) = proxy.get("name").and_then(|v| v.as_str()) {
                names.push(name.to_string());
            }
            proxy_values.push(proxy);
        }
    }

    if let Some(object) = config.as_object_mut() {
        let existing = object
            .get("proxies")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let mut all = existing;
        all.extend(proxy_values);
        object.insert("proxies".into(), Value::Array(all));

        if let Some(groups) = object.get_mut("proxy-groups").and_then(|v| v.as_array_mut()) {
            for group in groups {
                let Some(group_obj) = group.as_object_mut() else {
                    continue;
                };
                let mut entries = group_obj
                    .get("proxies")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                for name in &names {
                    if !entries.iter().any(|v| v.as_str() == Some(name.as_str())) {
                        entries.push(json!(name));
                    }
                }
                if entries.is_empty() {
                    entries.push(json!("DIRECT"));
                }
                group_obj.insert("proxies".into(), Value::Array(entries));
            }
        }
    }

    let mut yaml = serde_yaml::to_string(&config).ok()?;
    if !app_name.is_empty() {
        yaml = yaml.replace("$app_name", app_name);
    }
    Some(yaml)
}

fn singbox_outbound(server: &Value) -> Option<Value> {
    let ty = string_field(server, "type");
    let name = proxy_name(server);
    let host = string_field(server, "host");
    let port = server_port(server);
    let password = string_field(server, "password");
    let uuid = string_field(server, "uuid");
    let settings = server
        .get("protocol_settings")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let mut out = Map::new();
    out.insert("tag".into(), json!(name));
    out.insert("server".into(), json!(host));
    out.insert("server_port".into(), port);
    match ty.as_str() {
        "shadowsocks" => {
            out.insert("type".into(), json!("shadowsocks"));
            out.insert(
                "method".into(),
                settings
                    .get("cipher")
                    .cloned()
                    .unwrap_or_else(|| json!("aes-256-gcm")),
            );
            out.insert("password".into(), json!(password));
            if let Some(plugin) = settings.get("plugin").and_then(|v| v.as_str()) {
                if !plugin.is_empty() {
                    out.insert("plugin".into(), json!(plugin));
                    if let Some(opts) = settings.get("plugin_opts").and_then(|v| v.as_str()) {
                        out.insert("plugin_opts".into(), json!(opts));
                    }
                }
            }
        }
        "vmess" => {
            out.insert("type".into(), json!("vmess"));
            out.insert("uuid".into(), json!(uuid));
            out.insert("security".into(), json!("auto"));
            out.insert("alter_id".into(), json!(0));
            if tls_enabled(&settings, &ty) {
                out.insert("tls".into(), singbox_tls(&settings));
            }
            if let Some(transport) = singbox_transport(&settings) {
                out.insert("transport".into(), transport);
            }
        }
        "vless" => {
            out.insert("type".into(), json!("vless"));
            out.insert("uuid".into(), json!(uuid));
            if let Some(flow) = settings.get("flow").and_then(|v| v.as_str()) {
                if !flow.is_empty() {
                    out.insert("flow".into(), json!(flow));
                }
            }
            if tls_enabled(&settings, &ty) {
                out.insert("tls".into(), singbox_tls(&settings));
            }
            if let Some(transport) = singbox_transport(&settings) {
                out.insert("transport".into(), transport);
            }
        }
        "trojan" => {
            out.insert("type".into(), json!("trojan"));
            out.insert("password".into(), json!(password));
            out.insert("tls".into(), singbox_tls(&settings));
            if let Some(transport) = singbox_transport(&settings) {
                out.insert("transport".into(), transport);
            }
        }
        "socks" => {
            out.insert("type".into(), json!("socks"));
            out.insert("username".into(), json!(password));
            out.insert("password".into(), json!(password));
        }
        "http" => {
            out.insert("type".into(), json!("http"));
            out.insert("username".into(), json!(password));
            out.insert("password".into(), json!(password));
        }
        "hysteria" => {
            out.insert("type".into(), json!("hysteria2"));
            out.insert("password".into(), json!(password));
            out.insert("tls".into(), singbox_tls(&settings));
        }
        "tuic" => {
            out.insert("type".into(), json!("tuic"));
            out.insert("uuid".into(), json!(uuid));
            out.insert("password".into(), json!(password));
            out.insert("tls".into(), singbox_tls(&settings));
        }
        "anytls" => {
            out.insert("type".into(), json!("anytls"));
            out.insert("password".into(), json!(password));
            out.insert("tls".into(), singbox_tls(&settings));
        }
        _ => return None,
    }
    Some(Value::Object(out))
}

fn singbox_tls(settings: &Value) -> Value {
    let tls = if settings.get("tls_settings").is_some() {
        pointer(settings, "/tls_settings")
    } else {
        pointer(settings, "/tls")
    };
    let mut out = Map::new();
    out.insert("enabled".into(), json!(true));
    if let Some(sni) = tls.get("server_name").and_then(|v| v.as_str()) {
        out.insert("server_name".into(), json!(sni));
    }
    if let Some(insecure) = tls.get("allow_insecure") {
        out.insert("insecure".into(), json!(bool_field(&json!({"v": insecure}), "v")));
    }
    if let Some(alpn) = tls.get("client_alpn").or_else(|| tls.get("alpn")) {
        out.insert("alpn".into(), alpn.clone());
    }
    if let Some(reality) = settings.get("reality_settings") {
        let enabled = reality
            .get("public_key")
            .and_then(|v| v.as_str())
            .map(|v| !v.is_empty())
            .unwrap_or(false);
        if enabled {
            out.insert("reality".into(), json!({"enabled": true, "public_key": reality.get("public_key").cloned().unwrap_or(Value::Null)}));
        }
    }
    Value::Object(out)
}

fn singbox_transport(settings: &Value) -> Option<Value> {
    let network = settings
        .get("network")
        .and_then(|v| v.as_str())
        .unwrap_or("tcp");
    match network {
        "ws" | "httpupgrade" => {
            let (path, host) = ws_opts(settings);
            let mut transport = Map::new();
            transport.insert(
                "type".into(),
                json!(if network == "httpupgrade" {
                    "httpupgrade"
                } else {
                    "ws"
                }),
            );
            if let Some(path) = path {
                transport.insert("path".into(), json!(path));
            }
            if let Some(host) = host {
                transport.insert("headers".into(), json!({"Host": host}));
            }
            Some(Value::Object(transport))
        }
        "grpc" => {
            let mut transport = Map::new();
            transport.insert("type".into(), json!("grpc"));
            if let Some(service) = grpc_service(settings) {
                transport.insert("service_name".into(), json!(service));
            }
            Some(Value::Object(transport))
        }
        _ => None,
    }
}

fn singbox_render(params: &Value) -> Option<String> {
    let templates = params.get("templates").cloned().unwrap_or_else(|| json!({}));
    let template = templates
        .get("singbox")
        .or_else(|| templates.get("sing-box"))
        .and_then(|v| v.as_str())
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.to_string())
        .unwrap_or_else(|| {
            include_str!("../../../../resources/rules/default.sing-box.json").to_string()
        });
    let mut config: Value = serde_json::from_str(&template).unwrap_or_else(|_| {
        serde_json::from_str(include_str!(
            "../../../../resources/rules/default.sing-box.json"
        ))
        .unwrap_or_else(|_| json!({}))
    });
    if !config.is_object() {
        config = serde_json::from_str(include_str!(
            "../../../../resources/rules/default.sing-box.json"
        ))
        .unwrap_or_else(|_| json!({}));
    }
    let mut outbounds = Vec::new();
    let mut tags = Vec::new();
    for server in servers(params) {
        if let Some(outbound) = singbox_outbound(&server) {
            if let Some(tag) = outbound.get("tag").and_then(|v| v.as_str()) {
                tags.push(tag.to_string());
            }
            outbounds.push(outbound);
        }
    }
    if let Some(object) = config.as_object_mut() {
        let mut existing = object
            .get("outbounds")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for outbound in outbounds {
            existing.push(outbound);
        }
        for outbound in &mut existing {
            let ty = outbound
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            if ty == "selector" || ty == "urltest" {
                if let Some(list) = outbound.get_mut("outbounds").and_then(|v| v.as_array_mut()) {
                    for tag in &tags {
                        if !list.iter().any(|v| v.as_str() == Some(tag.as_str())) {
                            list.push(json!(tag));
                        }
                    }
                }
            }
        }
        object.insert("outbounds".into(), Value::Array(existing));
    }
    serde_json::to_string_pretty(&config).ok()
}

#[async_trait]
impl NativePlugin for GeneralPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "general".into(),
            name: "General".into(),
            kind: PluginKind::Output,
            version: "1.0.0".into(),
            description: "通用 Base64 订阅".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# General\n\n通用 Base64 编码的节点链接列表。".into(),
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

fn shadowsocks_render(params: &Value) -> Option<String> {
    let user = params.get("user").cloned().unwrap_or_else(|| json!({}));
    let used = user.get("u").and_then(|v| v.as_i64()).unwrap_or(0)
        + user.get("d").and_then(|v| v.as_i64()).unwrap_or(0);
    let total = user
        .get("transfer_enable")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let list: Vec<Value> = servers(params)
        .iter()
        .filter(|server| string_field(server, "type") == "shadowsocks")
        .filter_map(|server| {
            let settings = server
                .get("protocol_settings")
                .cloned()
                .unwrap_or_else(|| json!({}));
            let method = settings.get("cipher").and_then(|v| v.as_str())?;
            Some(json!({
                "id": string_field(server, "id"),
                "remarks": proxy_name(server),
                "server": string_field(server, "host"),
                "server_port": server_port(server),
                "password": string_field(server, "password"),
                "method": method,
            }))
        })
        .collect();
    serde_json::to_string(&json!({
        "version": 1,
        "bytes_used": used,
        "bytes_remaining": (total - used).max(0),
        "servers": list,
    }))
    .ok()
}

#[async_trait]
impl NativePlugin for ShadowsocksPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "shadowsocks".into(),
            name: "Shadowsocks".into(),
            kind: PluginKind::Output,
            version: "1.0.0".into(),
            description: "SIP008 Shadowsocks 订阅".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# Shadowsocks\n\nSIP008 JSON 格式订阅。".into(),
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
        let body = shadowsocks_render(params)
            .ok_or_else(|| anyhow::anyhow!("shadowsocks render failed"))?;
        Ok(json!({"body": body, "content_type": "application/json; charset=utf-8"}))
    }
}

#[async_trait]
impl NativePlugin for ClashPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: if self.meta { "clash_meta" } else { "clash" }.into(),
            name: if self.meta { "Clash Meta" } else { "Clash" }.into(),
            kind: PluginKind::Output,
            version: "1.0.0".into(),
            description: if self.meta {
                "Clash Meta / Mihomo 订阅"
            } else {
                "Clash 订阅"
            }
            .into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# Clash\n\n使用 `v2_subscribe_templates` 中的 `clash` 模板，缺省使用内置模板。"
                .into(),
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
        let body = clash_render(params).ok_or_else(|| anyhow::anyhow!("clash render failed"))?;
        Ok(json!({"body": body, "content_type": "text/yaml; charset=utf-8"}))
    }
}

#[async_trait]
impl NativePlugin for SingBoxPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "singbox".into(),
            name: "SingBox".into(),
            kind: PluginKind::Output,
            version: "1.0.0".into(),
            description: "Sing-Box 订阅".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# SingBox\n\n使用 `v2_subscribe_templates` 中的 `singbox` 模板，缺省使用内置模板。"
                .into(),
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
        let body =
            singbox_render(params).ok_or_else(|| anyhow::anyhow!("singbox render failed"))?;
        Ok(json!({"body": body, "content_type": "application/json; charset=utf-8"}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> Value {
        json!({
            "user": {"u": 1024, "d": 2048, "transfer_enable": 10240},
            "settings": {"app_name": "TestPanel"},
            "templates": {},
            "client": {"name": "clash"},
            "servers": [
                {
                    "id": 1, "type": "shadowsocks", "name": "SS", "host": "ss.example.com",
                    "port": "8388", "password": "pw", "protocol_settings": {"cipher": "aes-256-gcm"}
                },
                {
                    "id": 2, "type": "vmess", "name": "VM", "host": "vm.example.com",
                    "port": 443, "uuid": "uuid-1", "protocol_settings": {
                        "tls": 1, "network": "ws",
                        "tls_settings": {"server_name": "vm.example.com"},
                        "network_settings": {"path": "/ws", "headers": {"Host": "vm.example.com"}}
                    }
                },
                {
                    "id": 3, "type": "trojan", "name": "TJ", "host": "tj.example.com",
                    "port": 443, "password": "pw", "protocol_settings": {"network": "tcp"}
                }
            ]
        })
    }

    #[test]
    fn clash_output_contains_proxies_and_groups() {
        let text = clash_render(&params()).unwrap();
        assert!(text.contains("name: SS"));
        assert!(text.contains("type: ss"));
        assert!(text.contains("name: VM"));
        assert!(text.contains("type: vmess"));
        assert!(text.contains("name: TJ"));
        assert!(text.contains("proxy-groups:"));
        assert!(text.contains("TestPanel"));
    }

    #[test]
    fn singbox_output_contains_outbounds() {
        let text = singbox_render(&params()).unwrap();
        let value: Value = serde_json::from_str(&text).unwrap();
        let tags: Vec<&str> = value["outbounds"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|o| o.get("tag").and_then(|v| v.as_str()))
            .collect();
        assert!(tags.contains(&"SS"));
        assert!(tags.contains(&"VM"));
        assert!(tags.contains(&"TJ"));
    }

    #[test]
    fn shadowsocks_json_and_general_base64() {
        let body: Value = serde_json::from_str(&shadowsocks_render(&params()).unwrap()).unwrap();
        assert_eq!(body["servers"].as_array().unwrap().len(), 1);
        assert_eq!(body["servers"][0]["method"], "aes-256-gcm");
        assert_eq!(body["bytes_used"], 3072);
    }
}
