use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::{json, Value};

use super::payment_util::{
    config_str, constant_time_eq, field, header, hmac_sha512_hex, form_query,
};
use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};

pub struct CoinPaymentsPlugin;

fn config_fields() -> Value {
    json!({
        "coinpayments_merchant_id": field("string", "Merchant ID", true, "商户 ID", ""),
        "coinpayments_ipn_secret": field("string", "IPN Secret", true, "通知密钥", ""),
        "coinpayments_currency": field("string", "货币代码", true, "建议与 Merchant Settings 保持一致", "CNY"),
    })
}

fn pay_url(config: &Value, order: &Value) -> anyhow::Result<Value> {
    let merchant = config_str(config, "coinpayments_merchant_id");
    let currency = config_str(config, "coinpayments_currency");
    if merchant.is_empty() || currency.is_empty() {
        anyhow::bail!("CoinPayments 配置不完整");
    }
    let trade_no = order
        .get("trade_no")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let amount = order
        .get("total_amount")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let return_url = order
        .get("return_url")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let notify_url = order
        .get("notify_url")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let success_url = reqwest::Url::parse(return_url)
        .map(|url| {
            let mut out = format!("{}://{}", url.scheme(), url.host_str().unwrap_or(""));
            if let Some(port) = url.port() {
                out.push_str(&format!(":{port}"));
            }
            out
        })
        .unwrap_or_default();
    let params = vec![
        ("cmd".to_string(), "_pay_simple".to_string()),
        ("reset".to_string(), "1".to_string()),
        ("merchant".to_string(), merchant),
        ("item_name".to_string(), trade_no.to_string()),
        ("item_number".to_string(), trade_no.to_string()),
        ("want_shipping".to_string(), "0".to_string()),
        ("currency".to_string(), currency),
        ("amountf".to_string(), format!("{:.2}", amount as f64 / 100.0)),
        ("success_url".to_string(), success_url),
        ("cancel_url".to_string(), return_url.to_string()),
        ("ipn_url".to_string(), notify_url.to_string()),
    ];
    let query = form_query(&params);
    Ok(json!({
        "type": 1,
        "data": format!("https://www.coinpayments.net/index.php?{query}")
    }))
}

fn verify_notify(
    config: &Value,
    params: &HashMap<String, Value>,
    headers: &Value,
) -> anyhow::Result<Value> {
    let merchant = config_str(config, "coinpayments_merchant_id");
    let secret = config_str(config, "coinpayments_ipn_secret");
    if params.get("merchant").and_then(|v| v.as_str()) != Some(merchant.as_str()) {
        anyhow::bail!("No or incorrect Merchant ID passed");
    }
    let mut entries: Vec<(String, String)> = params
        .iter()
        .filter_map(|(key, value)| value.as_str().map(|v| (key.clone(), v.to_string())))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let request = form_query(&entries).replace('\\', "");
    let signature = header(headers, "hmac");
    let expected = hmac_sha512_hex(&secret, request.as_bytes());
    if !constant_time_eq(signature, &expected) {
        anyhow::bail!("HMAC signature does not match");
    }
    let status = params
        .get("status")
        .and_then(|v| {
            v.as_i64().or_else(|| {
                v.as_str().and_then(|s| s.parse::<f64>().ok()).map(|f| f as i64)
            })
        })
        .unwrap_or(0);
    let trade_no = params
        .get("item_number")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let callback = params
        .get("txn_id")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    if status >= 100 || status == 2 {
        let mut result = json!({
            "trade_no": trade_no,
            "callback_no": callback,
            "custom_result": "IPN OK",
        });
        if let Some(amount) = params
            .get("amountf")
            .and_then(|v| v.as_str())
            .and_then(|v| v.parse::<f64>().ok())
        {
            result["paid_amount"] = json!((amount * 100.0).round() as i64);
        }
        Ok(result)
    } else if status < 0 {
        anyhow::bail!("Payment Timed Out or Error")
    } else {
        Ok(json!({"pending": true, "custom_result": "IPN OK: pending"}))
    }
}

#[async_trait]
impl NativePlugin for CoinPaymentsPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "coin_payments".into(),
            name: "CoinPayments".into(),
            kind: PluginKind::Payment,
            version: "1.0.0".into(),
            description: "CoinPayments 加密货币支付".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: config_fields(),
            readme: "# CoinPayments\n\n通过 CoinPayments 简单支付接口跳转，并使用 HMAC-SHA512 校验 IPN。".into(),
        }
    }

    fn payment_methods(&self) -> Vec<String> {
        vec!["CoinPayments".into()]
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        match method {
            "payment.methods" => Ok(json!(["CoinPayments"])),
            "payment.form" => Ok(config_fields()),
            "payment.pay" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let order = params.get("order").cloned().unwrap_or_else(|| json!({}));
                pay_url(&config, &order)
            }
            "payment.notify" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let values: HashMap<String, Value> = params
                    .get("params")
                    .and_then(|v| v.as_object())
                    .map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                    .unwrap_or_default();
                let headers = params.get("headers").cloned().unwrap_or_else(|| json!({}));
                verify_notify(&config, &values, &headers)
            }
            other => anyhow::bail!("unsupported method: {other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_ipn_hmac_and_status() {
        let config = json!({
            "coinpayments_merchant_id": "merchant",
            "coinpayments_ipn_secret": "secret"
        });
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("merchant".into(), json!("merchant"));
        params.insert("status".into(), json!("100"));
        params.insert("item_number".into(), json!("T1"));
        params.insert("txn_id".into(), json!("C1"));
        params.insert("amountf".into(), json!("10.00"));
        let mut entries: Vec<(String, String)> = params
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        let query = form_query(&entries).replace('\\', "");
        let sig = hmac_sha512_hex("secret", query.as_bytes());
        let headers = json!({"hmac": sig});
        let result = verify_notify(&config, &params, &headers).unwrap();
        assert_eq!(result["trade_no"], "T1");
        assert_eq!(result["callback_no"], "C1");
        assert_eq!(result["paid_amount"], 1000);

        params.insert("status".into(), json!("0"));
        let mut entries: Vec<(String, String)> = params
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        let sig = hmac_sha512_hex("secret", form_query(&entries).replace('\\', "").as_bytes());
        let result = verify_notify(&config, &params, &json!({"hmac": sig})).unwrap();
        assert_eq!(result["pending"], true);
    }
}
