use std::collections::HashMap;

use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::util::now;

/// In-memory settings cache backed by `v2_settings` (Laravel `admin_setting()`).
#[derive(Debug, Clone, Default)]
pub struct Settings {
    values: HashMap<String, Value>,
}

impl Settings {
    pub async fn load(pool: &MySqlPool) -> Self {
        let rows: Vec<(String, Option<String>)> =
            sqlx::query_as("SELECT name, value FROM v2_settings")
                .fetch_all(pool)
                .await
                .unwrap_or_default();

        let mut values = HashMap::new();
        if let Value::Object(map) = defaults() {
            for (k, v) in map {
                values.insert(k, v);
            }
        }
        for (name, raw) in rows {
            let name = name.to_lowercase();
            let value = match raw {
                Some(raw) => match serde_json::from_str::<Value>(&raw) {
                    Ok(v) => v,
                    Err(_) => Value::String(raw),
                },
                None => Value::Null,
            };
            values.insert(name, value);
        }
        Self { values }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(&key.to_lowercase())
    }

    pub fn value(&self, key: &str) -> Value {
        self.get(key).cloned().unwrap_or(Value::Null)
    }

    pub fn str(&self, key: &str, default: &str) -> String {
        match self.get(key) {
            Some(Value::String(s)) => s.clone(),
            Some(Value::Number(n)) => n.to_string(),
            Some(Value::Bool(b)) => (if *b { "1" } else { "0" }).to_string(),
            Some(Value::Null) | None => default.to_string(),
            Some(other) => other.to_string(),
        }
    }

    pub fn i64(&self, key: &str, default: i64) -> i64 {
        match self.get(key) {
            Some(Value::Number(n)) => n.as_i64().unwrap_or(default),
            Some(Value::String(s)) => s.parse().unwrap_or(default),
            Some(Value::Bool(b)) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            _ => default,
        }
    }

    pub fn bool(&self, key: &str, default: bool) -> bool {
        match self.get(key) {
            Some(Value::Bool(b)) => *b,
            Some(Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
            Some(Value::String(s)) => {
                matches!(s.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on")
            }
            _ => default,
        }
    }

    pub fn f64(&self, key: &str, default: f64) -> f64 {
        match self.get(key) {
            Some(Value::Number(n)) => n.as_f64().unwrap_or(default),
            Some(Value::String(s)) => s.parse().unwrap_or(default),
            _ => default,
        }
    }

    pub fn arr(&self, key: &str) -> Vec<Value> {
        match self.get(key) {
            Some(Value::Array(a)) => a.clone(),
            Some(Value::String(s)) => serde_json::from_str(s).unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    /// The secure admin path. Mirrors the Laravel default:
    /// `admin_setting('secure_path', admin_setting('frontend_admin_path', crc32b(app_key)))`.
    pub fn secure_path(&self, app_key: &str) -> String {
        let secure = self.str("secure_path", "");
        if !secure.is_empty() && secure != "0" {
            return secure.trim_matches('/').to_string();
        }
        let frontend = self.str("frontend_admin_path", "");
        if !frontend.is_empty() && frontend != "0" {
            return frontend.trim_matches('/').to_string();
        }
        crc32b(app_key)
    }

    pub fn subscribe_path(&self) -> String {
        let p = self.str("subscribe_path", "s");
        let p = p.trim_matches('/');
        if p.is_empty() {
            "s".into()
        } else {
            p.to_string()
        }
    }

    pub fn public_map(&self) -> Value {
        let mut map = serde_json::Map::new();
        for (k, v) in &self.values {
            map.insert(k.clone(), v.clone());
        }
        Value::Object(map)
    }

    pub fn as_map(&self) -> &HashMap<String, Value> {
        &self.values
    }

    pub async fn save(pool: &MySqlPool, values: &Value) -> anyhow::Result<()> {
        let Some(obj) = values.as_object() else {
            return Ok(());
        };
        let ts = now();
        for (key, value) in obj {
            let key = key.to_lowercase();
            let stored = match value {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            // Insert or update by unique name. The legacy schema may not have a
            // unique index on `name`, so do an explicit lookup first.
            let existing: Option<(u64,)> =
                sqlx::query_as("SELECT id FROM v2_settings WHERE name = ? LIMIT 1")
                    .bind(&key)
                    .fetch_optional(pool)
                    .await?;
            if let Some((id,)) = existing {
                sqlx::query("UPDATE v2_settings SET value = ?, updated_at = FROM_UNIXTIME(?) WHERE id = ?")
                    .bind(stored)
                    .bind(ts)
                    .bind(id)
                    .execute(pool)
                    .await?;
            } else {
                sqlx::query(
                    "INSERT INTO v2_settings (name, value, created_at, updated_at) VALUES (?, ?, FROM_UNIXTIME(?), FROM_UNIXTIME(?))",
                )
                .bind(&key)
                .bind(stored)
                .bind(ts)
                .bind(ts)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }
}

fn crc32b(input: &str) -> String {
    // CRC32 (IEEE, same polynomial as PHP's crc32b).
    const TABLE: [u32; 256] = build_table();
    let mut crc: u32 = 0xFFFF_FFFF;
    for byte in input.as_bytes() {
        crc = TABLE[((crc ^ *byte as u32) & 0xFF) as usize] ^ (crc >> 8);
    }
    format!("{:x}", crc ^ 0xFFFF_FFFF)
}

const fn build_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            crc = if crc & 1 != 0 {
                0xEDB8_8320 ^ (crc >> 1)
            } else {
                crc >> 1
            };
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

/// Compatibility defaults used when a setting has never been written.
pub fn defaults() -> Value {
    json!({
        "app_name": "Fboard",
        "app_description": "",
        "app_url": "",
        "logo": "",
        "secure_path": "",
        "subscribe_path": "s",
        "subscribe_url": "",
        "email_verify": 0,
        "invite_force": 0,
        "email_whitelist_enable": 0,
        "captcha_enable": 0,
        "captcha_type": "recaptcha",
        "password_limit_enable": 1,
        "password_limit_count": 5,
        "password_limit_expire": 60,
        "register_limit_by_ip_enable": 0,
        "register_limit_count": 3,
        "register_limit_expire": 60,
        "plan_change_enable": 1,
        "surplus_enable": 1,
        "show_info_to_server_enable": 0,
        "reset_traffic_method": 1,
        "deposit_enable": 1,
        "deposit_min_amount": 100,
        "deposit_max_amount": 999999900,
        "currency": "CNY",
        "currency_symbol": "¥",
        "telegram_bot_enable": 0,
        "withdraw_close_enable": 0,
        "commission_distribution_enable": 0
    })
}
