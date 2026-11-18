use async_trait::async_trait;
use serde_json::{json, Value};

use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};

pub struct CoreProtocolsPlugin;

#[async_trait]
impl NativePlugin for CoreProtocolsPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "core_protocols".into(),
            name: "Core Protocols".into(),
            kind: PluginKind::Protocol,
            version: "1.0.0".into(),
            description: "内置核心协议字段定义".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: json!({}),
            readme: "# Core Protocols\n\n提供 Shadowsocks、VMess、VLESS、Trojan、Hysteria、TUIC 等协议的节点字段定义。".into(),
        }
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        match method {
            "protocol.definitions" => {
                let base = params
                    .get("value")
                    .cloned()
                    .unwrap_or_else(crate::api::admin::builtin_protocol_definitions);
                Ok(base)
            }
            other => anyhow::bail!("unsupported method: {other}"),
        }
    }
}
