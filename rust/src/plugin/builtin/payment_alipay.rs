use std::collections::HashMap;

use async_trait::async_trait;
use base64::Engine;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1v15::{Signature, SigningKey, VerifyingKey};
use rsa::pkcs8::DecodePrivateKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::sha2::Sha256;
use rsa::signature::{SignatureEncoding, Signer, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde_json::{json, Value};

use super::payment_util::{config_str, field};
use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};

pub struct AlipayF2fPlugin;

const GATEWAY: &str = "https://openapi.alipay.com/gateway.do";

fn config_fields() -> Value {
    json!({
        "app_id": field("string", "支付宝 APPID", true, "支付宝开放平台应用的 APPID", ""),
        "private_key": field("text", "支付宝私钥", true, "应用私钥，用于签名", ""),
        "public_key": field("text", "支付宝公钥", true, "支付宝公钥，用于验签", ""),
        "product_name": field("string", "自定义商品名称", false, "将会体现在支付宝账单中", ""),
    })
}

fn wrap_pem(value: &str, label: &str) -> String {
    if value.contains("BEGIN") {
        return value.trim().to_string();
    }
    let body: String = value.chars().filter(|c| !c.is_whitespace()).collect();
    let wrapped = body
        .as_bytes()
        .chunks(64)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect::<Vec<_>>()
        .join("\n");
    format!("-----BEGIN {label}-----\n{wrapped}\n-----END {label}-----")
}

fn private_key(raw: &str) -> anyhow::Result<RsaPrivateKey> {
    let raw = raw.trim();
    if raw.contains("BEGIN") {
        if raw.contains("BEGIN RSA PRIVATE KEY") {
            return Ok(RsaPrivateKey::from_pkcs1_pem(raw)?);
        }
        return Ok(RsaPrivateKey::from_pkcs8_pem(raw)?);
    }
    let pkcs8 = wrap_pem(raw, "PRIVATE KEY");
    if let Ok(key) = RsaPrivateKey::from_pkcs8_pem(&pkcs8) {
        return Ok(key);
    }
    let pkcs1 = wrap_pem(raw, "RSA PRIVATE KEY");
    Ok(RsaPrivateKey::from_pkcs1_pem(&pkcs1)?)
}

fn public_key(raw: &str) -> anyhow::Result<RsaPublicKey> {
    let pem = wrap_pem(raw, "PUBLIC KEY");
    Ok(RsaPublicKey::from_public_key_pem(&pem)?)
}

fn sign(data: &str, private: &str) -> anyhow::Result<String> {
    let key = private_key(private)?;
    let signing = SigningKey::<Sha256>::new(key);
    let signature = signing.sign(data.as_bytes());
    Ok(base64::engine::general_purpose::STANDARD.encode(signature.to_bytes()))
}

fn verify(data: &str, signature: &str, public: &str) -> bool {
    let Ok(key) = public_key(public) else {
        return false;
    };
    let verifying = VerifyingKey::<Sha256>::new(key);
    let signature = signature.replace(' ', "+");
    let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(signature) else {
        return false;
    };
    let Ok(signature) = Signature::try_from(bytes.as_slice()) else {
        return false;
    };
    verifying.verify(data.as_bytes(), &signature).is_ok()
}

fn sorted_query(params: &[(String, String)]) -> String {
    let mut params = params.to_vec();
    params.sort_by(|a, b| a.0.cmp(&b.0));
    params
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("&")
}

