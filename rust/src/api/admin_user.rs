use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};

use crate::error::{ApiError, ApiResult};
use crate::models::{OrderRow, PaymentRow, PlanRow, UserRow};
use crate::plugin::PluginContext;
use crate::response::{ok, paginate, str_field};
use crate::state::{AdminUser, SharedState};
use crate::util;

// ---------------------------------------------------------------------------
// Users
// ---------------------------------------------------------------------------

pub async fn users_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let current = body.get("current").and_then(|v| v.as_i64()).unwrap_or(1).max(1);
    let page_size = body
        .get("pageSize")
        .and_then(|v| v.as_i64())
        .unwrap_or(10)
        .clamp(1, 200);

    let mut sql = String::from("SELECT * FROM v2_user WHERE 1 = 1");
    let mut binds: Vec<Value> = Vec::new();

    if let Some(filters) = body.get("filter").and_then(|v| v.as_array()) {
        for filter in filters {
            let field = filter.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let value = filter.get("value").cloned().unwrap_or(Value::Null);
            let allowed = [
                "id", "email", "plan_id", "group_id", "banned", "is_admin", "is_staff",
                "invite_user_id", "telegram_id", "commission_rate", "discount",
            ];
            if !allowed.contains(&field) {
                continue;
            }
            let logic = filter
                .get("logic")
                .and_then(|v| v.as_str())
                .unwrap_or("and")
                .to_lowercase();
            let op = if logic == "or" { "OR" } else { "AND" };
            // Admin filters use either a plain value or {operator, value}.
            let (operator, value) = if let Some(obj) = value.as_object() {
                let operator = obj
                    .get("operator")
                    .and_then(|v| v.as_str())
                    .unwrap_or("=");
                let value = obj.get("value").cloned().unwrap_or(Value::Null);
                (operator.to_string(), value)
            } else {
                ("=".to_string(), value)
            };
            let sql_op = match operator.as_str() {
                ">" => ">",
                ">=" => ">=",
                "<" => "<",
                "<=" => "<=",
                "!=" | "<>" => "!=",
                "like" | "contains" => "LIKE",
                _ => "=",
            };
            if field == "email" || field == "id" {
                let text = value
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| value.to_string());
                if sql_op == "LIKE" || (field == "email" && !text.chars().all(|c| c.is_ascii_digit())) {
                    sql.push_str(&format!(" {op} {field} LIKE ?"));
                    binds.push(json!(util::like_pattern(&text)));
                } else {
                    sql.push_str(&format!(" {op} {field} {sql_op} ?"));
                    binds.push(json!(text));
                }
            } else {
                sql.push_str(&format!(" {op} {field} {sql_op} ?"));
                binds.push(value);
            }
        }
    }

    if let Some(search) = body.get("search").and_then(|v| v.as_str()) {
        if !search.is_empty() {
            sql.push_str(" AND (email LIKE ? OR pinyin_index LIKE ?)");
            binds.push(json!(util::like_pattern(search)));
            binds.push(json!(util::like_pattern(search)));
        }
    }

    // Count with the same filters.
    let count_sql = sql.replace("SELECT *", "SELECT COUNT(*)");
    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    for value in &binds {
        count_query = bind_value_count(count_query, value);
    }
    let total = count_query.fetch_one(&state.db).await?.0;

    let mut order_by = String::from(" ORDER BY id DESC");
    if let Some(sorts) = body.get("sort").and_then(|v| v.as_array()) {
        let mut parts = Vec::new();
        for sort in sorts {
            let field = sort.get("id").and_then(|v| v.as_str()).unwrap_or("");
            if !field.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') || field.is_empty() {
                continue;
            }
            let desc = sort.get("desc").and_then(|v| v.as_bool()).unwrap_or(true);
            parts.push(format!("{field} {}", if desc { "DESC" } else { "ASC" }));
        }
        if !parts.is_empty() {
            order_by = format!(" ORDER BY {}", parts.join(", "));
        }
    }
    sql.push_str(&order_by);
    sql.push_str(" LIMIT ? OFFSET ?");
    binds.push(json!(page_size));
    binds.push(json!((current - 1) * page_size));

    let mut query = sqlx::query_as::<_, UserRow>(&sql);
    for value in &binds {
        query = bind_value_user(query, value);
    }
    let rows = query.fetch_all(&state.db).await?;

    let mut data = Vec::new();
    for user in rows {
        data.push(user_admin_json(&state, &user).await?);
    }
    Ok(paginate(data, total, current, page_size))
}

