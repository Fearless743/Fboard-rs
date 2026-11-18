use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::{json, Value};

use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};
use crate::util;

pub struct EpayPlugin;

fn config_fields() -> Value {
    json!({
        "url": {
            "type": "string",
            "label": "支付网关地址",
            "placeholder": "https://pay.example.com",
            "description": "请填写完整的支付网关地址，包括协议（http 或 https）",
            "required": true,
            "value": ""
        },
        "pid": {
            "type": "string",
            "label": "商户 ID",
            "placeholder": "",
            "description": "请填写商户 ID",
            "required": true,
            "value": ""
        },
        "key": {
            "type": "string",
            "label": "通信密钥",
            "placeholder": "",
            "description": "请填写通信密钥",
            "required": true,
            "value": ""
        },
        "type": {
            "type": "string",
            "label": "支付类型",
            "placeholder": "alipay",
            "description": "支付类型，如 alipay / wxpay / qqpay，可留空",
            "required": false,
            "value": ""
        }
    })
}

#[async_trait]
impl NativePlugin for EpayPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "epay".into(),
            name: "EPay".into(),
            kind: PluginKind::Payment,
            version: "1.0.0".into(),
            description: "EPay 聚合支付".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: config_fields(),
            readme: "# EPay\n\n配置支付网关地址、商户 ID 与通信密钥即可使用。".into(),
        }
    }

    fn payment_methods(&self) -> Vec<String> {
        vec!["EPay".into()]
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        match method {
            "payment.methods" => Ok(json!(["EPay"])),
            "payment.form" => Ok(config_fields()),
            "payment.pay" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let order = params.get("order").cloned().unwrap_or_else(|| json!({}));
                let trade_no = order
                    .get("trade_no")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let amount = order
                    .get("total_amount")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let notify_url = order
                    .get("notify_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let return_url = order
                    .get("return_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                pay(&config, trade_no, amount, notify_url, return_url)
                    .ok_or_else(|| anyhow::anyhow!("EPay 配置不完整"))
            }
            "payment.notify" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let values: HashMap<String, Value> = params
                    .get("params")
                    .and_then(|v| v.as_object())
                    .map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                    .unwrap_or_default();
                notify(&config, &values).ok_or_else(|| anyhow::anyhow!("EPay 验签失败"))
            }
            other => anyhow::bail!("unsupported method: {other}"),
        }
    }
}

fn pay(
    config: &Value,
    trade_no: &str,
    amount_cents: i64,
    notify_url: &str,
    return_url: &str,
) -> Option<Value> {
    let url = config.get("url").and_then(|v| v.as_str())?;
    let pid = config.get("pid").and_then(|v| v.as_str())?;
    let key = config.get("key").and_then(|v| v.as_str())?;
    let mut params: Vec<(String, String)> = vec![
        ("money".into(), format!("{:.2}", amount_cents as f64 / 100.0)),
        ("name".into(), trade_no.into()),
        ("notify_url".into(), notify_url.into()),
        ("return_url".into(), return_url.into()),
        ("out_trade_no".into(), trade_no.into()),
        ("pid".into(), pid.into()),
    ];
    if let Some(ty) = config
        .get("type")
        .and_then(|v| v.as_str())
        .filter(|v| !v.is_empty())
    {
        params.push(("type".into(), ty.into()));
    }
    params.sort_by(|a, b| a.0.cmp(&b.0));
    let query = params
        .iter()
        .map(|(k, v)| format!("{k}={}", urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let sign = util::md5_hex(&format!("{query}{key}"));
    Some(json!({
        "type": 1,
        "data": format!(
            "{}/submit.php?{query}&sign={sign}&sign_type=MD5",
            url.trim_end_matches('/')
        )
    }))
}

fn notify(config: &Value, params: &HashMap<String, Value>) -> Option<Value> {
    let key = config.get("key").and_then(|v| v.as_str())?;
    let sign = params.get("sign")?.as_str()?.to_string();
    let mut filtered: Vec<(String, String)> = params
        .iter()
        .filter(|(k, _)| k.as_str() != "sign" && k.as_str() != "sign_type")
        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
        .collect();
    filtered.sort_by(|a, b| a.0.cmp(&b.0));
    let query = filtered
        .iter()
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let expected = util::md5_hex(&format!("{query}{key}"));
    if expected != sign {
        return None;
    }
    if params.get("trade_status")?.as_str()? != "TRADE_SUCCESS" {
        return None;
    }
    Some(json!({
        "trade_no": params.get("out_trade_no")?.as_str()?,
        "callback_no": params.get("trade_no").and_then(|v| v.as_str()).unwrap_or(""),
        "paid_amount": params.get("money").and_then(|v| v.as_str()).and_then(|v| v.parse::<f64>().ok()).map(|v| (v * 100.0).round() as i64),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Value {
        json!({"url": "https://pay.example.com", "pid": "1001", "key": "secret"})
    }

    #[test]
    fn pay_builds_signed_url() {
        let result = pay(&config(), "T1", 1234, "https://panel/notify", "https://panel/return").unwrap();
        let url = result["data"].as_str().unwrap();
        assert!(url.starts_with("https://pay.example.com/submit.php?"));
        assert!(url.contains("money=12.34"));
        assert!(url.contains("sign_type=MD5"));
        assert!(url.contains("&sign="));
    }

    #[test]
    fn notify_verifies_signature() {
        let config = config();
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("pid".into(), json!("1001"));
        params.insert("out_trade_no".into(), json!("T1"));
        params.insert("trade_no".into(), json!("C1"));
        params.insert("trade_status".into(), json!("TRADE_SUCCESS"));
        params.insert("money".into(), json!("12.34"));
        let mut filtered: Vec<(String, String)> = params
            .iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
            .collect();
        filtered.sort_by(|a, b| a.0.cmp(&b.0));
        let query = filtered
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        params.insert("sign".into(), json!(util::md5_hex(&format!("{query}secret"))));
        let verified = notify(&config, &params).unwrap();
        assert_eq!(verified["trade_no"], "T1");
        assert_eq!(verified["callback_no"], "C1");
        assert_eq!(verified["paid_amount"], 1234);

        params.insert("sign".into(), json!("bad"));
        assert!(notify(&config, &params).is_none());
    }
}
