//! Plugin runtime.
//!
//! Core payment, protocol, subscription-output and Telegram features are
//! compiled into the binary. Additional plugins are Lua scripts under
//! `plugins/<code>/plugin.json` + `init.lua`.

mod builtin;
mod lua;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::config::Config;
use crate::settings::Settings;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluginKind {
    Payment,
    Protocol,
    Output,
    Feature,
}

impl PluginKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            PluginKind::Payment => "payment",
            PluginKind::Protocol => "protocol",
            PluginKind::Output => "output",
            PluginKind::Feature => "feature",
        }
    }

    pub fn parse(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "payment" => PluginKind::Payment,
            "protocol" => PluginKind::Protocol,
            "output" => PluginKind::Output,
            _ => PluginKind::Feature,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PluginMeta {
    pub code: String,
    pub name: String,
    pub kind: PluginKind,
    pub version: String,
    pub description: String,
    pub author: String,
    pub builtin: bool,
    pub path: Option<PathBuf>,
    pub config_fields: Value,
    pub readme: String,
}

impl PluginMeta {
    pub fn to_json(&self, enabled: bool, config: Value) -> Value {
        json!({
            "name": self.name,
            "code": self.code,
            "type": self.kind.as_str(),
            "version": self.version,
            "description": self.description,
            "author": self.author,
            "is_enabled": enabled,
            "config": config,
            "is_core": self.builtin,
            "builtin": self.builtin,
            "path": self.path.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
        })
    }
}

pub struct PluginContext<'a> {
    pub db: &'a MySqlPool,
    pub settings: &'a Settings,
    pub app_url: &'a str,
    pub config: Value,
}

impl<'a> PluginContext<'a> {
    pub fn new(db: &'a MySqlPool, settings: &'a Settings, app_url: &'a str, config: Value) -> Self {
        Self {
            db,
            settings,
            app_url,
            config,
        }
    }
}

#[async_trait]
pub trait NativePlugin: Send + Sync {
    fn meta(&self) -> PluginMeta;

    /// Names accepted by `v2_payment.payment` for payment plugins.
    fn payment_methods(&self) -> Vec<String> {
        Vec::new()
    }

    /// Client flags used to select custom subscription-output plugins.
    fn flags(&self) -> Vec<String> {
        Vec::new()
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value>;
}

pub struct PluginRegistry {
    builtin: Vec<Arc<dyn NativePlugin>>,
    lua: RwLock<Vec<Arc<dyn NativePlugin>>>,
    dirs: Vec<PathBuf>,
}

impl PluginRegistry {
    pub fn load(cfg: &Config) -> Arc<Self> {
        let registry = Arc::new(Self {
            builtin: builtin::plugins(),
            lua: RwLock::new(Vec::new()),
            dirs: cfg.plugin_dirs.clone(),
        });
        registry.reload();
        registry
    }

    /// Re-scan Lua plugins from `FBOARD_PLUGIN_DIRS`.
    pub fn reload(&self) {
        let builtin_codes: Vec<String> = self
            .builtin
            .iter()
            .map(|plugin| plugin.meta().code.to_ascii_lowercase())
            .collect();
        let loaded: Vec<Arc<dyn NativePlugin>> = lua::load(&self.dirs)
            .into_iter()
            .filter(|plugin| {
                let code = plugin.meta().code.to_ascii_lowercase();
                if builtin_codes.contains(&code) {
                    tracing::warn!(code = %code, "Lua plugin conflicts with a built-in plugin; ignored");
                    false
                } else {
                    true
                }
            })
            .collect();
        *self.lua.write().unwrap() = loaded;
    }

    pub fn all(&self) -> Vec<Arc<dyn NativePlugin>> {
        let mut out = self.builtin.clone();
        out.extend(self.lua.read().unwrap().iter().cloned());
        out
    }

    pub fn get(&self, code: &str) -> Option<Arc<dyn NativePlugin>> {
        let wanted = code.to_ascii_lowercase();
        self.all()
            .into_iter()
            .find(|plugin| plugin.meta().code.eq_ignore_ascii_case(&wanted))
    }

    pub fn contains(&self, code: &str) -> bool {
        self.get(code).is_some()
    }

    pub fn types(&self) -> Vec<String> {
        let mut types: Vec<String> = self
            .all()
            .iter()
            .map(|plugin| plugin.meta().kind.as_str().to_string())
            .collect();
        types.sort();
        types.dedup();
        types
    }