async fn user_admin_json(state: &SharedState, user: &UserRow) -> ApiResult<Value> {
    let plan: Option<(i64, String)> = match user.plan_id {
        Some(id) => sqlx::query_as("SELECT id, name FROM v2_plan WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?,
        None => None,
    };
    let group: Option<(i64, String)> = match user.group_id {
        Some(id) => sqlx::query_as("SELECT id, name FROM v2_server_group WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?,
        None => None,
    };
    let invite: Option<(i64, String)> = match user.invite_user_id {
        Some(id) => sqlx::query_as("SELECT id, email FROM v2_user WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?,
        None => None,
    };
    let mut data = user.public_json();
    if let Some(obj) = data.as_object_mut() {
        obj.insert("balance".into(), json!(user.balance as f64 / 100.0));
        obj.insert(
            "commission_balance".into(),
            json!(user.commission_balance as f64 / 100.0),
        );
        obj.insert("total_used".into(), json!(user.u + user.d));
        obj.insert(
            "subscribe_url".into(),
            json!(crate::api::subscribe::subscribe_url(state, user).await),
        );
        obj.insert(
            "plan".into(),
            plan.map(|(id, name)| json!({"id": id, "name": name}))
                .unwrap_or(Value::Null),
        );
        obj.insert(
            "group".into(),
            group
                .map(|(id, name)| json!({"id": id, "name": name}))
                .unwrap_or(Value::Null),
        );
        obj.insert(
            "invite_user".into(),
            invite
                .map(|(id, email)| json!({"id": id, "email": email}))
                .unwrap_or(Value::Null),
        );
        obj.insert("remarks".into(), json!(user.remarks));
        obj.insert("remark".into(), json!(user.remarks));
        obj.insert("is_admin".into(), json!(user.is_admin != 0));
        obj.insert("is_staff".into(), json!(user.is_staff != 0));
        obj.insert("banned".into(), json!(user.banned != 0));
        obj.insert("speed_limit".into(), json!(user.speed_limit));
        obj.insert("device_limit".into(), json!(user.device_limit));
        obj.insert("expired_at".into(), json!(user.expired_at));
    }
    Ok(data)
}

pub async fn user_info_by_id(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let user: Option<UserRow> = sqlx::query_as("SELECT * FROM v2_user WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    let user = user.ok_or_else(|| ApiError::not_found("用户不存在"))?;
    Ok(ok(user_admin_json(&state, &user).await?))
}

pub async fn user_update(
    State(state): State<SharedState>,
    AdminUser(admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body
        .get("id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| ApiError::bad_request("用户不存在"))?;
    let user: Option<UserRow> = sqlx::query_as("SELECT * FROM v2_user WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    let user = user.ok_or_else(|| ApiError::not_found("用户不存在"))?;

    if admin.id != 1 {
        for flag in ["is_admin", "is_staff"] {
            if let Some(value) = body.get(flag) {
                let submitted = crate::api::admin::json_bool(value);
                let current = if flag == "is_admin" {
                    user.is_admin != 0
                } else {
                    user.is_staff != 0
                };
                if submitted != current {
                    return Err(ApiError::forbidden("无权限修改管理员/员工标识"));
                }
            }
        }
    }

    let mut sets: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    if let Some(email) = str_field(&body, "email") {
        let exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM v2_user WHERE email = ? LIMIT 1")
            .bind(&email)
            .fetch_optional(&state.db)
            .await?;
        if let Some((other,)) = exists {
            if other != id {
                return Err(ApiError::bad_request("邮箱已被使用"));
            }
        }
        sets.push("email = ?".into());
        values.push(json!(email));
    }
    if let Some(password) = str_field(&body, "password") {
        if password.len() >= 8 {
            sets.push("password = ?".into());
            values.push(json!(util::hash_password(&password)));
            sets.push("password_algo = NULL".into());
            sets.push("password_salt = NULL".into());
        }
    }
    for (field, key) in [
        ("transfer_enable", "transfer_enable"),
        ("u", "u"),
        ("d", "d"),
        ("plan_id", "plan_id"),
        ("group_id", "group_id"),
        ("commission_type", "commission_type"),
        ("commission_rate", "commission_rate"),
        ("discount", "discount"),
        ("speed_limit", "speed_limit"),
        ("device_limit", "device_limit"),
        ("invite_user_id", "invite_user_id"),
        ("expired_at", "expired_at"),
    ] {
        if let Some(value) = body.get(key) {
            if value.is_null() {
                sets.push(format!("{field} = NULL"));
            } else if let Some(n) = value.as_i64() {
                sets.push(format!("{field} = ?"));
                values.push(json!(n));
            }
        }
    }
    if let Some(value) = body.get("balance").and_then(|v| v.as_f64()) {
        sets.push("balance = ?".into());
        values.push(json!(util::yuan_to_cents(value)));
    }
    if let Some(value) = body.get("commission_balance").and_then(|v| v.as_f64()) {
        sets.push("commission_balance = ?".into());
        values.push(json!(util::yuan_to_cents(value)));
    }
    for field in ["banned", "is_admin", "is_staff", "remind_expire", "remind_traffic"] {
        if let Some(value) = body.get(field) {
            sets.push(format!("{field} = ?"));
            values.push(json!(crate::api::admin::json_bool(value) as i64));
        }
    }
    if let Some(remarks) = body.get("remarks").or_else(|| body.get("remark")) {
        sets.push("remarks = ?".into());
        values.push(remarks.clone());
    }
    if let Some(plan_id) = body.get("plan_id").and_then(|v| v.as_i64()) {
        let plan_group: Option<(Option<i64>,)> =
            sqlx::query_as("SELECT group_id FROM v2_plan WHERE id = ?")
                .bind(plan_id)
                .fetch_optional(&state.db)
                .await?;
        if let Some((group_id,)) = plan_group {
            sets.push("group_id = ?".into());
            values.push(json!(group_id));
        }
    }

    if sets.is_empty() {
        return Ok(ok(json!(true)));
    }
    sets.push("updated_at = ?".into());
    values.push(json!(util::now()));
    let sql = format!("UPDATE v2_user SET {} WHERE id = ?", sets.join(", "));
    let mut query = sqlx::query(&sql);
    for value in &values {
        query = bind_value(query, value);
    }
    query.bind(id).execute(&state.db).await?;

    if crate::api::admin::json_bool(body.get("banned").unwrap_or(&Value::Bool(false))) {
        sqlx::query("DELETE FROM personal_access_tokens WHERE tokenable_id = ? AND tokenable_type LIKE '%User'")
            .bind(id)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn user_generate(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let prefix = str_field(&body, "email_prefix").unwrap_or_default();
    let suffix = str_field(&body, "email_suffix").unwrap_or_default();
    let password = str_field(&body, "password").unwrap_or_else(|| util::random_chars(12));
    let count = body
        .get("generate_count")
        .and_then(|v| v.as_i64())
        .unwrap_or(1)
        .clamp(1, 500);
    let plan_id = body.get("plan_id").and_then(|v| v.as_i64());
    let expired_at = body.get("expired_at").and_then(|v| {
        v.as_i64().or_else(|| {
            v.as_str()
                .and_then(|s| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok())
                .map(|d| d.and_utc().timestamp())
        })
    });
    let mut created = Vec::new();
    for _ in 0..count {
        let email = if suffix.is_empty() {
            format!("{}{}", prefix, util::random_chars(8))
        } else {
            format!("{}{}@{}", prefix, util::random_chars(8), suffix)
        };
        let now = util::now();
        let result = sqlx::query(
            "INSERT INTO v2_user (email, password, uuid, token, remind_expire, remind_traffic, balance, commission_balance, t, u, d, transfer_enable, banned, is_admin, plan_id, expired_at, created_at, updated_at) VALUES (?, ?, ?, ?, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, ?, ?, ?, ?)",
        )
        .bind(&email)
        .bind(util::hash_password(&password))
        .bind(util::uuid_v4())
        .bind(util::guid())
        .bind(plan_id)
        .bind(expired_at)
        .bind(now)
        .bind(now)
        .execute(&state.db)
        .await?;
        if let Some(plan_id) = plan_id {
            if let Some(plan) = sqlx::query_as::<_, PlanRow>("SELECT * FROM v2_plan WHERE id = ?")
                .bind(plan_id)
                .fetch_optional(&state.db)
                .await?
            {
                sqlx::query("UPDATE v2_user SET group_id = ?, transfer_enable = ? WHERE id = ?")
                    .bind(plan.group_id)
                    .bind(plan.transfer_enable.unwrap_or(0))
                    .bind(result.last_insert_id() as i64)
                    .execute(&state.db)
                    .await?;
            }
        }
        created.push(json!({"email": email, "password": password}));
    }
    Ok(ok(Value::Array(created)))
}

pub async fn user_ban(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let banned = crate::api::admin::json_bool(body.get("banned").unwrap_or(&Value::Bool(true))) as i64;
    let mut ids: Vec<i64> = Vec::new();
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()) {
        ids.push(id);
    }
    if let Some(filters) = body.get("filter").and_then(|v| v.as_array()) {
        for filter in filters {
            if filter.get("id").and_then(|v| v.as_str()) == Some("id") {
                if let Some(id) = filter.get("value").and_then(|v| v.as_i64()) {
                    ids.push(id);
                }
            }
        }
    }
    for id in ids {
        sqlx::query("UPDATE v2_user SET banned = ?, updated_at = ? WHERE id = ?")
            .bind(banned)
            .bind(util::now())
            .bind(id)
            .execute(&state.db)
            .await?;
        if banned == 1 {
            sqlx::query("DELETE FROM personal_access_tokens WHERE tokenable_id = ? AND tokenable_type LIKE '%User'")
                .bind(id)
                .execute(&state.db)
                .await?;
        }
    }
    Ok(ok(json!(true)))
}

pub async fn user_reset_secret(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("UPDATE v2_user SET token = ?, uuid = ?, updated_at = ? WHERE id = ?")
        .bind(util::guid())
        .bind(util::uuid_v4())
        .bind(util::now())
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn user_destroy(
    State(state): State<SharedState>,
    AdminUser(admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    if id == 1 || id == admin.id {
        return Err(ApiError::bad_request("不能删除超级管理员"));
    }
    sqlx::query("DELETE FROM personal_access_tokens WHERE tokenable_id = ? AND tokenable_type LIKE '%User'")
        .bind(id)
        .execute(&state.db)
        .await?;
    sqlx::query("DELETE FROM v2_user WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn user_set_invite(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    user_update(State(state), AdminUser::empty(), Json(body)).await
}

pub async fn user_invite_list(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let rows: Vec<(i64, String, i64)> =
        sqlx::query_as("SELECT id, email, created_at FROM v2_user WHERE invite_user_id = ? ORDER BY id DESC")
            .bind(id)
            .fetch_all(&state.db)
            .await?;
    Ok(ok(Value::Array(
        rows.into_iter()
            .map(|(id, email, created)| json!({"id": id, "email": email, "created_at": created}))
            .collect(),
    )))
}

pub async fn user_login_logs(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let user_id = query.get("user_id").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(20).max(1);
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_user_login_log WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await?;
    let rows: Vec<(i64, String, Option<String>, String, i64)> = sqlx::query_as(
        "SELECT id, ip, user_agent, method, created_at FROM v2_user_login_log WHERE user_id = ? ORDER BY id DESC LIMIT ? OFFSET ?",
    )
    .bind(user_id)
    .bind(page_size)
    .bind((current - 1) * page_size)
    .fetch_all(&state.db)
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, ip, ua, method, created)| {
            json!({"id": id, "ip": ip, "user_agent": ua, "method": method, "created_at": created})
        })
        .collect();
    Ok(paginate(data, total.0, current, page_size))
}

pub async fn user_dump_csv(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let rows: Vec<UserRow> = sqlx::query_as("SELECT * FROM v2_user ORDER BY id ASC")
        .fetch_all(&state.db)
        .await?;
    let mut csv = String::from("\u{feff}email,balance,commission_balance,transfer_enable,u,d,expired_at,token,plan_id\n");
    for user in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            user.email,
            user.balance as f64 / 100.0,
            user.commission_balance as f64 / 100.0,
            user.transfer_enable,
            user.u,
            user.d,
            user.expired_at.unwrap_or(0),
            user.token,
            user.plan_id.unwrap_or(0)
        ));
    }
    Ok((axum::http::StatusCode::OK, [("content-type", "text/csv; charset=utf-8")], csv).into_response())
}

// Bound helpers for dynamic SQL.
fn bind_value<'a>(
    query: sqlx::query::Query<'a, sqlx::MySql, sqlx::mysql::MySqlArguments>,
    value: &Value,
) -> sqlx::query::Query<'a, sqlx::MySql, sqlx::mysql::MySqlArguments> {
    match value {
        Value::String(s) => query.bind(s.clone()),
        Value::Number(n) if n.is_f64() => query.bind(n.as_f64().unwrap_or(0.0)),
        Value::Number(n) => query.bind(n.as_i64().unwrap_or(0)),
        Value::Bool(b) => query.bind(*b as i64),
        Value::Null => query.bind(Option::<String>::None),
        other => query.bind(other.to_string()),
    }
}

fn bind_value_user<'a>(
    query: sqlx::query::QueryAs<'a, sqlx::MySql, UserRow, sqlx::mysql::MySqlArguments>,
    value: &Value,
) -> sqlx::query::QueryAs<'a, sqlx::MySql, UserRow, sqlx::mysql::MySqlArguments> {
    match value {
        Value::String(s) => query.bind(s.clone()),
        Value::Number(n) if n.is_f64() => query.bind(n.as_f64().unwrap_or(0.0)),
        Value::Number(n) => query.bind(n.as_i64().unwrap_or(0)),
        Value::Bool(b) => query.bind(*b as i64),
        Value::Null => query.bind(Option::<String>::None),
        other => query.bind(other.to_string()),
    }
}

// ---------------------------------------------------------------------------
// Orders
// ---------------------------------------------------------------------------

pub async fn orders_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(10).max(1);
    let mut sql = String::from("SELECT * FROM v2_order WHERE 1 = 1");
    let mut binds: Vec<Value> = Vec::new();
    for (key, value) in &query {
        match key.as_str() {
            "status" | "user_id" | "plan_id" | "type" => {
                if let Ok(n) = value.parse::<i64>() {
                    sql.push_str(&format!(" AND {key} = ?"));
                    binds.push(json!(n));
                }
            }
            "trade_no" | "email" if !value.is_empty() => {
                if key == "trade_no" {
                    sql.push_str(" AND trade_no LIKE ?");
                } else {
                    sql.push_str(" AND user_id IN (SELECT id FROM v2_user WHERE email LIKE ?)");
                }
                binds.push(json!(util::like_pattern(value)));
            }
            _ => {}
        }
    }
    let count_sql = sql.replace("SELECT *", "SELECT COUNT(*)");
    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    for value in &binds {
        count_query = bind_value_count(count_query, value);
    }
    let total = count_query.fetch_one(&state.db).await?.0;

    sql.push_str(" ORDER BY created_at DESC, id DESC LIMIT ? OFFSET ?");
    binds.push(json!(page_size));
    binds.push(json!((current - 1) * page_size));
    let mut query = sqlx::query_as::<_, OrderRow>(&sql);
    for value in &binds {
        query = bind_value_order(query, value);
    }
    let rows = query.fetch_all(&state.db).await?;
    let mut data = Vec::new();
    for order in rows {
        data.push(order_admin_json(&state, &order).await?);
    }
    Ok(paginate(data, total, current, page_size))
}

async fn order_admin_json(state: &SharedState, order: &OrderRow) -> ApiResult<Value> {
    let user: Option<(i64, String)> = sqlx::query_as("SELECT id, email FROM v2_user WHERE id = ?")
        .bind(order.user_id)
        .fetch_optional(&state.db)
        .await?;
    let plan: Option<(i64, String)> = sqlx::query_as("SELECT id, name FROM v2_plan WHERE id = ?")
        .bind(order.plan_id)
        .fetch_optional(&state.db)
        .await?;
    let payment: Option<(i64, String)> = match order.payment_id {
        Some(id) => sqlx::query_as("SELECT id, name FROM v2_payment WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?,
        None => None,
    };
    Ok(json!({
        "id": order.id,
        "invite_user_id": order.invite_user_id,
        "user_id": order.user_id,
        "plan_id": order.plan_id,
        "coupon_id": order.coupon_id,
        "payment_id": order.payment_id,
        "type": order.r#type,
        "period": order.period,
        "trade_no": order.trade_no,
        "callback_no": order.callback_no,
        "total_amount": order.total_amount,
        "handling_amount": order.handling_amount,
        "discount_amount": order.discount_amount,
        "surplus_amount": order.surplus_amount,
        "balance_amount": order.balance_amount,
        "status": order.status,
        "commission_status": order.commission_status,
        "commission_balance": order.commission_balance,
        "paid_at": order.paid_at,
        "created_at": order.created_at,
        "updated_at": order.updated_at,
        "user": user.map(|(id, email)| json!({"id": id, "email": email})).unwrap_or(Value::Null),
        "plan": plan.map(|(id, name)| json!({"id": id, "name": name})).unwrap_or(Value::Null),
        "payment": payment.map(|(id, name)| json!({"id": id, "name": name})).unwrap_or(Value::Null),
    }))
}

fn bind_value_order<'a>(
    query: sqlx::query::QueryAs<'a, sqlx::MySql, OrderRow, sqlx::mysql::MySqlArguments>,
    value: &Value,
) -> sqlx::query::QueryAs<'a, sqlx::MySql, OrderRow, sqlx::mysql::MySqlArguments> {
    match value {
        Value::String(s) => query.bind(s.clone()),
        Value::Number(n) if n.is_f64() => query.bind(n.as_f64().unwrap_or(0.0)),
        Value::Number(n) => query.bind(n.as_i64().unwrap_or(0)),
        Value::Bool(b) => query.bind(*b as i64),
        Value::Null => query.bind(Option::<String>::None),
        other => query.bind(other.to_string()),
    }
}

