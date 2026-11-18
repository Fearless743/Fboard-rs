use std::sync::Arc;

use dashmap::DashMap;

use crate::util::now;

/// Small in-process cache used for email codes / login throttling. Redis can be
/// added later without changing handlers; keys follow the legacy `CacheKey` names.
#[derive(Clone, Default)]
pub struct MemoryCache {
    inner: Arc<DashMap<String, (String, i64)>>,
}

impl MemoryCache {
    pub fn get(&self, key: &str) -> Option<String> {
        let entry = self.inner.get(key)?;
        if entry.1 != 0 && entry.1 < now() {
            drop(entry);
            self.inner.remove(key);
            return None;
        }
        Some(entry.0.clone())
    }

    pub fn put(&self, key: impl Into<String>, value: impl Into<String>, ttl_seconds: i64) {
        let expires = if ttl_seconds > 0 {
            now() + ttl_seconds
        } else {
            0
        };
        self.inner.insert(key.into(), (value.into(), expires));
    }

    pub fn forget(&self, key: &str) {
        self.inner.remove(key);
    }

    pub fn increment(&self, key: &str, ttl_seconds: i64) -> i64 {
        let current = self
            .get(key)
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(0)
            + 1;
        self.put(key, current.to_string(), ttl_seconds);
        current
    }

    pub fn remember(&self, key: &str, value: impl Into<String>, ttl_seconds: i64) -> String {
        let value = value.into();
        self.put(key, value.clone(), ttl_seconds);
        value
    }

    pub fn purge_expired(&self) {
        let now = now();
        self.inner.retain(|_, (_, exp)| *exp == 0 || *exp >= now);
    }
}

pub mod keys {
    pub fn password_error(email: &str) -> String {
        format!("PASSWORD_ERROR_LIMIT_{email}")
    }
    pub fn password_error_ip(ip: &str) -> String {
        format!("PASSWORD_ERROR_LIMIT_IP_{ip}")
    }
    pub fn email_verify_code(email: &str) -> String {
        format!("EMAIL_VERIFY_CODE_{email}")
    }
    pub fn forget_request_limit(email: &str) -> String {
        format!("FORGET_REQUEST_LIMIT_{email}")
    }
    pub fn register_ip_rate(ip: &str) -> String {
        format!("REGISTER_IP_RATE_LIMIT_{ip}")
    }
    pub fn server_online_user(node_type: &str, node_id: u64) -> String {
        format!("SERVER_{}_ONLINE_USER_{node_id}", node_type.to_uppercase())
    }
    pub fn server_last_push(node_type: &str, node_id: u64) -> String {
        format!("SERVER_{}_LAST_PUSH_AT_{node_id}", node_type.to_uppercase())
    }
    pub fn server_load_status(node_type: &str, node_id: u64) -> String {
        format!("SERVER_{}_LOAD_STATUS_{node_id}", node_type.to_uppercase())
    }
}
