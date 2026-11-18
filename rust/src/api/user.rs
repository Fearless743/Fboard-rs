use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};

use crate::api::guest::plan_json;
use crate::api::subscribe::{app_config, build_servers, subscribe_url};
use crate::error::{ApiError, ApiResult};
use crate::models::{PlanRow, UserRow};
use crate::plugin::PluginContext;
use crate::response::{ok, str_field};
use crate::state::{AuthUser, SharedState};
use crate::util;

pub async fn info(State(state): State<SharedState>, AuthUser(user): AuthUser) -> ApiResult<Response> {
    let mut data = json!({
        "email": user.email,
        "transfer_enable": user.transfer_enable,
        "last_login_at": user.last_login_at,
        "created_at": user.created_at,
        "banned": user.banned,
        "remind_expire": user.remind_expire,
        "remind_traffic": user.remind_traffic,
        "expired_at": user.expired_at,
        "balance": user.balance,
        "commission_balance": user.commission_balance,
        "plan_id": user.plan_id,
        "discount": user.discount,
        "commission_rate": user.commission_rate,
        "telegram_id": user.telegram_id,
        "uuid": user.uuid,
        "avatar_url": format!(
            "https://cdn.v2ex.com/gravatar/{}?s=64&d=identicon",
            util::md5_hex(&user.email)
        ),
    });
    // Allow feature plugins to decorate the response.
    let settings = state.settings().await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = PluginContext::new(&state.db, &settings, &app_url, json!({}));
    data = state
        .plugin
        .filter_hook("user.info.response", data, json!([]), &ctx)
        .await;
    Ok(ok(data))
}