pub async fn order_update(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let trade_no = str_field(&body, "trade_no").or_else(|| str_field(&body, "id"));
    let Some(trade_no) = trade_no else {
        return Err(ApiError::bad_request("订单不存在"));
    };
    let order: Option<OrderRow> =
        sqlx::query_as("SELECT * FROM v2_order WHERE trade_no = ? OR id = ? LIMIT 1")
            .bind(&trade_no)
            .bind(trade_no.parse::<i64>().unwrap_or(0))
            .fetch_optional(&state.db)
            .await?;
    let order = order.ok_or_else(|| ApiError::not_found("订单不存在"))?;
    let mut sets = Vec::new();
    let mut values: Vec<Value> = Vec::new();
    for field in ["total_amount", "handling_amount", "discount_amount", "status", "commission_status", "commission_balance"] {
        if let Some(value) = body.get(field).and_then(|v| v.as_i64()) {
            sets.push(format!("{field} = ?"));
            values.push(json!(value));
        }
    }
    if let Some(callback) = str_field(&body, "callback_no") {
        sets.push("callback_no = ?".into());
        values.push(json!(callback));
    }
    if sets.is_empty() {
        return Ok(ok(json!(true)));
    }
    sets.push("updated_at = ?".into());
    values.push(json!(util::now()));
    let sql = format!("UPDATE v2_order SET {} WHERE id = ?", sets.join(", "));
    let mut query = sqlx::query(&sql);
    for value in &values {
        query = bind_value(query, value);
    }
    query.bind(order.id).execute(&state.db).await?;
    Ok(ok(json!(true)))
}

