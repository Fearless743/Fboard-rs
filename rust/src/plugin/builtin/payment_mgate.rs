use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::{json, Value};

use super::payment_util::{config_str, constant_time_eq, field, form_query};
use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};

pub struct MgatePlugin;

fn config_fields() -> Value {
    json!({
        "mgate_url": field("string", "API 地址", true, "MGate 支付网关 API 地址", ""),
        "mgate_app_id": field("string", "APP ID", true, "MGate 应用标识符", ""),
        "mgate_app_secret": field("string", "App Secret", true, "MGate 应用密钥", ""),
        "mgate_source_currency": field("string", "源货币", false, "默认 CNY", "CNY"),
    })
}

async fn create_payment(config: &Value, order: &Value) -> anyhow::Result<Value> {
    let url = config_str(config, "mgate_url").trim_end_matches('/').to_string();
    let app_id = config_str(config, "mgate_app_id");
    let secret = config_str(config, "mgate_app_secret");
    if url.is_empty() || app_id.is_empty() || secret.is_empty() {
        anyhow::bail!("MGate 配置不完整");
    }
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
    let mut params = vec![
        ("out_trade_no".to_string(), trade_no.to_string()),
        ("total_amount".to_string(), amount.to_string()),
        ("notify_url".to_string(), notify_url.to_string()),
        ("return_url".to_string(), return_url.to_string()),
    ];
    let currency = config_str(config, "mgate_source_currency");
    if !currency.is_empty() {
        params.push(("source_currency".to_string(), currency));
    }
    params.push(("app_id".to_string(), app_id));
    params.sort_by(|a, b| a.0.cmp(&b.0));
    let sign = crate::util::md5_hex(&format!("{}{secret}", form_query(&params)));
    params.push(("sign".to_string(), sign));

    let response: Value = reqwest::Client::new()
        .post(format!("{url}/v1/gateway/fetch"))
        .header("User-Agent", "MGate")
        .form(&params)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await?
        .json()
        .await?;
    if let Some(errors) = response.get("errors") {
        if let Some((_, value)) = errors.as_object().and_then(|o| o.iter().next()) {
            let message = value
                .as_array()
                .and_then(|a| a.first())
                .and_then(|v| v.as_str())
                .unwrap_or("MGate 请求失败");
            anyhow::bail!("{message}");
        }
    }
    let pay_url = response
        .pointer("/data/pay_url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("MGate 接口请求失败"))?;
    Ok(json!({"type": 1, "data": pay_url}))
}

fn verify_notify(config: &Value, params: &HashMap<String, Value>) -> Option<Value> {
    let secret = config_str(config, "mgate_app_secret");
    let sign = params.get("sign")?.as_str()?.to_string();
    let mut entries: Vec<(String, String)> = params
        .iter()
        .filter(|(key, _)| key.as_str() != "sign")
        .filter_map(|(key, value)| value.as_str().map(|v| (key.clone(), v.to_string())))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let expected = crate::util::md5_hex(&format!("{}{secret}", form_query(&entries)));
    if !constant_time_eq(&sign, &expected) {
        return None;
    }
    let status = params
        .get("status")
        .or_else(|| params.get("trade_status"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if !status.is_empty() {
        let allowed = [
            "paid",
            "success",
            "succeeded",
            "completed",
            "trade_success",
            "1",
            "2",
        ];
        if !allowed.contains(&status.as_str()) {
            return None;
        }
    }
    let mut result = json!({
        "trade_no": params.get("out_trade_no")?.as_str()?,
        "callback_no": params.get("trade_no").and_then(|v| v.as_str()).unwrap_or(""),
    });
    if let Some(amount) = params
        .get("total_amount")
        .and_then(|v| v.as_str())
        .and_then(|v| v.parse::<i64>().ok())
    {
        result["paid_amount"] = json!(amount);
    }
    Some(result)
}

#[async_trait]
impl NativePlugin for MgatePlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "mgate".into(),
            name: "MGate".into(),
            kind: PluginKind::Payment,
            version: "1.0.0".into(),
            description: "MGate 聚合支付".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: config_fields(),
            readme: "# MGate\n\n通过 MGate `/v1/gateway/fetch` 创建支付并校验 MD5 签名。".into(),
        }
    }

    fn payment_methods(&self) -> Vec<String> {
        vec!["MGate".into()]
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        _ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        match method {
            "payment.methods" => Ok(json!(["MGate"])),
            "payment.form" => Ok(config_fields()),
            "payment.pay" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let order = params.get("order").cloned().unwrap_or_else(|| json!({}));
                create_payment(&config, &order).await
            }
            "payment.notify" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let values: HashMap<String, Value> = params
                    .get("params")
                    .and_then(|v| v.as_object())
                    .map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                    .unwrap_or_default();
                verify_notify(&config, &values)
                    .ok_or_else(|| anyhow::anyhow!("MGate 验签失败"))
            }
            other => anyhow::bail!("unsupported method: {other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_md5_signature() {
        let config = json!({"mgate_app_secret": "secret"});
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("out_trade_no".into(), json!("T1"));
        params.insert("trade_no".into(), json!("C1"));
        params.insert("total_amount".into(), json!("1000"));
        params.insert("status".into(), json!("paid"));
        let mut entries: Vec<(String, String)> = params
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        let sign = crate::util::md5_hex(&format!("{}{}", form_query(&entries), "secret"));
        params.insert("sign".into(), json!(sign));
        let result = verify_notify(&config, &params).unwrap();
        assert_eq!(result["trade_no"], "T1");
        assert_eq!(result["paid_amount"], 1000);
        params.insert("sign".into(), json!("bad"));
        assert!(verify_notify(&config, &params).is_none());
    }
}