fn encoded_query(params: &[(String, String)]) -> String {
    params
        .iter()
        .map(|(key, value)| {
            format!(
                "{}={}",
                urlencoding::encode(key),
                urlencoding::encode(value)
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

async fn precreate(
    config: &Value,
    order: &Value,
    app_name: &str,
) -> anyhow::Result<Value> {
    let app_id = config_str(config, "app_id");
    let private = config_str(config, "private_key");
    let public = config_str(config, "public_key");
    if app_id.is_empty() || private.is_empty() || public.is_empty() {
        anyhow::bail!("支付宝配置不完整");
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
    let subject = {
        let configured = config_str(config, "product_name");
        if configured.is_empty() {
            format!("{app_name} - 订阅")
        } else {
            configured
        }
    };
    let biz_content = json!({
        "subject": subject,
        "out_trade_no": trade_no,
        "total_amount": format!("{:.2}", amount as f64 / 100.0),
    })
    .to_string();
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut params: Vec<(String, String)> = vec![
        ("app_id".into(), app_id),
        ("method".into(), "alipay.trade.precreate".into()),
        ("charset".into(), "UTF-8".into()),
        ("sign_type".into(), "RSA2".into()),
        ("timestamp".into(), timestamp),
        ("biz_content".into(), biz_content),
        ("version".into(), "1.0".into()),
        ("_input_charset".into(), "UTF-8".into()),
    ];
    if !notify_url.is_empty() {
        params.push(("notify_url".into(), notify_url.into()));
    }
    let to_sign = sorted_query(&params);
    let signature = sign(&to_sign, &private)?;
    params.push(("sign".into(), signature));
    let url = format!("{GATEWAY}?{}", encoded_query(&params));

    let response: Value = reqwest::Client::new()
        .get(url)
        .timeout(std::time::Duration::from_secs(20))
        .send()
        .await?
        .json()
        .await?;
    let body = response
        .get("alipay_trade_precreate_response")
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("支付宝请求失败"))?;
    if body.get("msg").and_then(|v| v.as_str()).unwrap_or("") != "Success" {
        anyhow::bail!(
            "{}",
            body.get("sub_msg")
                .and_then(|v| v.as_str())
                .unwrap_or("支付宝下单失败")
        );
    }
    let qr = body
        .get("qr_code")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("获取付款二维码失败"))?;
    Ok(json!({"type": 0, "data": qr}))
}

fn notify(
    config: &Value,
    params: &HashMap<String, Value>,
) -> Option<Value> {
    if params.get("trade_status")?.as_str()? != "TRADE_SUCCESS" {
        return None;
    }
    let public = config_str(config, "public_key");
    if public.is_empty() {
        return None;
    }
    let sign = params.get("sign")?.as_str()?.to_string();
    let mut entries: Vec<(String, String)> = params
        .iter()
        .filter(|(key, _)| key.as_str() != "sign" && key.as_str() != "sign_type")
        .filter_map(|(key, value)| value.as_str().map(|v| (key.clone(), v.to_string())))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let data = entries
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("&");
    if !verify(&data, &sign, &public) {
        return None;
    }
    let mut result = json!({
        "trade_no": params.get("out_trade_no")?.as_str()?,
        "callback_no": params.get("trade_no").and_then(|v| v.as_str()).unwrap_or(""),
    });
    if let Some(amount) = params
        .get("total_amount")
        .and_then(|v| v.as_str())
        .and_then(|v| v.parse::<f64>().ok())
    {
        result["paid_amount"] = json!((amount * 100.0).round() as i64);
    }
    Some(result)
}

#[async_trait]
impl NativePlugin for AlipayF2fPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "alipay_f2f".into(),
            name: "AlipayF2F".into(),
            kind: PluginKind::Payment,
            version: "1.0.0".into(),
            description: "支付宝当面付".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: config_fields(),
            readme: "# AlipayF2F\n\n使用支付宝开放平台当面付接口，返回二维码链接。".into(),
        }
    }

    fn payment_methods(&self) -> Vec<String> {
        vec!["AlipayF2F".into()]
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        match method {
            "payment.methods" => Ok(json!(["AlipayF2F"])),
            "payment.form" => Ok(config_fields()),
            "payment.pay" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let order = params.get("order").cloned().unwrap_or_else(|| json!({}));
                let app_name = ctx.settings.str("app_name", "Fboard");
                precreate(&config, &order, &app_name).await
            }
            "payment.notify" => {
                let config = params.get("config").cloned().unwrap_or_else(|| json!({}));
                let values: HashMap<String, Value> = params
                    .get("params")
                    .and_then(|v| v.as_object())
                    .map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                    .unwrap_or_default();
                notify(&config, &values)
                    .ok_or_else(|| anyhow::anyhow!("支付宝验签失败"))
            }
            other => anyhow::bail!("unsupported method: {other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_raw_pem() {
        let pem = wrap_pem("abc", "PUBLIC KEY");
        assert!(pem.starts_with("-----BEGIN PUBLIC KEY-----"));
        assert!(pem.ends_with("-----END PUBLIC KEY-----"));
    }

    #[test]
    fn signs_and_verifies_with_generated_keys() {
        use rsa::pkcs1::EncodeRsaPrivateKey;
        use rsa::pkcs8::{EncodePublicKey, LineEnding};
        let mut rng = rand::thread_rng();
        let private = RsaPrivateKey::new(&mut rng, 1024).unwrap();
        let public = RsaPublicKey::from(&private);
        let private_pem = private.to_pkcs1_pem(LineEnding::LF).unwrap().to_string();
        let public_pem = public.to_public_key_pem(LineEnding::LF).unwrap();
        let signature = sign("a=1&b=2", &private_pem).unwrap();
        assert!(verify("a=1&b=2", &signature, &public_pem));
        assert!(!verify("a=1&b=3", &signature, &public_pem));

        // Raw base64 key bodies (what the admin form usually contains).
        let private_raw = private_pem
            .lines()
            .filter(|line| !line.starts_with("-----"))
            .collect::<Vec<_>>()
            .join("");
        let public_raw = public_pem
            .lines()
            .filter(|line| !line.starts_with("-----"))
            .collect::<Vec<_>>()
            .join("");
        let signature = sign("x=1", &private_raw).unwrap();
        assert!(verify("x=1", &signature, &public_raw));
    }

    #[test]
    fn sorts_and_signs_query() {
        let params = vec![
            ("b".to_string(), "2".to_string()),
            ("a".to_string(), "1".to_string()),
        ];
        assert_eq!(sorted_query(&params), "a=1&b=2");
        assert_eq!(encoded_query(&params), "b=2&a=1");
    }
}
