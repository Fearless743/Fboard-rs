use hmac::{Hmac, Mac};
use serde_json::{json, Value};
use sha2::{Sha256, Sha512};

pub fn config_str(config: &Value, key: &str) -> String {
    match config.get(key) {
        Some(Value::String(value)) => value.clone(),
        Some(Value::Number(value)) => value.to_string(),
        Some(Value::Bool(value)) => (if *value { "1" } else { "0" }).into(),
        _ => String::new(),
    }
}

pub fn field(kind: &str, label: &str, required: bool, description: &str, default: &str) -> Value {
    json!({
        "type": kind,
        "label": label,
        "required": required,
        "description": description,
        "value": default,
    })
}

pub fn header<'a>(headers: &'a Value, name: &str) -> &'a str {
    headers
        .get(name.to_ascii_lowercase())
        .and_then(|value| value.as_str())
        .unwrap_or("")
}

pub fn hmac_sha256_hex(key: &str, data: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("hmac key");
    mac.update(data);
    hex::encode(mac.finalize().into_bytes())
}

pub fn hmac_sha512_hex(key: &str, data: &[u8]) -> String {
    let mut mac = Hmac::<Sha512>::new_from_slice(key.as_bytes()).expect("hmac key");
    mac.update(data);
    hex::encode(mac.finalize().into_bytes())
}

/// Form-urlencoded encoding compatible with historical panel query strings.
pub fn form_encode(value: &str) -> String {
    urlencoding::encode(value).replace("%20", "+")
}

pub fn form_query(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .map(|(key, value)| format!("{}={}", form_encode(key), form_encode(value)))
        .collect::<Vec<_>>()
        .join("&")
}

pub fn constant_time_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() {
        return false;
    }
    let mut diff = 0u8;
    for (a, b) in left.iter().zip(right.iter()) {
        diff |= a ^ b;
    }
    diff == 0
}
