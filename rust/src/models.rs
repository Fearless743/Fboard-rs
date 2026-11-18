use serde_json::{json, Value};
use sqlx::types::BigDecimal;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Default)]
pub struct UserRow {
    pub id: i64,
    pub invite_user_id: Option<i64>,
    pub telegram_id: Option<i64>,
    pub email: String,
    pub password: String,
    pub password_algo: Option<String>,
    pub password_salt: Option<String>,
    pub balance: i64,
    pub discount: Option<i64>,
    pub commission_type: i64,
    pub commission_rate: Option<i64>,
    pub commission_balance: i64,
    pub t: i64,
    pub u: i64,
    pub d: i64,
    pub transfer_enable: i64,
    pub banned: i64,
    pub is_admin: i64,
    pub last_login_at: Option<i64>,
    pub is_staff: i64,
    pub last_login_ip: Option<String>,
    pub register_ip: Option<String>,
    pub uuid: String,
    pub group_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub speed_limit: Option<i64>,
    pub remind_expire: Option<i64>,
    pub remind_traffic: Option<i64>,
    pub token: String,
    pub expired_at: Option<i64>,
    pub next_reset_at: Option<i64>,
    pub last_reset_at: Option<i64>,
    pub reset_count: i64,
    pub device_limit: Option<i64>,
    pub online_count: Option<i64>,
    pub last_online_at: Option<chrono::DateTime<chrono::Utc>>,
    pub remarks: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub usque_device_private: Option<String>,
    pub usque_device_public: Option<String>,
}

impl UserRow {
    pub fn public_json(&self) -> Value {
        json!({
            "id": self.id,
            "email": self.email,
            "transfer_enable": self.transfer_enable,
            "u": self.u,
            "d": self.d,
            "t": self.t,
            "balance": self.balance,
            "commission_balance": self.commission_balance,
            "plan_id": self.plan_id,
            "group_id": self.group_id,
            "speed_limit": self.speed_limit,
            "device_limit": self.device_limit,
            "expired_at": self.expired_at,
            "next_reset_at": self.next_reset_at,
            "last_reset_at": self.last_reset_at,
            "reset_count": self.reset_count,
            "banned": self.banned,
            "is_admin": self.is_admin,
            "is_staff": self.is_staff,
            "discount": self.discount,
            "commission_rate": self.commission_rate,
            "commission_type": self.commission_type,
            "invite_user_id": self.invite_user_id,
            "telegram_id": self.telegram_id,
            "remind_expire": self.remind_expire,
            "remind_traffic": self.remind_traffic,
            "uuid": self.uuid,
            "token": self.token,
            "created_at": self.created_at,
            "last_login_at": self.last_login_at,
            "last_login_ip": self.last_login_ip,
            "register_ip": self.register_ip,
            "online_count": self.online_count,
        })
    }