pub async fn order_paid(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let trade_no = str_field(&body, "trade_no").ok_or_else(|| ApiError::bad_request("订单不存在"))?;
    let callback = str_field(&body, "callback_no").unwrap_or_else(|| trade_no.clone());
    let ok_paid = crate::api::order::mark_order_paid(&state, &trade_no, &callback).await?;
    Ok(ok(json!(ok_paid)))
}

pub async fn order_cancel(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let trade_no = str_field(&body, "trade_no").ok_or_else(|| ApiError::bad_request("订单不存在"))?;
    sqlx::query("UPDATE v2_order SET status = 2, updated_at = ? WHERE trade_no = ? AND status IN (0,1)")
        .bind(util::now())
        .bind(trade_no)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn order_detail(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body
        .get("id")
        .and_then(|v| v.as_i64())
        .or_else(|| str_field(&body, "id").and_then(|v| v.parse::<i64>().ok()))
        .or_else(|| str_field(&body, "trade_no").and_then(|v| v.parse::<i64>().ok()))
        .unwrap_or(0);
    let trade_no = str_field(&body, "trade_no")
        .or_else(|| str_field(&body, "id"))
        .unwrap_or_default();
    let order: Option<OrderRow> = sqlx::query_as(
        "SELECT * FROM v2_order WHERE id = ? OR trade_no = ? ORDER BY id DESC LIMIT 1",
    )
    .bind(id)
    .bind(&trade_no)
    .fetch_optional(&state.db)
    .await?;
    let order = order.ok_or_else(|| ApiError::not_found("订单不存在"))?;
    let mut data = order_admin_json(&state, &order).await?;
    if let Some(object) = data.as_object_mut() {
        let surplus_orders: Vec<OrderRow> = if let Some(ids) = &order.surplus_order_ids {
            let parsed: Vec<i64> = serde_json::from_str(ids).unwrap_or_default();
            if parsed.is_empty() {
                Vec::new()
            } else {
                let placeholders = vec!["?"; parsed.len()].join(",");
                let sql = format!("SELECT * FROM v2_order WHERE id IN ({placeholders})");
                let mut query = sqlx::query_as::<_, OrderRow>(&sql);
                for value in parsed {
                    query = query.bind(value);
                }
                query.fetch_all(&state.db).await.unwrap_or_default()
            }
        } else {
            Vec::new()
        };
        object.insert(
            "surplus_orders".into(),
            Value::Array(
                surplus_orders
                    .iter()
                    .map(|o| {
                        json!({
                            "id": o.id, "trade_no": o.trade_no, "total_amount": o.total_amount,
                            "status": o.status, "created_at": o.created_at,
                        })
                    })
                    .collect(),
            ),
        );
        if let Some(invite_user_id) = order.invite_user_id {
            let invite: Option<(i64, String)> =
                sqlx::query_as("SELECT id, email FROM v2_user WHERE id = ?")
                    .bind(invite_user_id)
                    .fetch_optional(&state.db)
                    .await?;
            object.insert(
                "invite_user".into(),
                invite
                    .map(|(id, email)| json!({"id": id, "email": email}))
                    .unwrap_or(Value::Null),
            );
            let log: Option<(i64, i64)> = sqlx::query_as(
                "SELECT order_amount, get_amount FROM v2_commission_log WHERE trade_no = ? ORDER BY id DESC LIMIT 1",
            )
            .bind(&order.trade_no)
            .fetch_optional(&state.db)
            .await?;
            object.insert(
                "commission_log".into(),
                log.map(|(order_amount, get_amount)| {
                    json!({"order_amount": order_amount, "get_amount": get_amount})
                })
                .unwrap_or(Value::Null),
            );
        }
    }
    Ok(ok(data))
}

pub async fn order_assign(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let email = str_field(&body, "email").unwrap_or_default();
    let plan_id = body.get("plan_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let period = str_field(&body, "period").unwrap_or_default();
    let amount = body
        .get("total_amount")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let user: Option<(i64,)> = sqlx::query_as("SELECT id FROM v2_user WHERE email = ? LIMIT 1")
        .bind(&email)
        .fetch_optional(&state.db)
        .await?;
    let Some((user_id,)) = user else {
        return Err(ApiError::not_found("该用户不存在"));
    };
    let plan: Option<PlanRow> = sqlx::query_as("SELECT * FROM v2_plan WHERE id = ?")
        .bind(plan_id)
        .fetch_optional(&state.db)
        .await?;
    let Some(plan) = plan else {
        return Err(ApiError::not_found("该订阅不存在"));
    };
    let existing: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM v2_order WHERE user_id = ? AND status IN (0,1)",
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;
    if existing.0 > 0 {
        return Err(ApiError::bad_request("该用户还有待支付的订单，无法分配"));
    }
    let period = match period.as_str() {
        "hour_price" => "hourly",
        "day_price" => "daily",
        "month_price" => "monthly",
        "quarter_price" => "quarterly",
        "half_year_price" => "half_yearly",
        "year_price" => "yearly",
        "two_year_price" => "two_yearly",
        "three_year_price" => "three_yearly",
        "onetime_price" => "onetime",
        "reset_price" => "reset_traffic",
        other => other,
    };
    let order_type = if period == "reset_traffic" {
        4
    } else if plan.id != 0 {
        1
    } else {
        1
    };
    let trade_no = util::generate_order_no();
    let now = util::now();
    sqlx::query(
        "INSERT INTO v2_order (invite_user_id, user_id, plan_id, type, period, trade_no, total_amount, status, commission_status, commission_balance, created_at, updated_at) VALUES (NULL, ?, ?, ?, ?, ?, ?, 0, 0, 0, ?, ?)",
    )
    .bind(user_id)
    .bind(plan.id)
    .bind(order_type)
    .bind(period)
    .bind(&trade_no)
    .bind(util::yuan_to_cents(amount))
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;
    Ok(ok(json!(trade_no)))
}

pub async fn order_bulk_commission(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let ids: Vec<i64> = body
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_i64()).collect())
        .unwrap_or_default();
    for id in ids {
        sqlx::query("UPDATE v2_order SET commission_status = 2, actual_commission_balance = commission_balance, updated_at = ? WHERE id = ?")
            .bind(util::now())
            .bind(id)
            .execute(&state.db)
            .await?;
        if let Some((invite_user_id, commission_balance, trade_no, order_amount)) =
            sqlx::query_as::<_, (Option<i64>, i64, String, i64)>(
                "SELECT invite_user_id, commission_balance, trade_no, total_amount FROM v2_order WHERE id = ?",
            )
            .bind(id)
            .fetch_optional(&state.db)
            .await?
        {
            if let Some(invite_user_id) = invite_user_id {
                if commission_balance > 0 {
                    sqlx::query("UPDATE v2_user SET commission_balance = commission_balance + ?, updated_at = ? WHERE id = ?")
                        .bind(commission_balance)
                        .bind(util::now())
                        .bind(invite_user_id)
                        .execute(&state.db)
                        .await?;
                    let now = util::now();
                    let _ = sqlx::query(
                        "INSERT INTO v2_commission_log (invite_user_id, user_id, trade_no, order_amount, get_amount, created_at, updated_at) VALUES (?, 0, ?, ?, ?, ?, ?)",
                    )
                    .bind(invite_user_id)
                    .bind(trade_no)
                    .bind(order_amount)
                    .bind(commission_balance)
                    .bind(now)
                    .bind(now)
                    .execute(&state.db)
                    .await;
                }
            }
        }
    }
    Ok(ok(json!(true)))
}