    pub fn config_fields(&self, code: &str) -> Value {
        self.get(code)
            .map(|plugin| plugin.meta().config_fields)
            .unwrap_or_else(|| json!({}))
    }

    pub fn readme(&self, code: &str) -> String {
        self.get(code)
            .map(|plugin| plugin.meta().readme)
            .unwrap_or_default()
    }

    pub async fn config(&self, db: &MySqlPool, code: &str) -> Value {
        let row: Option<(Option<Vec<u8>>,)> =
            sqlx::query_as("SELECT config FROM v2_plugins WHERE code = ? LIMIT 1")
                .bind(code)
                .fetch_optional(db)
                .await
                .ok()
                .flatten();
        row.and_then(|(raw,)| raw)
            .and_then(|raw| serde_json::from_slice::<Value>(&raw).ok())
            .unwrap_or_else(|| json!({}))
    }

    pub async fn is_enabled(&self, db: &MySqlPool, code: &str) -> bool {
        let default = self
            .get(code)
            .map(|plugin| {
                plugin.meta().builtin
                    && matches!(
                        plugin.meta().kind,
                        PluginKind::Protocol | PluginKind::Output
                    )
            })
            .unwrap_or(false);
        self.is_enabled_or(db, code, default).await
    }

    pub async fn is_enabled_or(&self, db: &MySqlPool, code: &str, default: bool) -> bool {
        let row: Option<(i64,)> =
            sqlx::query_as("SELECT is_enabled FROM v2_plugins WHERE code = ? LIMIT 1")
                .bind(code)
                .fetch_optional(db)
                .await
                .ok()
                .flatten();
        match row {
            Some((enabled,)) => enabled != 0,
            None => default,
        }
    }

    pub async fn enabled_map(&self, db: &MySqlPool) -> HashMap<String, bool> {
        let rows: Vec<(String, i64)> = sqlx::query_as("SELECT code, is_enabled FROM v2_plugins")
            .fetch_all(db)
            .await
            .unwrap_or_default();
        let mut stored: HashMap<String, bool> = rows
            .into_iter()
            .map(|(code, enabled)| (code.to_ascii_lowercase(), enabled != 0))
            .collect();
        let mut out = HashMap::new();
        for plugin in self.all() {
            let meta = plugin.meta();
            let default =
                meta.builtin && matches!(meta.kind, PluginKind::Protocol | PluginKind::Output);
            out.insert(
                meta.code.clone(),
                stored
                    .remove(&meta.code.to_ascii_lowercase())
                    .unwrap_or(default),
            );
        }
        out
    }

    pub async fn call(
        &self,
        code: &str,
        method: &str,
        params: Value,
        ctx: &PluginContext<'_>,
    ) -> Option<Value> {
        let plugin = self.get(code)?;
        match plugin.call(method, &params, ctx).await {
            Ok(value) => Some(value),
            Err(err) => {
                tracing::warn!(code, method, error = %err, "native plugin call failed");
                None
            }
        }
    }

    async fn resolve_payment(&self, method: &str) -> Option<Arc<dyn NativePlugin>> {
        self.all().into_iter().find(|plugin| {
            plugin.meta().kind == PluginKind::Payment
                && (plugin.meta().code.eq_ignore_ascii_case(method)
                    || plugin
                        .payment_methods()
                        .iter()
                        .any(|name| name.eq_ignore_ascii_case(method)))
        })
    }

    pub async fn payment_methods(&self) -> Value {
        let mut out: Vec<String> = Vec::new();
        for plugin in self.all() {
            if plugin.meta().kind != PluginKind::Payment {
                continue;
            }
            for name in plugin.payment_methods() {
                if !out
                    .iter()
                    .any(|existing| existing.eq_ignore_ascii_case(&name))
                {
                    out.push(name);
                }
            }
        }
        Value::Array(out.into_iter().map(Value::String).collect())
    }

    pub async fn payment_pay(
        &self,
        method: &str,
        config: Value,
        order: Value,
        ctx: &PluginContext<'_>,
    ) -> Option<Value> {
        let plugin = self.resolve_payment(method).await?;
        let params = json!({"config": config, "order": order});
        self.call(&plugin.meta().code, "payment.pay", params, ctx)
            .await
    }

    pub async fn payment_notify(
        &self,
        method: &str,
        config: Value,
        params: HashMap<String, Value>,
        raw_body: &str,
        headers: Value,
        ctx: &PluginContext<'_>,
    ) -> Option<Value> {
        let plugin = self.resolve_payment(method).await?;
        let payload = json!({
            "config": config,
            "params": params,
            "raw_body": raw_body,
            "headers": headers,
        });
        self.call(&plugin.meta().code, "payment.notify", payload, ctx)
            .await
    }

