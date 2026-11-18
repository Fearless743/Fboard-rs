use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sha2::{Digest, Sha256};

pub fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

pub fn random_chars(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn random_hex(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut out = String::with_capacity(len);
    for _ in 0..len {
        out.push_str(&format!("{:x}", rng.gen_range(0..16u8)));
    }
    out
}

/// 32-char subscription token (Laravel `Helper::guid()`).
pub fn guid() -> String {
    random_hex(32)
}

pub fn uuid_v4() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Generate an order number compatible with the legacy format:
/// yyyymmddHHMMSS + 6 digits of microtime + 5 random digits.
pub fn generate_order_no() -> String {
    let now = chrono::Local::now();
    let micros = now.timestamp_subsec_micros();
    let mut rng = rand::thread_rng();
    let rand_part: u32 = rng.gen_range(10000..100000);
    format!("{}{:06}{}", now.format("%Y%m%d%H%M%S"), micros, rand_part)
}

pub fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn md5_hex(input: &str) -> String {
    format!("{:x}", md5::compute(input.as_bytes()))
}

pub fn base64_std(data: &str) -> String {
    base64::engine::general_purpose::STANDARD.encode(data.as_bytes())
}

pub fn base64_url_safe(data: &str) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data.as_bytes())
}

/// Verify a password against the formats supported by the legacy panel.
pub fn verify_password(algo: Option<&str>, salt: Option<&str>, password: &str, hash: &str) -> bool {
    match algo.unwrap_or("") {
        "md5" => md5_hex(password) == hash,
        "sha256" => sha256_hex(password) == hash,
        "md5salt" => md5_hex(&format!("{}{}", password, salt.unwrap_or(""))) == hash,
        "sha256salt" => sha256_hex(&format!("{}{}", password, salt.unwrap_or(""))) == hash,
        _ => bcrypt::verify(password, hash).unwrap_or(false),
    }
}

pub fn hash_password(password: &str) -> String {
    // bcrypt default cost matches PHP's PASSWORD_DEFAULT (10).
    bcrypt::hash(password, 10).unwrap_or_else(|_| String::new())
}

/// Human readable traffic, matching `Helper::trafficConvert`.
pub fn traffic_convert(bytes: f64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut value = bytes.max(0.0);
    let mut idx = 0;
    while value >= 1024.0 && idx < units.len() - 1 {
        value /= 1024.0;
        idx += 1;
    }
    if idx == 0 {
        format!("{:.0} {}", value, units[idx])
    } else {
        format!("{:.2} {}", value, units[idx])
    }
}

pub fn wrap_ipv6(addr: &str) -> String {
    if addr.parse::<std::net::Ipv6Addr>().is_ok() {
        format!("[{addr}]")
    } else {
        addr.to_string()
    }
}

pub fn yuan_to_cents(yuan: f64) -> i64 {
    (yuan * 100.0).round() as i64
}

pub fn sanitize_redirect(redirect: Option<&str>) -> String {
    let Some(redirect) = redirect else {
        return "dashboard".into();
    };
    let redirect = redirect.trim();
    if redirect.is_empty() {
        return "dashboard".into();
    }
    if redirect.starts_with('/') {
        if redirect.contains('\\') || redirect.starts_with("//") {
            return "dashboard".into();
        }
        return redirect.to_string();
    }
    let ok = redirect
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "_-./".contains(c));
    if ok && !redirect.contains('\\') {
        redirect.to_string()
    } else {
        "dashboard".into()
    }
}

pub fn validate_subscription_token(token: Option<&str>) -> Option<String> {
    let token = token?.trim();
    if token.len() == 32 && token.chars().all(|c| c.is_ascii_hexdigit()) {
        Some(token.to_ascii_lowercase())
    } else {
        None
    }
}

/// Extract the bearer token from an Authorization header.
pub fn bearer_token(header: Option<&str>) -> Option<String> {
    let header = header?.trim();
    let token = header
        .strip_prefix("Bearer ")
        .or_else(|| header.strip_prefix("bearer "))
        .unwrap_or(header)
        .trim();
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

/// Current rate with optional time ranges (`v2_server.rate_time_*`).
pub fn current_rate(rate: f64, enabled: bool, ranges: &serde_json::Value) -> f64 {
    if !enabled {
        return rate;
    }
    let now = chrono::Local::now().format("%H:%M").to_string();
    if let Some(arr) = ranges.as_array() {
        for range in arr {
            let start = range.get("start").and_then(|v| v.as_str()).unwrap_or("");
            let end = range.get("end").and_then(|v| v.as_str()).unwrap_or("");
            if !start.is_empty()
                && !end.is_empty()
                && now >= start.to_string()
                && now <= end.to_string()
            {
                return range.get("rate").and_then(|v| v.as_f64()).unwrap_or(rate);
            }
        }
    }
    rate
}

pub fn json_from_str<T: serde::de::DeserializeOwned + Default>(value: Option<&str>) -> T {
    value
        .and_then(|v| serde_json::from_str(v).ok())
        .unwrap_or_default()
}

pub fn json_value(value: Option<&str>) -> serde_json::Value {
    value
        .and_then(|v| serde_json::from_str(v).ok())
        .unwrap_or(serde_json::Value::Null)
}

/// SQL LIKE pattern escaping.
pub fn like_pattern(input: &str) -> String {
    format!(
        "%{}%",
        input
            .replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subscription_token_validation() {
        assert_eq!(
            validate_subscription_token(Some("0123456789ABCDEF0123456789ABCDEF")).as_deref(),
            Some("0123456789abcdef0123456789abcdef")
        );
        assert!(validate_subscription_token(Some("123")).is_none());
        assert!(validate_subscription_token(Some("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz")).is_none());
    }

    #[test]
    fn legacy_password_formats() {
        assert!(verify_password(Some("md5"), None, "secret", &md5_hex("secret")));
        assert!(verify_password(
            Some("md5salt"),
            Some("pepper"),
            "secret",
            &md5_hex("secretpepper")
        ));
        let hash = hash_password("secret123");
        assert!(verify_password(None, None, "secret123", &hash));
        assert!(!verify_password(None, None, "wrong", &hash));
    }

    #[test]
    fn redirect_sanitizer_blocks_open_redirects() {
        assert_eq!(sanitize_redirect(Some("/dashboard")), "/dashboard");
        assert_eq!(sanitize_redirect(Some("//evil.com")), "dashboard");
        assert_eq!(sanitize_redirect(Some("https://evil.com")), "dashboard");
    }
}