// ---------------------------------------------------------------------------
// Payments
// ---------------------------------------------------------------------------

pub async fn payments_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(10).max(1);
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_payment")
        .fetch_one(&state.db)
        .await?;
    let rows: Vec<PaymentRow> = sqlx::query_as(
        "SELECT * FROM v2_payment ORDER BY sort ASC, id ASC LIMIT ? OFFSET ?",
    )
    .bind(page_size)
    .bind((current - 1) * page_size)
    .fetch_all(&state.db)
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|p| {
            json!({
                "id": p.id, "uuid": p.uuid, "payment": p.payment, "name": p.name,
                "icon": p.icon, "config": p.config_value(), "notify_domain": p.notify_domain,
                "handling_fee_fixed": p.handling_fee_fixed,
                "handling_fee_percent": p.handling_fee_percent.as_ref().and_then(|v| v.to_string().parse::<f64>().ok()),
                "enable": p.enable != 0, "sort": p.sort,
                "created_at": p.created_at, "updated_at": p.updated_at,
            })
        })
        .collect();
    Ok(paginate(data, total.0, current, page_size))
}

pub async fn payment_methods(State(state): State<SharedState>, _admin: AdminUser) -> ApiResult<Response> {
    Ok(ok(state.plugin.payment_methods().await))
}