    pub async fn payment_form(
        &self,
        method: &str,
        config: Value,
        ctx: &PluginContext<'_>,
    ) -> Option<Value> {
        let plugin = self.resolve_payment(method).await?;
        self.call(
            &plugin.meta().code,
            "payment.form",
            json!({"config": config}),
            ctx,
        )
        .await
    }

    pub async fn protocol_definitions(&self, base: Value, ctx: &PluginContext<'_>) -> Value {
        let mut merged = self
            .call(
                "core_protocols",
                "protocol.definitions",
                json!({"value": base.clone()}),
                ctx,
            )
            .await
            .unwrap_or(base);
        if !merged.is_object() {
            merged = json!({});
        }
        for plugin in self.all() {
            let meta = plugin.meta();
            if meta.builtin || meta.kind != PluginKind::Protocol {
                continue;
            }
            if !db_enabled(ctx.db, &meta.code, false).await {
                continue;
            }
            if let Some(value) = self
                .call(
                    &meta.code,
                    "protocol.definitions",
                    json!({"value": merged.clone()}),
                    ctx,
                )
                .await
            {
                if value.is_object() {
                    merged = value;
                }
            }
        }
        merged
    }

    pub async fn render_subscription(
        &self,
        params: Value,
        ctx: &PluginContext<'_>,
        enabled: &HashMap<String, bool>,
    ) -> Option<Value> {
        let flag = params
            .get("flag")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let client = params
            .get("client")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let haystack = format!("{flag} {client}");
        let code = output_code(&haystack);
        let code = if enabled.get(code).copied().unwrap_or(true) {
            code
        } else {
            "general"
        };

        if code != "general" {
            if let Some(value) = self
                .call(code, "subscribe.render", params.clone(), ctx)
                .await
            {
                if value.get("body").is_some() {
                    return Some(value);
                }
            }
        }

        // Lua output plugins can claim a client via manifest `flags`.
        for plugin in self.all() {
            let meta = plugin.meta();
            if meta.builtin || meta.kind != PluginKind::Output {
                continue;
            }
            if !enabled
                .get(&meta.code.to_ascii_lowercase())
                .copied()
                .unwrap_or(false)
            {
                continue;
            }
            let flags = plugin.flags();
            if flags.is_empty()
                || !flags
                    .iter()
                    .any(|item| haystack.contains(&item.to_ascii_lowercase()))
            {
                continue;
            }
            if let Some(value) = self
                .call(&meta.code, "subscribe.render", params.clone(), ctx)
                .await
            {
                if value.get("body").is_some() {
                    return Some(value);
                }
            }
        }

        self.call("general", "subscribe.render", params, ctx).await
    }

    pub async fn telegram_webhook(
        &self,
        update: &Value,
        enabled: bool,
        ctx: &PluginContext<'_>,
    ) -> Value {
        if !enabled {
            return json!({"handled": false, "enabled": false});
        }
        let Some(value) = self
            .call(
                "telegram",
                "telegram.webhook",
                json!({"update": update}),
                ctx,
            )
            .await
        else {
            return json!({"handled": false});
        };
        value
    }

    pub async fn ui_navigation(&self, ctx: &PluginContext<'_>) -> Value {
        let mut out = Vec::new();
        for plugin in self.all() {
            let meta = plugin.meta();
            if meta.kind != PluginKind::Feature || meta.builtin {
                continue;
            }
            if !db_enabled(ctx.db, &meta.code, false).await {
                continue;
            }
            if let Some(Value::Array(items)) = self
                .call(&plugin.meta().code, "plugin.ui.nav", json!({}), ctx)
                .await
            {
                out.extend(items);
            }
        }
        Value::Array(out)
    }

    pub async fn ui_extensions(&self, ctx: &PluginContext<'_>) -> Value {
        let mut out = Vec::new();
        for plugin in self.all() {
            let meta = plugin.meta();
            if meta.kind != PluginKind::Feature || meta.builtin {
                continue;
            }
            if !db_enabled(ctx.db, &meta.code, false).await {
                continue;
            }
            if let Some(Value::Array(items)) = self
                .call(&plugin.meta().code, "plugin.ui", json!({}), ctx)
                .await
            {
                out.extend(items);
            }
        }
        Value::Array(out)
    }