    pub fn is_available(&self) -> bool {
        self.banned == 0
            && self.transfer_enable > 0
            && (self.expired_at.is_none() || self.expired_at.unwrap_or(0) > crate::util::now())
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct PlanRow {
    pub id: i64,
    pub group_id: Option<u64>,
    pub transfer_enable: Option<u64>,
    pub name: String,
    pub prices: Option<Vec<u8>>,
    pub sell: i64,
    pub speed_limit: Option<u64>,
    pub device_limit: Option<u64>,
    pub show: i64,
    pub sort: Option<i64>,
    pub renew: i64,
    pub content: Option<String>,
    pub tags: Option<Vec<u8>>,
    pub reset_traffic_method: Option<i64>,
    pub capacity_limit: Option<u64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl PlanRow {
    pub fn prices_value(&self) -> Value {
        self.prices
            .as_deref()
            .and_then(|v| serde_json::from_slice(v).ok())
            .unwrap_or_else(|| json!({}))
    }

    pub fn tags_value(&self) -> Value {
        self.tags
            .as_deref()
            .and_then(|v| serde_json::from_slice(v).ok())
            .unwrap_or_else(|| json!([]))
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct OrderRow {
    pub id: i64,
    pub invite_user_id: Option<i64>,
    pub user_id: i64,
    pub plan_id: i64,
    pub coupon_id: Option<i64>,
    pub payment_id: Option<i64>,
    pub r#type: i64,
    pub period: String,
    pub trade_no: String,
    pub callback_no: Option<String>,
    pub total_amount: i64,
    pub handling_amount: Option<i64>,
    pub discount_amount: Option<i64>,
    pub surplus_amount: Option<i64>,
    pub surplus_credit: Option<i64>,
    pub balance_amount: Option<i64>,
    pub surplus_order_ids: Option<String>,
    pub status: i64,
    pub commission_status: i64,
    pub commission_balance: i64,
    pub actual_commission_balance: Option<i64>,
    pub paid_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct ServerRow {
    pub id: u64,
    pub r#type: String,
    pub code: Option<String>,
    pub parent_id: Option<u64>,
    pub machine_id: Option<u64>,
    pub group_ids: Option<Vec<u8>>,
    pub route_ids: Option<Vec<u8>>,
    pub name: String,
    pub rate: Option<BigDecimal>,
    pub transfer_enable: Option<i64>,
    pub u: i64,
    pub d: i64,
    pub rate_time_enable: i64,
    pub rate_time_ranges: Option<Vec<u8>>,
    pub tags: Option<Vec<u8>>,
    pub host: String,
    pub port: String,
    pub server_port: i64,
    pub protocol_settings: Option<Vec<u8>>,
    pub custom_outbounds: Option<Vec<u8>>,
    pub custom_routes: Option<Vec<u8>>,
    pub cert_config: Option<Vec<u8>>,
    pub show: i64,
    pub enabled: Option<i64>,
    pub sort: Option<u64>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ServerRow {
    pub fn group_ids(&self) -> Vec<i64> {
        self.group_ids
            .as_deref()
            .and_then(|v| serde_json::from_slice::<Vec<Value>>(v).ok())
            .map(|v| {
                v.into_iter()
                    .filter_map(|x| match x {
                        Value::Number(n) => n.as_i64(),
                        Value::String(s) => s.parse().ok(),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn route_ids(&self) -> Vec<i64> {
        self.route_ids
            .as_deref()
            .and_then(|v| serde_json::from_slice::<Vec<Value>>(v).ok())
            .map(|v| {
                v.into_iter()
                    .filter_map(|x| match x {
                        Value::Number(n) => n.as_i64(),
                        Value::String(s) => s.parse().ok(),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn settings(&self) -> Value {
        self.protocol_settings
            .as_deref()
            .and_then(|v| serde_json::from_slice(v).ok())
            .unwrap_or_else(|| json!({}))
    }

    pub fn tags_value(&self) -> Value {
        self.tags
            .as_deref()
            .and_then(|v| serde_json::from_slice(v).ok())
            .unwrap_or_else(|| json!([]))
    }

    pub fn created_ts(&self) -> i64 {
        self.created_at
            .map(|t| t.timestamp())
            .unwrap_or_else(crate::util::now)
    }

    pub fn current_rate(&self) -> f64 {
        let ranges = self
            .rate_time_ranges
            .as_deref()
            .and_then(|v| serde_json::from_slice(v).ok())
            .unwrap_or(Value::Null);
        crate::util::current_rate(
            self.rate
                .as_ref()
                .and_then(|v| v.to_string().parse::<f64>().ok())
                .unwrap_or(1.0),
            self.rate_time_enable != 0,
            &ranges,
        )
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct PaymentRow {
    pub id: i64,
    pub uuid: String,
    pub payment: String,
    pub name: String,
    pub icon: Option<String>,
    pub config: String,
    pub notify_domain: Option<String>,
    pub handling_fee_fixed: Option<i64>,
    pub handling_fee_percent: Option<BigDecimal>,
    pub enable: i64,
    pub sort: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl PaymentRow {
    pub fn config_value(&self) -> Value {
        serde_json::from_str(&self.config).unwrap_or_else(|_| json!({}))
    }

    pub fn handling_fee_percent_f64(&self) -> f64 {
        self.handling_fee_percent
            .as_ref()
            .and_then(|v| v.to_string().parse::<f64>().ok())
            .unwrap_or(0.0)
    }
}

impl ServerRow {
    /// JSON array string of permission group ids, as stored in the DB.
    pub fn group_ids_string(&self) -> String {
        self.group_ids
            .as_deref()
            .map(|v| String::from_utf8_lossy(v).to_string())
            .unwrap_or_else(|| "[]".into())
    }
}