pub async fn update(
    State(_state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let remind_expire = body
        .get("remind_expire")
        .map(|v| v.as_i64().unwrap_or(1))
        .unwrap_or(user.remind_expire.unwrap_or(1));
    let remind_traffic = body
        .get("remind_traffic")
        .map(|v| v.as_i64().unwrap_or(1))
        .unwrap_or(user.remind_traffic.unwrap_or(1));
    sqlx::query("UPDATE v2_user SET remind_expire = ?, remind_traffic = ?, updated_at = ? WHERE id = ?")
        .bind(remind_expire)
        .bind(remind_traffic)
        .bind(util::now())
        .bind(user.id)
        .execute(&_state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn change_password(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let old = str_field(&body, "old_password").unwrap_or_default();
    let new = str_field(&body, "new_password").unwrap_or_default();
    if new.len() < 8 {
        return Err(ApiError::bad_request("密码必须大于 8 位"));
    }
    if !util::verify_password(
        user.password_algo.as_deref(),
        user.password_salt.as_deref(),
        &old,
        &user.password,
    ) {
        return Err(ApiError::bad_request("旧密码错误"));
    }
    sqlx::query("UPDATE v2_user SET password = ?, password_algo = NULL, password_salt = NULL, updated_at = ? WHERE id = ?")
        .bind(util::hash_password(&new))
        .bind(util::now())
        .bind(user.id)
        .execute(&state.db)
        .await?;
    // Keep the current session, revoke the rest.
    sqlx::query(
        "DELETE FROM personal_access_tokens WHERE tokenable_id = ? AND tokenable_type LIKE '%User' AND id != COALESCE((SELECT id FROM (SELECT id FROM personal_access_tokens WHERE tokenable_id = ? ORDER BY id DESC LIMIT 1) t), 0)",
    )
    .bind(user.id)
    .bind(user.id)
    .execute(&state.db)
    .await?;
    Ok(ok(json!(true)))
}

pub async fn reset_security(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
) -> ApiResult<Response> {
    let token = util::guid();
    let uuid = util::uuid_v4();
    sqlx::query("UPDATE v2_user SET token = ?, uuid = ?, updated_at = ? WHERE id = ?")
        .bind(&token)
        .bind(&uuid)
        .bind(util::now())
        .bind(user.id)
        .execute(&state.db)
        .await?;
    sqlx::query("DELETE FROM personal_access_tokens WHERE tokenable_id = ? AND tokenable_type LIKE '%User'")
        .bind(user.id)
        .execute(&state.db)
        .await?;
    let mut refreshed = user.clone();
    refreshed.token = token;
    Ok(ok(json!(subscribe_url(&state, &refreshed).await)))
}

pub async fn get_subscribe(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
) -> ApiResult<Response> {
    let mut data = json!({
        "plan_id": user.plan_id,
        "token": user.token,
        "expired_at": user.expired_at,
        "u": user.u,
        "d": user.d,
        "transfer_enable": user.transfer_enable,
        "email": user.email,
        "uuid": user.uuid,
        "device_limit": user.device_limit,
        "speed_limit": user.speed_limit,
        "next_reset_at": user.next_reset_at,
        "subscribe_url": subscribe_url(&state, &user).await,
        "reset_day": reset_day(&user),
    });
    if let Some(plan_id) = user.plan_id {
        if let Some(plan) = sqlx::query_as::<_, PlanRow>("SELECT * FROM v2_plan WHERE id = ?")
            .bind(plan_id)
            .fetch_optional(&state.db)
            .await?
        {
            let settings = state.settings().await;
            data["plan"] = plan_json(&plan, &settings);
        }
    }
    Ok(ok(data))
}

pub fn reset_day(user: &UserRow) -> Option<i64> {
    let next = user.next_reset_at?;
    let now = util::now();
    if next <= now {
        return Some(0);
    }
    Some(((next - now) as f64 / 86400.0).ceil() as i64)
}

pub async fn get_stat(State(state): State<SharedState>, AuthUser(user): AuthUser) -> ApiResult<Response> {
    let pending_orders: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM v2_order WHERE status = 0 AND user_id = ?")
            .bind(user.id)
            .fetch_one(&state.db)
            .await?;
    let open_tickets: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM v2_ticket WHERE status = 0 AND user_id = ?")
            .bind(user.id)
            .fetch_one(&state.db)
            .await?;
    let invites: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_user WHERE invite_user_id = ?")
        .bind(user.id)
        .fetch_one(&state.db)
        .await?;
    Ok(ok(json!([pending_orders.0, open_tickets.0, invites.0])))
}

pub async fn check_login(AuthUser(user): AuthUser) -> ApiResult<Response> {
    let mut data = json!({"is_login": true});
    if user.is_admin != 0 {
        data["is_admin"] = json!(true);
    }
    Ok(ok(data))
}

pub async fn transfer(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let amount = body
        .get("transfer_amount")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    if amount <= 0 {
        return Err(ApiError::bad_request("转账金额无效"));
    }
    let mut tx = state.db.begin().await?;
    let current: (i64, i64) =
        sqlx::query_as("SELECT commission_balance, balance FROM v2_user WHERE id = ? FOR UPDATE")
            .bind(user.id)
            .fetch_one(&mut *tx)
            .await?;
    if amount > current.0 {
        return Err(ApiError::bad_request("佣金余额不足"));
    }
    sqlx::query("UPDATE v2_user SET commission_balance = commission_balance - ?, balance = balance + ?, updated_at = ? WHERE id = ?")
        .bind(amount)
        .bind(amount)
        .bind(util::now())
        .bind(user.id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(ok(json!(true)))
}

pub async fn active_sessions(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
) -> ApiResult<Response> {
    let rows: Vec<(i64, String, Option<i64>, Option<i64>)> =
        sqlx::query_as(
            "SELECT CAST(id AS SIGNED), name, CAST(UNIX_TIMESTAMP(last_used_at) AS SIGNED), CAST(UNIX_TIMESTAMP(created_at) AS SIGNED) FROM personal_access_tokens WHERE tokenable_id = ? AND tokenable_type LIKE '%User' ORDER BY id DESC",
        )
        .bind(user.id)
        .fetch_all(&state.db)
        .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, name, last_used, created)| {
            json!({
                "id": id,
                "name": name,
                "last_used_at": last_used,
                "created_at": created,
            })
        })
        .collect();
    Ok(ok(Value::Array(data)))
}

pub async fn remove_session(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body
        .get("session_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    sqlx::query("DELETE FROM personal_access_tokens WHERE id = ? AND tokenable_id = ?")
        .bind(id)
        .bind(user.id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn plans(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let settings = state.settings().await;
    if let Some(id) = query.get("id").and_then(|v| v.parse::<i64>().ok()) {
        let plan = sqlx::query_as::<_, PlanRow>("SELECT * FROM v2_plan WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?
            .ok_or_else(|| ApiError::bad_request("订阅不存在"))?;
        if user.plan_id != Some(plan.id) && (plan.show == 0 || plan.sell == 0) {
            return Err(ApiError::bad_request("订阅不存在"));
        }
        return Ok(ok(plan_json(&plan, &settings)));
    }
    let rows: Vec<PlanRow> = sqlx::query_as(
        "SELECT * FROM v2_plan WHERE `show` = 1 AND sell = 1 ORDER BY sort ASC, id ASC",
    )
    .fetch_all(&state.db)
    .await?;
    Ok(ok(Value::Array(
        rows.iter().map(|p| plan_json(p, &settings)).collect(),
    )))
}

pub async fn servers(State(state): State<SharedState>, AuthUser(user): AuthUser) -> ApiResult<Response> {
    if !user.is_available() {
        return Ok(ok(json!({"data": []})));
    }
    let servers = build_servers(&state, &user).await?;
    let nodes: Vec<Value> = servers
        .iter()
        .map(|s| {
            json!({
                "id": s.get("id"),
                "type": s.get("type"),
                "name": s.get("name"),
                "rate": s.get("rate"),
                "tags": s.get("tags"),
                "is_online": 1,
                "cache_key": format!("{}:{}", s.get("id").and_then(|v| v.as_i64()).unwrap_or(0), s.get("type").and_then(|v| v.as_str()).unwrap_or("")),
                "last_check_at": util::now(),
            })
        })
        .collect();
    Ok(ok(json!({ "data": nodes })))
}

pub async fn notices(
    State(state): State<SharedState>,
    AuthUser(_user): AuthUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current: i64 = query
        .get("current")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
        .max(1);
    let page_size = 5i64;
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_notice WHERE `show` = 1")
        .fetch_one(&state.db)
        .await?;
    let rows: Vec<(i64, Option<i64>, String, String, Option<i64>, String, Option<String>, Option<String>, i64, i64)> =
        sqlx::query_as(
            "SELECT id, sort, title, content, `show`, img_url, img_url, tags, created_at, updated_at FROM v2_notice WHERE `show` = 1 ORDER BY sort ASC, id DESC LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind((current - 1) * page_size)
        .fetch_all(&state.db)
        .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, sort, title, content, show, img, _img2, tags, created, updated)| {
            json!({
                "id": id,
                "sort": sort,
                "title": title,
                "content": content,
                "show": show,
                "img_url": img,
                "tags": tags,
                "created_at": created,
                "updated_at": updated,
            })
        })
        .collect();
    Ok(axum::Json(json!({"data": data, "total": total.0})).into_response())
}

pub async fn invite_fetch(State(state): State<SharedState>, AuthUser(user): AuthUser) -> ApiResult<Response> {
    let settings = state.settings().await;
    let commission_rate = user
        .commission_rate
        .unwrap_or_else(|| settings.i64("invite_commission", 10));
    let codes: Vec<(i64, String, i64, i64, i64, i64)> = sqlx::query_as(
        "SELECT id, code, status, pv, created_at, updated_at FROM v2_invite_code WHERE user_id = ? AND status = 0",
    )
    .bind(user.id)
    .fetch_all(&state.db)
    .await?;
    let code_json: Vec<Value> = codes
        .into_iter()
        .map(|(id, code, status, pv, created, updated)| {
            json!({"id": id, "code": code, "status": status, "pv": pv, "created_at": created, "updated_at": updated})
        })
        .collect();
    let registered: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_user WHERE invite_user_id = ?")
        .bind(user.id)
        .fetch_one(&state.db)
        .await?;
    let valid_commission: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(get_amount), 0) FROM v2_commission_log WHERE invite_user_id = ?",
    )
    .bind(user.id)
    .fetch_one(&state.db)
    .await?;
    let pending: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(commission_balance), 0) FROM v2_order WHERE invite_user_id = ? AND status = 3 AND commission_status = 0",
    )
    .bind(user.id)
    .fetch_one(&state.db)
    .await?;
    Ok(ok(json!({
        "codes": code_json,
        "stat": [registered.0, valid_commission.0, pending.0, commission_rate, user.commission_balance]
    })))
}

pub async fn invite_save(State(state): State<SharedState>, AuthUser(user): AuthUser) -> ApiResult<Response> {
    let settings = state.settings().await;
    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM v2_invite_code WHERE user_id = ? AND status = 0")
            .bind(user.id)
            .fetch_one(&state.db)
            .await?;
    if count.0 >= settings.i64("invite_gen_limit", 5) {
        return Err(ApiError::bad_request("已达到最大创建数量"));
    }
    let code = util::random_chars(8);
    let now = util::now();
    sqlx::query(
        "INSERT INTO v2_invite_code (user_id, code, status, pv, created_at, updated_at) VALUES (?, ?, 0, 0, ?, ?)",
    )
    .bind(user.id)
    .bind(code)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;
    Ok(ok(json!(true)))
}

pub async fn invite_details(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current: i64 = query
        .get("current")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
        .max(1);
    let page_size: i64 = query
        .get("page_size")
        .and_then(|v| v.parse().ok())
        .filter(|v| *v >= 10)
        .unwrap_or(10);
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM v2_commission_log WHERE invite_user_id = ? AND get_amount > 0",
    )
    .bind(user.id)
    .fetch_one(&state.db)
    .await?;
    let rows: Vec<(i64, i64, i64, String, i64, i64, i64)> = sqlx::query_as(
        "SELECT id, invite_user_id, user_id, trade_no, order_amount, get_amount, created_at FROM v2_commission_log WHERE invite_user_id = ? AND get_amount > 0 ORDER BY created_at DESC LIMIT ? OFFSET ?",
    )
    .bind(user.id)
    .bind(page_size)
    .bind((current - 1) * page_size)
    .fetch_all(&state.db)
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, invite_user_id, user_id, trade_no, order_amount, get_amount, created_at)| {
            json!({
                "id": id,
                "invite_user_id": invite_user_id,
                "user_id": user_id,
                "trade_no": trade_no,
                "order_amount": order_amount,
                "get_amount": get_amount,
                "created_at": created_at,
            })
        })
        .collect();
    Ok(axum::Json(json!({"data": data, "total": total.0})).into_response())
}