    pub async fn execute_action(
        &self,
        code: &str,
        action: &str,
        params: Value,
        ctx: &PluginContext<'_>,
    ) -> Option<Value> {
        if !db_enabled(ctx.db, code, false).await {
            return None;
        }
        self.call(
            code,
            "action.execute",
            json!({"action": action, "params": params}),
            ctx,
        )
        .await
    }

    /// Run a legacy-style filter hook through every feature plugin.
    pub async fn filter_hook(
        &self,
        hook: &str,
        value: Value,
        args: Value,
        ctx: &PluginContext<'_>,
    ) -> Value {
        let mut current = value;
        for plugin in self.all() {
            let meta = plugin.meta();
            // Built-in plugins expose typed methods; hooks are for Lua plugins.
            if meta.kind != PluginKind::Feature || meta.builtin {
                continue;
            }
            if !db_enabled(ctx.db, &meta.code, false).await {
                continue;
            }
            if let Some(result) = self
                .call(
                    &plugin.meta().code,
                    "hook.filter",
                    json!({"hook": hook, "value": current, "args": args}),
                    ctx,
                )
                .await
            {
                if let Some(next) = result.get("value").cloned() {
                    current = next;
                }
            }
        }
        current
    }

    /// Let a plugin own node configuration generation (e.g. protocol plugins).
    pub async fn server_config(&self, params: Value, ctx: &PluginContext<'_>) -> Option<Value> {
        for plugin in self.all() {
            if plugin.meta().builtin {
                continue;
            }
            if let Some(result) = self
                .call(&plugin.meta().code, "server.config", params.clone(), ctx)
                .await
            {
                return result.get("config").cloned().or(Some(result));
            }
        }
        None
    }
}

fn output_code(haystack: &str) -> &'static str {
    if haystack.contains("singbox") || haystack.contains("sing-box") {
        "singbox"
    } else if haystack.contains("quantumult") {
        "quantumultx"
    } else if haystack.contains("loon") {
        "loon"
    } else if haystack.contains("surfboard") {
        "surfboard"
    } else if haystack.contains("surge") {
        "surge"
    } else if haystack.contains("stash") {
        "stash"
    } else if haystack.contains("shadowrocket") {
        "shadowrocket"
    } else if haystack.contains("clash") || haystack.contains("mihomo") {
        if haystack.contains("meta") || haystack.contains("mihomo") {
            "clash_meta"
        } else {
            "clash"
        }
    } else if haystack.contains("shadowsocks") || haystack.contains("ssr") {
        "shadowsocks"
    } else {
        "general"
    }
}

async fn db_enabled(db: &MySqlPool, code: &str, default: bool) -> bool {
    let row: Option<(i64,)> =
        sqlx::query_as("SELECT is_enabled FROM v2_plugins WHERE code = ? LIMIT 1")
            .bind(code)
            .fetch_optional(db)
            .await
            .ok()
            .flatten();
    match row {
        Some((enabled,)) => enabled != 0,
        None => default,
    }
}

/// Best-effort Telegram notification used by ticket/payment flows.
pub async fn notify_telegram(state: &crate::state::SharedState, method: &str, params: Value) {
    let enabled = state
        .plugin
        .is_enabled_or(&state.db, "telegram", false)
        .await;
    if !enabled {
        return;
    }
    let settings = state.settings().await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let config = state.plugin.config(&state.db, "telegram").await;
    let ctx = PluginContext::new(&state.db, &settings, &app_url, config);
    let _ = state.plugin.call("telegram", method, params, &ctx).await;
}

#[cfg(test)]
mod tests {
    use super::output_code;

    #[test]
    fn selects_output_by_client() {
        assert_eq!(output_code("clash-verge/1.0"), "clash");
        assert_eq!(output_code("clash.meta/1.0"), "clash_meta");
        assert_eq!(output_code("mihomo/1.0"), "clash_meta");
        assert_eq!(output_code("sing-box/1.10"), "singbox");
        assert_eq!(output_code("surge/5"), "surge");
        assert_eq!(output_code("surfboard/1.0"), "surfboard");
        assert_eq!(output_code("loon/3.0"), "loon");
        assert_eq!(output_code("quantumult%20x/1.0"), "quantumultx");
        assert_eq!(output_code("stash/2.0"), "stash");
        assert_eq!(output_code("shadowrocket/2.2"), "shadowrocket");
        assert_eq!(output_code("shadowsocks/1.0"), "shadowsocks");
        assert_eq!(output_code("v2rayng/1.0"), "general");
    }
}
