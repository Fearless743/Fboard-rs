use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::{json, Value};

use super::payment_util::{config_str, constant_time_eq, field, header, hmac_sha256_hex};
use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};

pub struct CoinbasePlugin;

fn config_fields() -> Value {
    json!({
        "coinbase_url": field("string", "接口地址", true, "Coinbase Commerce API 地址", "https://api.commerce.coinbase.com/charges"),
        "coinbase_api_key": field("string", "API KEY", true, "Coinbase Commerce API 密钥", ""),
        "coinbase_webhook_key": field("string", "WEBHOOK KEY", true, "Webhook 签名验证密钥", ""),
    })
}

async fn create_charge(config: &Value, order: &Value) -> anyhow::Result<Value> {
    let url = config_str(config, "coinbase_url");
    let api_key = config_str(config, "coinbase_api_key");
    if url.is_empty() || api_key.is_empty() {
        anyhow::bail!("Coinbase 配置不完整");
    }
    let trade_no = order
        .get("trade_no")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let amount = order
        .get("total_amount")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let response: Value = reqwest::Client::new()
        .post(url)
        .header("X-CC-Api-Key", api_key)
        .header("X-CC-Version", "2018-03-22")
        .header("Accept", "application/json")
        .json(&json!({
            "name": "订阅套餐",
            "description": format!("订单号 {trade_no}"),
            "pricing_type": "fixed_price",
            "local_price": {
                "amount": format!("{:.2}", amount as f64 / 100.0),
                "currency": "CNY"
            },
            "metadata": {"outTradeNo": trade_no}
        }))
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await?
        .json()
        .await?;
    let link = response
        .pointer("/data/hosted_url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Coinbase 创建账单失败"))?;
    Ok(json!({"type": 1, "data": link}))
}

fn verify_notify(
    config: &Value,
    raw_body: &str,
    headers: &Value,
) -> anyhow::Result<Value> {
    let key = config_str(config, "coinbase_webhook_key");
    if key.is_empty() {
        anyhow::bail!("Coinbase WEBHOOK KEY 未配置");
    }
    let signature = header(headers, "x-cc-webhook-signature");
    let expected = hmac_sha256_hex(&key, raw_body.as_bytes());
    if !constant_time_eq(signature, &expected) {
        anyhow::bail!("HMAC signature does not match");
    }
    let payload: Value = serde_json::from_str(raw_body)?;
    let event_type = payload
        .pointer("/event/type")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let allowed = ["charge:confirmed", "charge:resolved", "charge:completed"];
    if !allowed.contains(&event_type) {
        anyhow::bail!("Unsupported Coinbase event type: {event_type}");
    }
    let trade_no = payload
        .pointer("/event/data/metadata/outTradeNo")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Coinbase payload missing trade reference"))?;
    let event_id = payload
        .pointer("/event/id")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let mut result = json!({
        "trade_no": trade_no,
        "callback_no": event_id,
    });
    if let Some(amount) = payload
        .pointer("/event/data/pricing/local/amount")
        .or_else(|| payload.pointer("/event/data/local_price/amount"))
        .and_then(|v| {
            v.as_f64()
                .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
        })
    {
        result["paid_amount"] = json!((amount * 100.0).round() as i64);
    }
    Ok(result)
}

#[async_trait]
impl NativePlugin for CoinbasePlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "coinbase".into(),
            name: "Coinbase".into(),
            kind: PluginKind::Payment,
            version: "1.0.0".into(),
            description: "Coinbase Commerce 加密货币支付".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: config_fields(),
            readme: "# Coinbase\n\n通过 Coinbase Commerce 创建 Charge，并校验 Webhook HMAC。".into(),
        }
    }

    fn payment_methods(&self) -> Vec<String> {
        vec!["Coinbase".into()]
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        match method {
            "payment.methods" => Ok(json!(["Coinbase"])),
            "payment.form" => Ok(config_fields()),
            "payment.pay" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let order = params.get("order").cloned().unwrap_or_else(|| json!({}));
                create_charge(&config, &order).await
            }
            "payment.notify" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let raw_body = params
                    .get("raw_body")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let headers = params.get("headers").cloned().unwrap_or_else(|| json!({}));
                verify_notify(&config, raw_body, &headers)
            }
            other => anyhow::bail!("unsupported method: {other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_webhook_and_extracts_order() {
        let config = json!({"coinbase_webhook_key": "secret"});
        let body = json!({
            "event": {
                "id": "evt_1",
                "type": "charge:confirmed",
                "data": {
                    "metadata": {"outTradeNo": "T1"},
                    "pricing": {"local": {"amount": "12.34"}}
                }
            }
        })
        .to_string();
        let sig = hmac_sha256_hex("secret", body.as_bytes());
        let headers = json!({"x-cc-webhook-signature": sig});
        let result = verify_notify(&config, &body, &headers).unwrap();
        assert_eq!(result["trade_no"], "T1");
        assert_eq!(result["callback_no"], "evt_1");
        assert_eq!(result["paid_amount"], 1234);
        let headers = json!({"x-cc-webhook-signature": "bad"});
        assert!(verify_notify(&config, &body, &headers).is_err());
    }
}