pub async fn payment_form(
    State(state): State<SharedState>,
    _admin: AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let payment = str_field(&body, "payment").unwrap_or_else(|| "EPay".into());
    let settings = state.settings().await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = PluginContext::new(&state.db, &settings, &app_url, json!({}));
    let form = state
        .plugin
        .payment_form(&payment, json!({}), &ctx)
        .await
        .unwrap_or_else(|| json!({}));
    Ok(ok(form))
}

pub async fn payment_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    let payment = str_field(&body, "payment").unwrap_or_default();
    let icon = str_field(&body, "icon");
    let config = body.get("config").cloned().unwrap_or_else(|| json!({}));
    let notify_domain = str_field(&body, "notify_domain");
    let enable = crate::api::admin::json_bool(body.get("enable").unwrap_or(&Value::Bool(false))) as i64;
    let sort = body.get("sort").and_then(|v| v.as_i64()).unwrap_or(0);
    let fixed = body.get("handling_fee_fixed").and_then(|v| v.as_i64());
    let percent = body.get("handling_fee_percent").and_then(|v| v.as_f64());
    let now = util::now();
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        sqlx::query(
            "UPDATE v2_payment SET payment = ?, name = ?, icon = ?, config = ?, notify_domain = ?, handling_fee_fixed = ?, handling_fee_percent = ?, enable = ?, sort = ?, updated_at = ? WHERE id = ?",
        )
        .bind(payment)
        .bind(name)
        .bind(icon)
        .bind(config.to_string())
        .bind(notify_domain)
        .bind(fixed)
        .bind(percent)
        .bind(enable)
        .bind(sort)
        .bind(now)
        .bind(id)
        .execute(&state.db)
        .await?;
    } else {
        sqlx::query(
            "INSERT INTO v2_payment (uuid, payment, name, icon, config, notify_domain, handling_fee_fixed, handling_fee_percent, enable, sort, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(util::guid())
        .bind(payment)
        .bind(name)
        .bind(icon)
        .bind(config.to_string())
        .bind(notify_domain)
        .bind(fixed)
        .bind(percent)
        .bind(enable)
        .bind(sort)
        .bind(now)
        .bind(now)
        .execute(&state.db)
        .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn payment_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("DELETE FROM v2_payment WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn payment_copy(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let row: Option<PaymentRow> = sqlx::query_as("SELECT * FROM v2_payment WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    let Some(p) = row else { return Err(ApiError::not_found("支付方式不存在")) };
    let now = util::now();
    sqlx::query(
        "INSERT INTO v2_payment (uuid, payment, name, icon, config, notify_domain, handling_fee_fixed, handling_fee_percent, enable, sort, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(util::guid())
    .bind(p.payment)
    .bind(format!("{} 副本", p.name))
    .bind(p.icon)
    .bind(p.config)
    .bind(p.notify_domain)
    .bind(p.handling_fee_fixed)
    .bind(p.handling_fee_percent)
    .bind(0)
    .bind(p.sort)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;
    Ok(ok(json!(true)))
}

pub async fn payment_show(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let row: Option<PaymentRow> = sqlx::query_as("SELECT * FROM v2_payment WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    let p = row.ok_or_else(|| ApiError::not_found("支付方式不存在"))?;
    Ok(ok(json!({
        "id": p.id, "uuid": p.uuid, "payment": p.payment, "name": p.name,
        "icon": p.icon, "config": p.config_value(), "notify_domain": p.notify_domain,
        "handling_fee_fixed": p.handling_fee_fixed,
        "handling_fee_percent": p.handling_fee_percent.as_ref().and_then(|v| v.to_string().parse::<f64>().ok()),
        "enable": p.enable != 0, "sort": p.sort,
    })))
}

pub async fn payment_sort(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    if let Some(ids) = body.get("ids").and_then(|v| v.as_array()) {
        for (index, id) in ids.iter().enumerate() {
            if let Some(id) = id.as_i64() {
                sqlx::query("UPDATE v2_payment SET sort = ? WHERE id = ?")
                    .bind(index as i64 + 1)
                    .bind(id)
                    .execute(&state.db)
                    .await?;
            }
        }
    }
    Ok(ok(json!(true)))
}

fn bind_value_count<'a>(
    query: sqlx::query::QueryAs<'a, sqlx::MySql, (i64,), sqlx::mysql::MySqlArguments>,
    value: &Value,
) -> sqlx::query::QueryAs<'a, sqlx::MySql, (i64,), sqlx::mysql::MySqlArguments> {
    match value {
        Value::String(s) => query.bind(s.clone()),
        Value::Number(n) if n.is_f64() => query.bind(n.as_f64().unwrap_or(0.0)),
        Value::Number(n) => query.bind(n.as_i64().unwrap_or(0)),
        Value::Bool(b) => query.bind(*b as i64),
        Value::Null => query.bind(Option::<String>::None),
        other => query.bind(other.to_string()),
    }
}

pub async fn user_send_mail(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let subject = str_field(&body, "subject").unwrap_or_else(|| "Fboard 通知".into());
    let content = str_field(&body, "content").unwrap_or_default();
    if content.is_empty() {
        return Err(ApiError::bad_request("邮件内容不能为空"));
    }
    type Recipient = (
        i64,
        String,
        String,
        Option<i64>,
        i64,
        i64,
        i64,
        i64,
        Option<String>,
    );
    let select = "SELECT u.id, u.email, u.uuid, u.expired_at, u.transfer_enable, u.u, u.d, CAST(UNIX_TIMESTAMP(u.created_at) AS SIGNED), p.name FROM v2_user u LEFT JOIN v2_plan p ON p.id = u.plan_id";
    let mut recipients: Vec<Recipient> = Vec::new();
    if let Some(filter) = body.get("filter") {
        if let Some(id) = filter.get("id").and_then(|v| v.as_i64()) {
            recipients = sqlx::query_as(&format!("{select} WHERE u.id = ?"))
                .bind(id)
                .fetch_all(&state.db)
                .await?;
        } else if let Some(email) = filter.get("email").and_then(|v| v.as_str()) {
            recipients = sqlx::query_as(&format!("{select} WHERE u.email = ?"))
                .bind(email)
                .fetch_all(&state.db)
                .await?;
        }
    }
    if recipients.is_empty() {
        recipients = sqlx::query_as(&format!(
            "{select} WHERE u.banned = 0 ORDER BY u.id ASC LIMIT 5000"
        ))
        .fetch_all(&state.db)
        .await?;
    }
    let settings = state.settings().await;
    let app_name = settings.str("app_name", "Fboard");
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let now_text = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut sent = 0usize;
    let mut failed = 0usize;
    for (id, email, uuid, expired_at, transfer_enable, u, d, _created_at, plan_name) in recipients {
        let mut render_vars = HashMap::new();
        render_vars.insert("app.name".into(), app_name.clone());
        render_vars.insert("app.url".into(), app_url.clone());
        render_vars.insert("now".into(), now_text.clone());
        render_vars.insert("user.id".into(), id.to_string());
        render_vars.insert("user.email".into(), email.clone());
        render_vars.insert("user.uuid".into(), uuid);
        render_vars.insert("user.plan_name".into(), plan_name.unwrap_or_default());
        render_vars.insert(
            "user.expired_at".into(),
            expired_at
                .map(|v| {
                    chrono::DateTime::from_timestamp(v, 0)
                        .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_default()
                })
                .unwrap_or_default(),
        );
        render_vars.insert("user.transfer_enable".into(), transfer_enable.to_string());
        render_vars.insert("user.transfer_used".into(), (u + d).to_string());
        render_vars.insert(
            "user.transfer_left".into(),
            (transfer_enable - (u + d)).to_string(),
        );

        let mut vars = HashMap::new();
        vars.insert("name".into(), app_name.clone());
        vars.insert("url".into(), app_url.clone());
        vars.insert(
            "content".into(),
            crate::mail::text_to_html(&crate::mail::render(&content, &render_vars)),
        );
        let subject = crate::mail::render(&subject, &render_vars);
        match crate::mail::send_template(&state.db, &settings, &email, "notify", &subject, &vars)
            .await
        {
            Ok(()) => sent += 1,
            Err(err) => {
                failed += 1;
                tracing::warn!(error = %err, %email, "admin mail send failed");
            }
        }
    }
    Ok(ok(json!({"sent": sent, "failed": failed})))
}
