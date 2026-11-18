use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::{json, Value};

use super::payment_util::{config_str, constant_time_eq, field, header, hmac_sha256_hex};
use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};

pub struct BtcpayPlugin;

fn config_fields() -> Value {
    json!({
        "btcpay_url": field("string", "API 接口所在网址", true, "包含最后的斜杠，例如 https://your-btcpay.com/", ""),
        "btcpay_storeId": field("string", "Store ID", true, "BTCPay 商店标识符", ""),
        "btcpay_api_key": field("string", "API KEY", true, "个人设置中的 API KEY", ""),
        "btcpay_webhook_key": field("string", "WEBHOOK KEY", true, "Webhook 通知密钥", ""),
    })
}

fn api_base(config: &Value) -> String {
    config_str(config, "btcpay_url")
        .trim_end_matches('/')
        .to_string()
}

async fn create_invoice(config: &Value, order: &Value) -> anyhow::Result<Value> {
    let base = api_base(config);
    let store = config_str(config, "btcpay_storeId");
    let api_key = config_str(config, "btcpay_api_key");
    if base.is_empty() || store.is_empty() || api_key.is_empty() {
        anyhow::bail!("BTCPay 配置不完整");
    }
    let trade_no = order
        .get("trade_no")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let amount = order
        .get("total_amount")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let url = format!("{base}/api/v1/stores/{store}/invoices");
    let response: Value = reqwest::Client::new()
        .post(url)
        .header("Authorization", format!("token {api_key}"))
        .json(&json!({
            "amount": format!("{:.2}", amount as f64 / 100.0),
            "currency": "CNY",
            "metadata": {"orderId": trade_no},
        }))
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await?
        .json()
        .await?;
    let link = response
        .get("checkoutLink")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("BTCPay 创建账单失败"))?;
    Ok(json!({"type": 1, "data": link}))
}

async fn verify_notify(
    config: &Value,
    raw_body: &str,
    headers: &Value,
) -> anyhow::Result<Value> {
    let key = config_str(config, "btcpay_webhook_key");
    if key.is_empty() {
        anyhow::bail!("BTCPay WEBHOOK KEY 未配置");
    }
    let signature = header(headers, "btcpay-sig");
    let expected = format!("sha256={}", hmac_sha256_hex(&key, raw_body.as_bytes()));
    if !constant_time_eq(signature, &expected) {
        anyhow::bail!("HMAC signature does not match");
    }
    let payload: Value = serde_json::from_str(raw_body)?;
    let invoice_id = payload
        .get("invoiceId")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    if invoice_id.is_empty() {
        anyhow::bail!("BTCPay payload missing invoiceId");
    }
    let base = api_base(config);
    let store = config_str(config, "btcpay_storeId");
    let api_key = config_str(config, "btcpay_api_key");
    let url = format!("{base}/api/v1/stores/{store}/invoices/{invoice_id}");
    let invoice: Value = reqwest::Client::new()
        .get(url)
        .header("Authorization", format!("token {api_key}"))
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await?
        .json()
        .await?;
    let status = invoice
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let allowed = ["Settled", "Complete", "complete", "settled", "paid", "Paid"];
    if !allowed.contains(&status) {
        anyhow::bail!("BTCPay invoice not settled: {status}");
    }
    let trade_no = invoice
        .pointer("/metadata/orderId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("BTCPay payload missing order reference"))?;
    let mut result = json!({
        "trade_no": trade_no,
        "callback_no": invoice_id,
    });
    if let Some(amount) = invoice.get("amount").and_then(|v| {
        v.as_f64()
            .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
    }) {
        result["paid_amount"] = json!((amount * 100.0).round() as i64);
    }
    Ok(result)
}

#[async_trait]
impl NativePlugin for BtcpayPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "btcpay".into(),
            name: "BTCPay".into(),
            kind: PluginKind::Payment,
            version: "1.0.0".into(),
            description: "BTCPay Server 加密货币支付".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: config_fields(),
            readme: "# BTCPay\n\n通过 BTCPay Server API 创建账单并校验 Webhook HMAC。".into(),
        }
    }

    fn payment_methods(&self) -> Vec<String> {
        vec!["BTCPay".into()]
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        match method {
            "payment.methods" => Ok(json!(["BTCPay"])),
            "payment.form" => Ok(config_fields()),
            "payment.pay" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let order = params.get("order").cloned().unwrap_or_else(|| json!({}));
                create_invoice(&config, &order).await
            }
            "payment.notify" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let raw_body = params
                    .get("raw_body")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let headers = params.get("headers").cloned().unwrap_or_else(|| json!({}));
                verify_notify(&config, raw_body, &headers).await
            }
            other => anyhow::bail!("unsupported method: {other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_webhook_signature() {
        let key = "secret";
        let body = "{\"invoiceId\":\"abc\"}";
        let signature = format!("sha256={}", hmac_sha256_hex(key, body.as_bytes()));
        let headers = json!({"btcpay-sig": signature});
        assert_eq!(super::header(&headers, "btcpay-sig"), format!("sha256={}", hmac_sha256_hex(key, body.as_bytes())));
    }
}