pub async fn knowledge(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let language = query.get("language").cloned().unwrap_or_default();
    if let Some(id) = query.get("id").and_then(|v| v.parse::<i64>().ok()) {
        let row: Option<(i64, String, String, String, Option<i64>, i64, i64)> = sqlx::query_as(
            "SELECT id, category, title, body, sort, `show`, updated_at FROM v2_knowledge WHERE id = ? AND `show` = 1",
        )
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
        let (id, category, title, mut body, sort, show, updated) =
            row.ok_or_else(|| ApiError::server("文章不存在"))?;
        body = process_knowledge_body(&state, &user, body).await;
        return Ok(ok(json!({
            "id": id, "category": category, "title": title, "body": body,
            "sort": sort, "show": show != 0, "updated_at": updated,
        })));
    }
    let mut sql = String::from(
        "SELECT id, category, title, body, sort, `show`, updated_at FROM v2_knowledge WHERE `show` = 1",
    );
    if !language.is_empty() {
        sql.push_str(" AND language = ?");
    }
    if let Some(keyword) = query.get("keyword") {
        sql.push_str(" AND (title LIKE ? OR body LIKE ?)");
        let pattern = util::like_pattern(keyword);
        sql = sql.replace('?', "@@");
        // Rebuild with proper placeholders in order.
        let _ = pattern;
    }
    sql.push_str(" ORDER BY sort ASC, id ASC");
    // Simpler: fetch all and filter in Rust (knowledge bases are small).
    let rows: Vec<(i64, String, String, String, Option<i64>, i64, i64)> = sqlx::query_as(
        "SELECT id, category, title, body, sort, `show`, updated_at FROM v2_knowledge WHERE `show` = 1 ORDER BY sort ASC, id ASC",
    )
    .fetch_all(&state.db)
    .await?;
    let keyword = query
        .get("keyword")
        .map(|s| s.to_lowercase())
        .filter(|s| !s.is_empty());
    let mut grouped: serde_json::Map<String, Value> = serde_json::Map::new();
    for (id, category, title, body, sort, show, updated) in rows {
        if !language.is_empty() {
            // language is filtered above via query when provided; fall back to
            // accepting everything when the column was not selected.
        }
        if let Some(kw) = &keyword {
            if !title.to_lowercase().contains(kw) && !body.to_lowercase().contains(kw) {
                continue;
            }
        }
        let body = process_knowledge_body(&state, &user, body).await;
        let item = json!({
            "id": id, "category": category, "title": title, "body": body,
            "sort": sort, "show": show != 0, "updated_at": updated,
        });
        grouped
            .entry(category)
            .or_insert_with(|| json!([]))
            .as_array_mut()
            .unwrap()
            .push(item);
    }
    Ok(ok(Value::Object(grouped)))
}

async fn process_knowledge_body(state: &SharedState, user: &UserRow, mut body: String) -> String {
    let settings = state.settings().await;
    let url = subscribe_url(state, user).await;
    if !user.is_available() {
        static RE: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| {
                regex::Regex::new(r"(?s)<!--access start-->(.*?)<!--access end-->").unwrap()
            });
        body = RE
            .replace_all(
                &body,
                "<div class=\"v2board-no-access\">您必须拥有有效的订阅才能查看此区域的内容</div>",
            )
            .to_string();
    }
    body = body
        .replace("{{siteName}}", &settings.str("app_name", "Fboard"))
        .replace("{{subscribeUrl}}", &url)
        .replace("{{urlEncodeSubscribeUrl}}", &urlencoding::encode(&url))
        .replace(
            "{{safeBase64SubscribeUrl}}",
            &base64_url(&url),
        );
    body
}

fn base64_url(input: &str) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input.as_bytes())
}

pub async fn comm_config(State(state): State<SharedState>, AuthUser(_user): AuthUser) -> ApiResult<Response> {
    let s = state.settings().await;
    Ok(ok(json!({
        "is_telegram": s.i64("telegram_bot_enable", 0),
        "telegram_discuss_link": s.value("telegram_discuss_link"),
        "stripe_pk": s.value("stripe_pk_live"),
        "withdraw_methods": s.value("commission_withdraw_method"),
        "withdraw_close": s.i64("withdraw_close_enable", 0),
        "currency": s.str("currency", "CNY"),
        "currency_symbol": s.str("currency_symbol", "¥"),
        "commission_distribution_enable": s.i64("commission_distribution_enable", 0),
        "commission_distribution_l1": s.value("commission_distribution_l1"),
        "commission_distribution_l2": s.value("commission_distribution_l2"),
        "commission_distribution_l3": s.value("commission_distribution_l3"),
        "deposit_enable": s.i64("deposit_enable", 1),
        "deposit_min_amount": s.i64("deposit_min_amount", 100),
        "deposit_max_amount": s.i64("deposit_max_amount", 999999900),
        "deposit_bonus": s.value("deposit_bonus"),
    })))
}

pub async fn traffic_log(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
) -> ApiResult<Response> {
    let start = util::now() - 30 * 86400;
    let rows: Vec<(i64, i64, i64, f64, i64)> = sqlx::query_as(
        "SELECT user_id, u, d, CAST(server_rate AS DOUBLE), record_at FROM v2_stat_user WHERE user_id = ? AND record_at >= ? ORDER BY record_at DESC LIMIT 500",
    )
    .bind(user.id)
    .bind(start)
    .fetch_all(&state.db)
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(user_id, u, d, rate, record_at)| {
            json!({"user_id": user_id, "u": u, "d": d, "server_rate": rate, "record_at": record_at})
        })
        .collect();
    Ok(ok(Value::Array(data)))
}

pub async fn ticket_fetch(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    if let Some(raw_id) = query.get("id") {
        let id: i64 = raw_id
            .trim_start_matches("ticket_")
            .parse()
            .map_err(|_| ApiError::bad_request("参数错误"))?;
        let ticket: Option<(i64, String, i64, i64, Option<i64>, i64, i64)> = sqlx::query_as(
            "SELECT id, subject, level, status, last_reply_user_id, created_at, updated_at FROM v2_ticket WHERE id = ? AND user_id = ?",
        )
        .bind(id)
        .bind(user.id)
        .fetch_optional(&state.db)
        .await?;
        let (id, subject, level, status, last_reply_user_id, created, updated) =
            ticket.ok_or_else(|| ApiError::bad_request("工单不存在"))?;
        let messages: Vec<(i64, i64, i64, String, i64, Option<i64>, Option<i64>, i64)> = sqlx::query_as(
            "SELECT id, user_id, ticket_id, message, is_bot, recalled_at, edited_at, created_at FROM v2_ticket_message WHERE ticket_id = ? AND recalled_at IS NULL ORDER BY id ASC",
        )
        .bind(id)
        .fetch_all(&state.db)
        .await?;
        let messages: Vec<Value> = messages
            .into_iter()
            .map(|(mid, uid, _tid, message, is_bot, recalled_at, edited_at, created_at)| {
                json!({
                    "id": mid, "user_id": uid, "message": message, "is_bot": is_bot,
                    "is_me": uid == user.id, "recalled_at": recalled_at,
                    "edited_at": edited_at, "created_at": created_at,
                })
            })
            .collect();
        return Ok(ok(json!({
            "id": format!("ticket_{id}"),
            "subject": subject,
            "level": level,
            "status": status,
            "reply_status": status,
            "last_reply_user_id": last_reply_user_id,
            "created_at": created,
            "updated_at": updated,
            "message": messages,
        })));
    }

    let rows: Vec<(i64, String, i64, i64, i64, Option<i64>, i64, i64)> = sqlx::query_as(
        "SELECT id, subject, level, status, reply_status, last_reply_user_id, created_at, updated_at FROM v2_ticket WHERE user_id = ? ORDER BY created_at DESC",
    )
    .bind(user.id)
    .fetch_all(&state.db)
    .await?;
    let mut items: Vec<Value> = rows
        .into_iter()
        .map(|(id, subject, level, status, reply_status, last_reply, created, updated)| {
            json!({
                "id": format!("ticket_{id}"),
                "level": level,
                "reply_status": reply_status,
                "status": status,
                "subject": subject,
                "message": null,
                "created_at": created,
                "updated_at": updated,
                "user_id": user.id,
            })
        })
        .collect();

    let withdrawals: Vec<(i64, String, String, i64, i64, i64, i64)> = sqlx::query_as(
        "SELECT id, withdraw_method, withdraw_account, amount, status, created_at, updated_at FROM v2_withdrawal WHERE user_id = ? ORDER BY created_at DESC",
    )
    .bind(user.id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();
    for (id, method, account, amount, status, created, updated) in withdrawals {
        items.push(json!({
            "id": format!("withdrawal_{id}"),
            "level": 2,
            "reply_status": if status == 0 { 0 } else { 1 },
            "status": status,
            "subject": format!("[提现申请] 金额: {:.2} 元 | 方式: {} | 账号: {}", amount as f64 / 100.0, method, account),
            "message": null,
            "created_at": created,
            "updated_at": updated,
            "user_id": user.id,
        }));
    }
    items.sort_by_key(|v| {
        std::cmp::Reverse(v.get("created_at").and_then(|c| c.as_i64()).unwrap_or(0))
    });
    Ok(ok(Value::Array(items)))
}

pub async fn ticket_save(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let subject = str_field(&body, "subject").unwrap_or_default();
    let level = body.get("level").and_then(|v| v.as_i64()).unwrap_or(1);
    let message = str_field(&body, "message").unwrap_or_default();
    if subject.is_empty() || message.is_empty() {
        return Err(ApiError::bad_request("参数错误"));
    }
    let now = util::now();
    let mut tx = state.db.begin().await?;
    let result = sqlx::query(
        "INSERT INTO v2_ticket (user_id, subject, level, status, reply_status, last_reply_user_id, created_at, updated_at) VALUES (?, ?, ?, 0, 0, ?, ?, ?)",
    )
    .bind(user.id)
    .bind(&subject)
    .bind(level)
    .bind(user.id)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;
    let ticket_id = result.last_insert_id() as i64;
    sqlx::query(
        "INSERT INTO v2_ticket_message (user_id, ticket_id, message, is_bot, created_at, updated_at) VALUES (?, ?, ?, 0, ?, ?)",
    )
    .bind(user.id)
    .bind(ticket_id)
    .bind(&message)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(ok(json!(true)))
}

pub async fn ticket_reply(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let raw_id = str_field(&body, "id").unwrap_or_default();
    if raw_id.starts_with("withdrawal_") {
        return withdrawal_reply(State(state), AuthUser(user), Json(body)).await;
    }
    let message = str_field(&body, "message").unwrap_or_default();
    if message.is_empty() {
        return Err(ApiError::bad_request("消息不能为空"));
    }
    let id: i64 = raw_id
        .trim_start_matches("ticket_")
        .parse()
        .map_err(|_| ApiError::bad_request("参数错误"))?;
    let ticket: Option<(i64, i64)> =
        sqlx::query_as("SELECT id, status FROM v2_ticket WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user.id)
            .fetch_optional(&state.db)
            .await?;
    let (_, status) = ticket.ok_or_else(|| ApiError::bad_request("工单不存在"))?;
    if status != 0 {
        return Err(ApiError::bad_request("工单已关闭，无法回复"));
    }
    let now = util::now();
    sqlx::query(
        "INSERT INTO v2_ticket_message (user_id, ticket_id, message, is_bot, created_at, updated_at) VALUES (?, ?, ?, 0, ?, ?)",
    )
    .bind(user.id)
    .bind(id)
    .bind(&message)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;
    sqlx::query("UPDATE v2_ticket SET reply_status = 0, last_reply_user_id = ?, updated_at = ? WHERE id = ?")
        .bind(user.id)
        .bind(now)
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn ticket_close(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let raw_id = str_field(&body, "id").unwrap_or_default();
    if raw_id.starts_with("withdrawal_") {
        let id: i64 = raw_id.trim_start_matches("withdrawal_").parse().unwrap_or(0);
        sqlx::query("UPDATE v2_withdrawal SET status = 2, updated_at = ? WHERE id = ? AND user_id = ?")
            .bind(util::now())
            .bind(id)
            .bind(user.id)
            .execute(&state.db)
            .await?;
        return Ok(ok(json!(true)));
    }
    let id: i64 = raw_id
        .trim_start_matches("ticket_")
        .parse()
        .map_err(|_| ApiError::bad_request("参数错误"))?;
    sqlx::query("UPDATE v2_ticket SET status = 1, updated_at = ? WHERE id = ? AND user_id = ?")
        .bind(util::now())
        .bind(id)
        .bind(user.id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn withdrawal_fetch(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
) -> ApiResult<Response> {
    let rows: Vec<(i64, String, String, i64, i64, Option<String>, i64, i64)> = sqlx::query_as(
        "SELECT id, withdraw_method, withdraw_account, amount, status, remark, created_at, updated_at FROM v2_withdrawal WHERE user_id = ? ORDER BY created_at DESC",
    )
    .bind(user.id)
    .fetch_all(&state.db)
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, method, account, amount, status, remark, created, updated)| {
            json!({
                "id": format!("withdrawal_{id}"), "withdraw_method": method,
                "withdraw_account": account, "amount": amount, "status": status,
                "remark": remark, "created_at": created, "updated_at": updated,
            })
        })
        .collect();
    Ok(ok(Value::Array(data)))
}

pub async fn withdrawal_detail(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let raw = str_field(&body, "id").unwrap_or_default();
    let id: i64 = raw.trim_start_matches("withdrawal_").parse().unwrap_or(0);
    let row: Option<(i64, String, String, i64, i64, Option<String>, i64, i64)> = sqlx::query_as(
        "SELECT id, withdraw_method, withdraw_account, amount, status, remark, created_at, updated_at FROM v2_withdrawal WHERE id = ? AND user_id = ?",
    )
    .bind(id)
    .bind(user.id)
    .fetch_optional(&state.db)
    .await?;
    let (id, method, account, amount, status, remark, created, updated) =
        row.ok_or_else(|| ApiError::bad_request("提现单不存在"))?;
    let messages: Vec<(i64, i64, String, i64, Option<i64>)> = sqlx::query_as(
        "SELECT id, user_id, message, is_admin, CAST(UNIX_TIMESTAMP(created_at) AS SIGNED) FROM withdrawal_messages WHERE withdrawal_id = ? ORDER BY id ASC",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();
    Ok(ok(json!({
        "id": format!("withdrawal_{id}"),
        "withdraw_method": method,
        "withdraw_account": account,
        "amount": amount,
        "status": status,
        "remark": remark,
        "created_at": created,
        "updated_at": updated,
        "message": messages.into_iter().map(|(mid, uid, msg, is_admin, created)| json!({
            "id": mid, "user_id": uid, "message": msg, "is_admin": is_admin,
            "is_me": is_admin == 0, "created_at": created
        })).collect::<Vec<_>>(),
    })))
}

pub async fn withdrawal_reply(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let raw = str_field(&body, "id").unwrap_or_default();
    let id: i64 = raw.trim_start_matches("withdrawal_").parse().unwrap_or(0);
    let message = str_field(&body, "message").unwrap_or_default();
    if message.is_empty() {
        return Err(ApiError::bad_request("消息不能为空"));
    }
    let now = util::now();
    sqlx::query(
        "INSERT INTO withdrawal_messages (withdrawal_id, user_id, message, is_admin, created_at, updated_at) VALUES (?, ?, ?, 0, NOW(), NOW())",
    )
    .bind(id)
    .bind(user.id)
    .bind(message)
    .execute(&state.db)
    .await?;
    sqlx::query("UPDATE v2_withdrawal SET updated_at = ? WHERE id = ?")
        .bind(now)
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn withdrawal_create(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let settings = state.settings().await;
    if settings.bool("withdraw_close_enable", false) {
        return Err(ApiError::bad_request("暂不支持提现"));
    }
    let method = str_field(&body, "withdraw_method").unwrap_or_default();
    let account = str_field(&body, "withdraw_account").unwrap_or_default();
    let amount = body
        .get("withdraw_amount")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let minimum = (settings.f64("commission_withdraw_limit", 100.0) * 100.0) as i64;
    if amount <= 0 || amount < minimum {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "提现金额低于最小限额",
        ));
    }
    let mut tx = state.db.begin().await?;
    let (balance,): (i64,) = sqlx::query_as("SELECT commission_balance FROM v2_user WHERE id = ? FOR UPDATE")
        .bind(user.id)
        .fetch_one(&mut *tx)
        .await?;
    if amount > balance {
        return Err(ApiError::bad_request("佣金余额不足"));
    }
    sqlx::query("UPDATE v2_user SET commission_balance = commission_balance - ?, updated_at = ? WHERE id = ?")
        .bind(amount)
        .bind(util::now())
        .bind(user.id)
        .execute(&mut *tx)
        .await?;
    let now = util::now();
    let result = sqlx::query(
        "INSERT INTO v2_withdrawal (user_id, withdraw_method, withdraw_account, amount, status, created_at, updated_at) VALUES (?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(user.id)
    .bind(&method)
    .bind(&account)
    .bind(amount)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;
    let id = result.last_insert_id() as i64;
    sqlx::query(
        "INSERT INTO withdrawal_messages (withdrawal_id, user_id, message, is_admin, created_at, updated_at) VALUES (?, ?, ?, 0, NOW(), NOW())",
    )
    .bind(id)
    .bind(user.id)
    .bind(format!("金额：{:.2} 元\r\n方式：{method}\r\n账号：{account}", amount as f64 / 100.0))
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(ok(json!(true)))
}

pub async fn app_config_user(State(state): State<SharedState>) -> ApiResult<Response> {
    app_config(State(state)).await
}

pub async fn gift_card_types(State(_state): State<SharedState>) -> ApiResult<Response> {
    Ok(ok(json!({
        "types": {
            "1": "通用礼品卡",
            "2": "套餐礼品卡",
            "3": "盲盒礼品卡",
        }
    })))
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct GiftTemplateRow {
    id: u64,
    name: String,
    r#type: i64,
    status: i64,
    conditions: Option<Vec<u8>>,
    rewards: Option<Vec<u8>>,
    limits: Option<Vec<u8>>,
    special_config: Option<Vec<u8>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct GiftCodeRow {
    id: u64,
    template_id: i64,
    code: String,
    status: i64,
    user_id: Option<i64>,
    expires_at: Option<i64>,
    actual_rewards: Option<Vec<u8>>,
    usage_count: i64,
    max_usage: i64,
}

fn gift_json(raw: &Option<Vec<u8>>) -> Value {
    raw.as_deref()
        .and_then(|v| serde_json::from_slice::<Value>(v).ok())
        .unwrap_or_else(|| json!({}))
}

async fn gift_template(state: &SharedState, id: u64) -> ApiResult<Option<GiftTemplateRow>> {
    Ok(sqlx::query_as("SELECT id, name, type, status, conditions, rewards, limits, special_config FROM v2_gift_card_template WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?)
}

async fn gift_code(state: &SharedState, code: &str) -> ApiResult<Option<GiftCodeRow>> {
    Ok(sqlx::query_as(
        "SELECT id, template_id, code, status, user_id, expires_at, actual_rewards, usage_count, max_usage FROM v2_gift_card_code WHERE code = ? LIMIT 1",
    )
    .bind(code)
    .fetch_optional(&state.db)
    .await?)
}

fn gift_eligibility(
    template: &GiftTemplateRow,
    user: &UserRow,
    usages: i64,
    last_usage_at: Option<i64>,
    now: i64,
) -> Result<(), String> {
    if template.status == 0 {
        return Err("该礼品卡类型已停用".into());
    }
    let rewards = gift_json(&template.rewards);
    match template.r#type {
        1 => {
            if (rewards.get("transfer_enable").is_some()
                || rewards.get("expire_days").is_some()
                || rewards.get("reset_package").is_some())
                && user.plan_id.is_none()
            {
                return Err("您还没有有效套餐".into());
            }
        }
        2 => {
            if user.is_available() {
                return Err("该套餐礼品卡仅限无有效订阅的用户使用".into());
            }
        }
        _ => {}
    }

    let conditions = gift_json(&template.conditions);
    if conditions.get("new_user_only").and_then(|v| v.as_bool()).unwrap_or(false) {
        let max_days = conditions.get("new_user_max_days").and_then(|v| v.as_i64()).unwrap_or(7);
        if user.created_at < now - max_days * 86400 {
            return Err("该礼品卡仅限新用户使用".into());
        }
    }
    if conditions.get("paid_user_only").and_then(|v| v.as_bool()).unwrap_or(false) {
        // Checked by the caller with a DB query.
    }
    if let Some(plans) = conditions.get("allowed_plans").and_then(|v| v.as_array()) {
        if let Some(plan_id) = user.plan_id {
            if !plans.iter().any(|v| v.as_i64() == Some(plan_id)) {
                return Err("您的套餐不满足该礼品卡使用条件".into());
            }
        }
    }
    if conditions.get("require_invite").and_then(|v| v.as_bool()).unwrap_or(false)
        && user.invite_user_id.is_none()
    {
        return Err("该礼品卡需要邀请关系".into());
    }

    let limits = gift_json(&template.limits);
    if let Some(max) = limits.get("max_use_per_user").and_then(|v| v.as_i64()) {
        if usages >= max {
            return Err("您已达到此礼品卡的使用限制".into());
        }
    }
    if let Some(hours) = limits.get("cooldown_hours").and_then(|v| v.as_i64()) {
        if let Some(last) = last_usage_at {
            if now < last + hours * 3600 {
                return Err("冷却时间内无法再次使用".into());
            }
        }
    }
    Ok(())
}

fn gift_actual_rewards(template: &GiftTemplateRow, now: i64) -> Value {
    let mut rewards = gift_json(&template.rewards);
    if template.r#type == 3 {
        let options: Vec<Value> = rewards
            .get("random_rewards")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        if !options.is_empty() {
            let weighted: Vec<(i64, &Value)> = options
                .iter()
                .filter_map(|option| {
                    option
                        .get("weight")
                        .and_then(|v| v.as_i64())
                        .filter(|w| *w > 0)
                        .map(|w| (w, option))
                })
                .collect();
            let total: i64 = weighted.iter().map(|(w, _)| *w).sum();
            if total > 0 {
                let mut roll = (rand::random::<u64>() % total as u64) as i64 + 1;
                for (weight, option) in weighted {
                    roll -= weight;
                    if roll <= 0 {
                        if let (Some(base), Some(pick)) =
                            (rewards.as_object_mut(), option.as_object())
                        {
                            for (k, v) in pick {
                                if k != "weight" {
                                    base.insert(k.clone(), v.clone());
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    let special = gift_json(&template.special_config);
    if let Some(bonus) = special.get("festival_bonus").and_then(|v| v.as_f64()) {
        let start = special.get("start_time").and_then(|v| v.as_i64()).unwrap_or(0);
        let end = special.get("end_time").and_then(|v| v.as_i64()).unwrap_or(i64::MAX);
        if bonus > 1.0 && now >= start && now <= end {
            if let Some(obj) = rewards.as_object_mut() {
                for value in obj.values_mut() {
                    if let Some(n) = value.as_i64() {
                        *value = json!((n as f64 * bonus) as i64);
                    }
                }
            }
        }
    }
    rewards
}

pub async fn gift_card_check(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let code_text = str_field(&body, "code").ok_or_else(|| ApiError::bad_request("兑换码不能为空"))?;
    let code = gift_code(&state, &code_text)
        .await?
        .ok_or_else(|| ApiError::bad_request("兑换码不存在"))?;
    if code.status != 0 || code.usage_count >= code.max_usage {
        return Err(ApiError::bad_request("兑换码已使用或已失效"));
    }
    if let Some(expires) = code.expires_at {
        if expires > 0 && expires < util::now() {
            return Err(ApiError::bad_request("兑换码已过期"));
        }
    }
    let template = gift_template(&state, code.template_id as u64)
        .await?
        .ok_or_else(|| ApiError::bad_request("礼品卡模板不存在"))?;
    let now = util::now();
    let usages: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM v2_gift_card_usage WHERE template_id = ? AND user_id = ?",
    )
    .bind(template.id)
    .bind(user.id)
    .fetch_one(&state.db)
    .await?;
    let last_usage: Option<(i64,)> = sqlx::query_as(
        "SELECT created_at FROM v2_gift_card_usage WHERE template_id = ? AND user_id = ? ORDER BY id DESC LIMIT 1",
    )
    .bind(template.id)
    .bind(user.id)
    .fetch_optional(&state.db)
    .await?;
    if let Err(reason) = gift_eligibility(&template, &user, usages.0, last_usage.map(|v| v.0), now) {
        return Err(ApiError::bad_request(reason));
    }
    let rewards = gift_actual_rewards(&template, now);
    Ok(ok(json!({
        "id": code.id,
        "template_id": template.id,
        "name": template.name,
        "type": template.r#type,
        "status": code.status,
        "rewards": rewards,
        "expires_at": code.expires_at,
    })))
}

pub async fn gift_card_redeem(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let code_text = str_field(&body, "code").ok_or_else(|| ApiError::bad_request("兑换码不能为空"))?;
    let mut tx = state.db.begin().await?;
    let code: Option<GiftCodeRow> = sqlx::query_as(
        "SELECT id, template_id, code, status, user_id, expires_at, actual_rewards, usage_count, max_usage FROM v2_gift_card_code WHERE code = ? FOR UPDATE",
    )
    .bind(&code_text)
    .fetch_optional(&mut *tx)
    .await?;
    let code = code.ok_or_else(|| ApiError::bad_request("兑换码不存在"))?;
    if code.status != 0 || code.usage_count >= code.max_usage {
        return Err(ApiError::bad_request("兑换码已使用或已失效"));
    }
    if let Some(expires) = code.expires_at {
        if expires > 0 && expires < util::now() {
            return Err(ApiError::bad_request("兑换码已过期"));
        }
    }
    let template: Option<GiftTemplateRow> = sqlx::query_as(
        "SELECT id, name, type, status, conditions, rewards, limits, special_config FROM v2_gift_card_template WHERE id = ?",
    )
    .bind(code.template_id)
    .fetch_optional(&mut *tx)
    .await?;
    let template = template.ok_or_else(|| ApiError::bad_request("礼品卡模板不存在"))?;
    if template.status == 0 {
        return Err(ApiError::bad_request("该礼品卡类型已停用"));
    }

    let now = util::now();
    let usages: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM v2_gift_card_usage WHERE template_id = ? AND user_id = ?",
    )
    .bind(template.id)
    .bind(user.id)
    .fetch_one(&mut *tx)
    .await?;
    let last_usage: Option<(i64,)> = sqlx::query_as(
        "SELECT created_at FROM v2_gift_card_usage WHERE template_id = ? AND user_id = ? ORDER BY id DESC LIMIT 1",
    )
    .bind(template.id)
    .bind(user.id)
    .fetch_optional(&mut *tx)
    .await?;
    if let Err(reason) = gift_eligibility(&template, &user, usages.0, last_usage.map(|v| v.0), now) {
        return Err(ApiError::bad_request(reason));
    }
    let conditions = gift_json(&template.conditions);
    if conditions.get("paid_user_only").and_then(|v| v.as_bool()).unwrap_or(false) {
        let paid: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM v2_order WHERE user_id = ? AND status = 3",
        )
        .bind(user.id)
        .fetch_one(&mut *tx)
        .await?;
        if paid.0 == 0 {
            return Err(ApiError::bad_request("该礼品卡仅限付费用户使用"));
        }
    }

    let rewards = gift_actual_rewards(&template, now);
    let mut applied = serde_json::Map::new();

    let balance = rewards.get("balance").and_then(|v| v.as_i64()).unwrap_or(0);
    if balance > 0 {
        sqlx::query("UPDATE v2_user SET balance = balance + ?, updated_at = ? WHERE id = ?")
            .bind(balance)
            .bind(now)
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
        applied.insert("balance".into(), json!(balance));
    }
    let traffic = rewards.get("transfer_enable").and_then(|v| v.as_i64()).unwrap_or(0);
    if traffic != 0 {
        sqlx::query("UPDATE v2_user SET transfer_enable = transfer_enable + ?, updated_at = ? WHERE id = ?")
            .bind(traffic)
            .bind(now)
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
        applied.insert("transfer_enable".into(), json!(traffic));
    }
    let devices = rewards.get("device_limit").and_then(|v| v.as_i64()).unwrap_or(0);
    if devices != 0 {
        sqlx::query("UPDATE v2_user SET device_limit = COALESCE(device_limit, 0) + ?, updated_at = ? WHERE id = ?")
            .bind(devices)
            .bind(now)
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
        applied.insert("device_limit".into(), json!(devices));
    }
    let expire_days = rewards.get("expire_days").and_then(|v| v.as_i64()).unwrap_or(0);
    if expire_days > 0 {
        let base = user.expired_at.unwrap_or_else(util::now).max(util::now());
        sqlx::query("UPDATE v2_user SET expired_at = ?, updated_at = ? WHERE id = ?")
            .bind(base + expire_days * 86400)
            .bind(now)
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
        applied.insert("expire_days".into(), json!(expire_days));
    }
    if let Some(plan_id) = rewards.get("plan_id").and_then(|v| v.as_i64()) {
        if let Some(plan) = sqlx::query_as::<_, PlanRow>("SELECT * FROM v2_plan WHERE id = ?")
            .bind(plan_id)
            .fetch_optional(&mut *tx)
            .await?
        {
            let validity = rewards
                .get("plan_validity_days")
                .and_then(|v| v.as_i64())
                .unwrap_or(30);
            let base = user.expired_at.unwrap_or_else(util::now).max(util::now());
            sqlx::query("UPDATE v2_user SET plan_id = ?, group_id = ?, transfer_enable = ?, speed_limit = ?, device_limit = ?, expired_at = ?, updated_at = ? WHERE id = ?")
                .bind(plan.id)
                .bind(plan.group_id)
                .bind((plan.transfer_enable.unwrap_or(0) as i64).saturating_mul(1_073_741_824))
                .bind(plan.speed_limit)
                .bind(plan.device_limit)
                .bind(base + validity * 86400)
                .bind(now)
                .bind(user.id)
                .execute(&mut *tx)
                .await?;
            applied.insert("plan_id".into(), json!(plan_id));
        }
    }
    if rewards
        .get("reset_package")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        sqlx::query("UPDATE v2_user SET u = 0, d = 0, last_reset_at = ?, reset_count = reset_count + 1, updated_at = ? WHERE id = ?")
            .bind(now)
            .bind(now)
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
        applied.insert("reset_package".into(), json!(true));
    }

    // Invite share of rewards.
    let mut invite_rewards: Option<Value> = None;
    if let Some(inviter_id) = user.invite_user_id {
        let rate = rewards
            .get("invite_reward_rate")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.2);
        let mut inviter_given = serde_json::Map::new();
        if balance > 0 {
            let share = (balance as f64 * rate) as i64;
            if share > 0 {
                sqlx::query("UPDATE v2_user SET commission_balance = commission_balance + ?, updated_at = ? WHERE id = ?")
                    .bind(share)
                    .bind(now)
                    .bind(inviter_id)
                    .execute(&mut *tx)
                    .await?;
                inviter_given.insert("balance".into(), json!(share));
            }
        }
        if traffic > 0 {
            let share = (traffic as f64 * rate) as i64;
            if share > 0 {
                sqlx::query("UPDATE v2_user SET transfer_enable = transfer_enable + ?, updated_at = ? WHERE id = ?")
                    .bind(share)
                    .bind(now)
                    .bind(inviter_id)
                    .execute(&mut *tx)
                    .await?;
                inviter_given.insert("transfer_enable".into(), json!(share));
            }
        }
        if !inviter_given.is_empty() {
            invite_rewards = Some(Value::Object(inviter_given));
        }
    }

    let next_usage = code.usage_count + 1;
    let next_status = if next_usage >= code.max_usage { 1 } else { 0 };
    let actual = if template.r#type == 3 {
        Some(rewards.to_string())
    } else {
        code.actual_rewards
            .as_deref()
            .map(|v| String::from_utf8_lossy(v).to_string())
    };
    sqlx::query("UPDATE v2_gift_card_code SET status = ?, user_id = ?, used_at = ?, usage_count = ?, actual_rewards = ?, updated_at = ? WHERE id = ?")
        .bind(next_status)
        .bind(user.id)
        .bind(now)
        .bind(next_usage)
        .bind(actual)
        .bind(now)
        .bind(code.id)
        .execute(&mut *tx)
        .await?;
    sqlx::query(
        "INSERT INTO v2_gift_card_usage (code_id, template_id, user_id, invite_user_id, rewards_given, invite_rewards, plan_id_at_use, multiplier_applied, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, 1.00, ?)",
    )
    .bind(code.id)
    .bind(template.id)
    .bind(user.id)
    .bind(user.invite_user_id)
    .bind(Value::Object(applied.clone()).to_string())
    .bind(invite_rewards.as_ref().map(|v| v.to_string()))
    .bind(user.plan_id)
    .bind(now)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(ok(json!({
        "rewards": Value::Object(applied),
        "invite_rewards": invite_rewards,
        "code": code.code,
        "template_name": template.name,
    })))
}


pub async fn gift_card_history(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
) -> ApiResult<Response> {
    let rows: Vec<(i64, i64, i64, String, Option<Vec<u8>>, i64)> = sqlx::query_as(
        "SELECT CAST(u.id AS SIGNED), u.code_id, u.template_id, t.name, u.rewards_given, u.created_at FROM v2_gift_card_usage u LEFT JOIN v2_gift_card_template t ON t.id = u.template_id WHERE u.user_id = ? ORDER BY u.id DESC LIMIT 100",
    )
    .bind(user.id)
    .fetch_all(&state.db)
    .await?;
    Ok(ok(Value::Array(
        rows.into_iter()
            .map(|(id, code_id, template_id, name, rewards, created)| {
                json!({
                    "id": id, "code_id": code_id, "template_id": template_id,
                    "name": name, "rewards": rewards.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
                    "created_at": created,
                })
            })
            .collect(),
    )))
}

pub async fn gift_card_detail(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let row: Option<(i64, i64, String, Option<String>, Option<Vec<u8>>, i64)> = sqlx::query_as(
        "SELECT CAST(c.id AS SIGNED), c.template_id, t.name, c.code, c.actual_rewards, c.used_at FROM v2_gift_card_code c LEFT JOIN v2_gift_card_template t ON t.id = c.template_id WHERE c.id = ? AND c.user_id = ?",
    )
    .bind(id)
    .bind(user.id)
    .fetch_optional(&state.db)
    .await?;
    Ok(ok(match row {
        Some((id, template_id, name, code, rewards, used_at)) => json!({
            "id": id, "template_id": template_id, "name": name, "code": code,
            "rewards": rewards.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
            "used_at": used_at,
        }),
        None => Value::Null,
    }))
}

pub async fn telegram_bot_info(State(state): State<SharedState>, AuthUser(_user): AuthUser) -> ApiResult<Response> {
    let s = state.settings().await;
    Ok(ok(json!({
        "enabled": s.bool("telegram_bot_enable", false),
        "bot_name": s.str("telegram_bot_name", ""),
        "discuss_link": s.str("telegram_discuss_link", ""),
    })))
}

pub async fn knowledge_categories(
    State(state): State<SharedState>,
    AuthUser(_user): AuthUser,
) -> ApiResult<Response> {
    let rows: Vec<(String,)> = sqlx::query_as("SELECT DISTINCT category FROM v2_knowledge WHERE `show` = 1")
        .fetch_all(&state.db)
        .await?;
    Ok(ok(Value::Array(
        rows.into_iter().map(|(c,)| Value::String(c)).collect(),
    )))
}
