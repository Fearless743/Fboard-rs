use std::collections::HashMap;
use chrono::{Datelike, TimeZone};
use std::path::{Path, PathBuf};

use axum::extract::{Multipart, Query, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};

use crate::error::{ApiError, ApiResult};
use crate::response::{ok, paginate, str_field};
use crate::state::{AdminUser, SharedState};
use crate::util;

// ---------------------------------------------------------------------------
// Notices
// ---------------------------------------------------------------------------

pub async fn notices_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(10).max(1);
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_notice")
        .fetch_one(&state.db)
        .await?;
    let rows: Vec<(i64, Option<i64>, String, String, i64, Option<String>, Option<String>, i64, i64)> =
        sqlx::query_as(
            "SELECT id, sort, title, content, `show`, img_url, tags, created_at, updated_at FROM v2_notice ORDER BY sort ASC, id DESC LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind((current - 1) * page_size)
        .fetch_all(&state.db)
        .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, sort, title, content, show, img, tags, created, updated)| {
            json!({
                "id": id, "sort": sort, "title": title, "content": content,
                "show": show, "img_url": img, "tags": tags,
                "created_at": created, "updated_at": updated,
            })
        })
        .collect();
    Ok(paginate(data, total.0, current, page_size))
}

pub async fn notice_detail(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let row: Option<(i64, Option<i64>, String, String, i64, Option<String>, Option<String>, i64, i64)> =
        sqlx::query_as(
            "SELECT id, sort, title, content, `show`, img_url, tags, created_at, updated_at FROM v2_notice WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    Ok(ok(match row {
        Some((id, sort, title, content, show, img, tags, created, updated)) => json!({
            "id": id, "sort": sort, "title": title, "content": content,
            "show": show, "img_url": img, "tags": tags,
            "created_at": created, "updated_at": updated,
        }),
        None => Value::Null,
    }))
}

pub async fn notice_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let title = str_field(&body, "title").unwrap_or_default();
    let content = str_field(&body, "content").unwrap_or_default();
    let sort = body.get("sort").and_then(|v| v.as_i64()).unwrap_or(0);
    let show = if let Some(v) = body.get("show") {
        crate::api::admin::json_bool(v) as i64
    } else {
        0
    };
    let img_url = str_field(&body, "img_url");
    let tags = str_field(&body, "tags");
    let now = util::now();
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        sqlx::query("UPDATE v2_notice SET sort = ?, title = ?, content = ?, `show` = ?, img_url = ?, tags = ?, updated_at = ? WHERE id = ?")
            .bind(sort)
            .bind(title)
            .bind(content)
            .bind(show)
            .bind(img_url)
            .bind(tags)
            .bind(now)
            .bind(id)
            .execute(&state.db)
            .await?;
    } else {
        sqlx::query("INSERT INTO v2_notice (sort, title, content, `show`, img_url, tags, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(sort)
            .bind(title)
            .bind(content)
            .bind(show)
            .bind(img_url)
            .bind(tags)
            .bind(now)
            .bind(now)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn notice_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("DELETE FROM v2_notice WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn notice_show(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let show = crate::api::admin::json_bool(body.get("show").unwrap_or(&Value::Bool(false))) as i64;
    sqlx::query("UPDATE v2_notice SET `show` = ?, updated_at = ? WHERE id = ?")
        .bind(show)
        .bind(util::now())
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn notice_sort(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    if let Some(ids) = body.get("ids").and_then(|v| v.as_array()) {
        for (index, id) in ids.iter().enumerate() {
            if let Some(id) = id.as_i64() {
                sqlx::query("UPDATE v2_notice SET sort = ? WHERE id = ?")
                    .bind(index as i64 + 1)
                    .bind(id)
                    .execute(&state.db)
                    .await?;
            }
        }
    }
    Ok(ok(json!(true)))
}

// ---------------------------------------------------------------------------
// Tickets
// ---------------------------------------------------------------------------

pub async fn tickets_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    if let Some(id) = query.get("id") {
        let id: i64 = id.trim_start_matches("ticket_").parse().unwrap_or(0);
        let ticket: Option<(i64, i64, String, i64, i64, i64, Option<i64>, i64, i64)> = sqlx::query_as(
            "SELECT id, user_id, subject, level, status, reply_status, last_reply_user_id, created_at, updated_at FROM v2_ticket WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
        let Some((id, user_id, subject, level, status, reply_status, last_reply, created, updated)) = ticket else {
            return Err(ApiError::not_found("工单不存在"));
        };
        let messages: Vec<(i64, i64, String, i64, Option<i64>, Option<i64>, i64)> = sqlx::query_as(
            "SELECT id, user_id, message, is_bot, recalled_at, edited_at, created_at FROM v2_ticket_message WHERE ticket_id = ? ORDER BY id ASC",
        )
        .bind(id)
        .fetch_all(&state.db)
        .await?;
        return Ok(ok(json!({
            "id": format!("ticket_{id}"),
            "user_id": user_id,
            "subject": subject,
            "level": level,
            "status": status,
            "reply_status": reply_status,
            "last_reply_user_id": last_reply,
            "created_at": created,
            "updated_at": updated,
            "message": messages.into_iter().map(|(mid, uid, msg, is_bot, recalled, edited, created)| json!({
                "id": mid, "user_id": uid, "message": msg, "is_bot": is_bot != 0,
                "recalled_at": recalled, "edited_at": edited, "created_at": created,
            })).collect::<Vec<_>>(),
        })));
    }

    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(10).max(1);
    let status = query.get("status").and_then(|v| v.parse::<i64>().ok());
    let total: (i64,) = match status {
        Some(status) => sqlx::query_as("SELECT COUNT(*) FROM v2_ticket WHERE status = ?")
            .bind(status)
            .fetch_one(&state.db)
            .await?,
        None => sqlx::query_as("SELECT COUNT(*) FROM v2_ticket")
            .fetch_one(&state.db)
            .await?,
    };
    let rows: Vec<(i64, i64, String, i64, i64, i64, Option<i64>, i64, i64)> = match status {
        Some(status) => sqlx::query_as(
            "SELECT id, user_id, subject, level, status, reply_status, last_reply_user_id, created_at, updated_at FROM v2_ticket WHERE status = ? ORDER BY updated_at DESC LIMIT ? OFFSET ?",
        )
        .bind(status)
        .bind(page_size)
        .bind((current - 1) * page_size)
        .fetch_all(&state.db)
        .await?,
        None => sqlx::query_as(
            "SELECT id, user_id, subject, level, status, reply_status, last_reply_user_id, created_at, updated_at FROM v2_ticket ORDER BY updated_at DESC LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind((current - 1) * page_size)
        .fetch_all(&state.db)
        .await?,
    };
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, user_id, subject, level, status, reply_status, last_reply, created, updated)| {
            json!({
                "id": format!("ticket_{id}"), "user_id": user_id, "subject": subject,
                "level": level, "status": status, "reply_status": reply_status,
                "last_reply_user_id": last_reply, "created_at": created, "updated_at": updated,
            })
        })
        .collect();
    Ok(paginate(data, total.0, current, page_size))
}

pub async fn ticket_reply(
    State(state): State<SharedState>,
    AdminUser(admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let raw = str_field(&body, "id").unwrap_or_default();
    let id: i64 = raw.trim_start_matches("ticket_").parse().unwrap_or(0);
    let message = str_field(&body, "message").unwrap_or_default();
    if message.is_empty() {
        return Err(ApiError::bad_request("消息不能为空"));
    }
    let now = util::now();
    sqlx::query("INSERT INTO v2_ticket_message (user_id, ticket_id, message, is_bot, created_at, updated_at) VALUES (?, ?, ?, 0, ?, ?)")
        .bind(admin.id)
        .bind(id)
        .bind(message)
        .bind(now)
        .bind(now)
        .execute(&state.db)
        .await?;
    sqlx::query("UPDATE v2_ticket SET reply_status = 1, last_reply_user_id = ?, updated_at = ? WHERE id = ?")
        .bind(admin.id)
        .bind(now)
        .bind(id)
        .execute(&state.db)
        .await?;
    {
        let state = state.clone();
        tokio::spawn(async move {
            crate::plugin::notify_telegram(
                &state,
                "telegram.ticket_notify",
                json!({"ticket_id": id}),
            )
            .await;
        });
    }
    Ok(ok(json!(true)))
}

pub async fn ticket_close(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let ids: Vec<i64> = if let Some(ids) = body.get("ids").and_then(|v| v.as_array()) {
        ids.iter()
            .filter_map(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.trim_start_matches("ticket_").parse().ok())))
            .collect()
    } else {
        body.get("id")
            .and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.trim_start_matches("ticket_").parse().ok())))
            .into_iter()
            .collect()
    };
    for id in ids {
        sqlx::query("UPDATE v2_ticket SET status = 1, updated_at = ? WHERE id = ?")
            .bind(util::now())
            .bind(id)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn ticket_message_update(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let message = str_field(&body, "message").unwrap_or_default();
    sqlx::query("UPDATE v2_ticket_message SET message = ?, edited_at = ?, updated_at = ? WHERE id = ?")
        .bind(message)
        .bind(util::now())
        .bind(util::now())
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn ticket_message_recall(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("UPDATE v2_ticket_message SET recalled_at = ?, updated_at = ? WHERE id = ?")
        .bind(util::now())
        .bind(util::now())
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

// ---------------------------------------------------------------------------
// Coupons
// ---------------------------------------------------------------------------

pub async fn coupons_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(10).max(1);
    let search = query.get("search").cloned().unwrap_or_default();
    let (total, rows): (i64, Vec<(i64, String, String, i64, i64, i64, Option<i64>, Option<i64>, Option<String>, Option<String>, i64, i64, i64, i64)>) =
        if search.is_empty() {
            let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_coupon")
                .fetch_one(&state.db)
                .await?;
            let rows = sqlx::query_as(
                "SELECT id, code, name, type, value, `show`, limit_use, limit_use_with_user, limit_plan_ids, limit_period, started_at, ended_at, created_at, updated_at FROM v2_coupon ORDER BY id DESC LIMIT ? OFFSET ?",
            )
            .bind(page_size)
            .bind((current - 1) * page_size)
            .fetch_all(&state.db)
            .await?;
            (total.0, rows)
        } else {
            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM v2_coupon WHERE code LIKE ? OR name LIKE ?",
            )
            .bind(util::like_pattern(&search))
            .bind(util::like_pattern(&search))
            .fetch_one(&state.db)
            .await?;
            let rows = sqlx::query_as(
                "SELECT id, code, name, type, value, `show`, limit_use, limit_use_with_user, limit_plan_ids, limit_period, started_at, ended_at, created_at, updated_at FROM v2_coupon WHERE code LIKE ? OR name LIKE ? ORDER BY id DESC LIMIT ? OFFSET ?",
            )
            .bind(util::like_pattern(&search))
            .bind(util::like_pattern(&search))
            .bind(page_size)
            .bind((current - 1) * page_size)
            .fetch_all(&state.db)
            .await?;
            (total.0, rows)
        };
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, code, name, ty, value, show, limit, limit_user, plan_ids, period, started, ended, created, updated)| {
            json!({
                "id": id, "code": code, "name": name, "type": ty, "value": value,
                "show": show, "limit_use": limit, "limit_use_with_user": limit_user,
                "limit_plan_ids": plan_ids.as_deref().and_then(|v| serde_json::from_str::<Value>(v).ok()),
                "limit_period": period.as_deref().and_then(|v| serde_json::from_str::<Value>(v).ok()),
                "started_at": started, "ended_at": ended,
                "created_at": created, "updated_at": updated,
            })
        })
        .collect();
    Ok(paginate(data, total, current, page_size))
}

pub async fn coupon_generate(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    let r#type = body.get("type").and_then(|v| v.as_i64()).unwrap_or(1);
    let value = body.get("value").and_then(|v| v.as_i64()).unwrap_or(0);
    let count = body.get("generate_count").and_then(|v| v.as_i64()).unwrap_or(1).clamp(1, 500);
    let show = crate::api::admin::json_bool(body.get("show").unwrap_or(&Value::Bool(true))) as i64;
    let limit_use = body.get("limit_use").and_then(|v| v.as_i64());
    let limit_user = body.get("limit_use_with_user").and_then(|v| v.as_i64());
    let plan_ids = body.get("limit_plan_ids").map(|v| v.to_string());
    let period = body.get("limit_period").map(|v| v.to_string());
    let started = body
        .get("started_at")
        .and_then(|v| v.as_i64())
        .unwrap_or(util::now());
    let ended = body
        .get("ended_at")
        .and_then(|v| v.as_i64())
        .unwrap_or(util::now() + 365 * 86400);
    let now = util::now();
    for _ in 0..count {
        let code = util::random_chars(16).to_uppercase();
        sqlx::query(
            "INSERT INTO v2_coupon (code, name, type, value, `show`, limit_use, limit_use_with_user, limit_plan_ids, limit_period, started_at, ended_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(code)
        .bind(&name)
        .bind(r#type)
        .bind(value)
        .bind(show)
        .bind(limit_use)
        .bind(limit_user)
        .bind(&plan_ids)
        .bind(&period)
        .bind(started)
        .bind(ended)
        .bind(now)
        .bind(now)
        .execute(&state.db)
        .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn coupon_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let ids: Vec<i64> = if let Some(ids) = body.get("ids").and_then(|v| v.as_array()) {
        ids.iter().filter_map(|v| v.as_i64()).collect()
    } else {
        body.get("id").and_then(|v| v.as_i64()).into_iter().collect()
    };
    for id in ids {
        sqlx::query("DELETE FROM v2_coupon WHERE id = ?")
            .bind(id)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn coupon_show(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let show = crate::api::admin::json_bool(body.get("show").unwrap_or(&Value::Bool(false))) as i64;
    sqlx::query("UPDATE v2_coupon SET `show` = ?, updated_at = ? WHERE id = ?")
        .bind(show)
        .bind(util::now())
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn coupon_update(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let name = str_field(&body, "name").unwrap_or_default();
    let value = body.get("value").and_then(|v| v.as_i64()).unwrap_or(0);
    let limit = body.get("limit_use").and_then(|v| v.as_i64());
    let show = crate::api::admin::json_bool(body.get("show").unwrap_or(&Value::Bool(true))) as i64;
    sqlx::query("UPDATE v2_coupon SET name = ?, value = ?, limit_use = ?, `show` = ?, updated_at = ? WHERE id = ?")
        .bind(name)
        .bind(value)
        .bind(limit)
        .bind(show)
        .bind(util::now())
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn coupon_batch_drop(
    State(state): State<SharedState>,
    admin: AdminUser,
    body: Json<Value>,
) -> ApiResult<Response> {
    coupon_drop(State(state), admin, body).await
}

pub async fn coupon_drop_expired(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    sqlx::query("DELETE FROM v2_coupon WHERE ended_at < ?")
        .bind(util::now())
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn coupon_drop_depleted(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    sqlx::query("DELETE FROM v2_coupon WHERE limit_use IS NOT NULL AND limit_use <= 0")
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

// ---------------------------------------------------------------------------
// Knowledge
// ---------------------------------------------------------------------------

pub async fn knowledge_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(10).max(1);
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_knowledge")
        .fetch_one(&state.db)
        .await?;
    let rows: Vec<(i64, String, String, String, String, Option<i64>, i64, i64, i64)> = sqlx::query_as(
        "SELECT id, language, category, title, body, sort, `show`, created_at, updated_at FROM v2_knowledge ORDER BY sort ASC, id DESC LIMIT ? OFFSET ?",
    )
    .bind(page_size)
    .bind((current - 1) * page_size)
    .fetch_all(&state.db)
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, language, category, title, body, sort, show, created, updated)| {
            json!({
                "id": id, "language": language, "category": category, "title": title,
                "body": body, "sort": sort, "show": show,
                "created_at": created, "updated_at": updated,
            })
        })
        .collect();
    Ok(paginate(data, total.0, current, page_size))
}

pub async fn knowledge_categories(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let rows: Vec<(String,)> = sqlx::query_as("SELECT DISTINCT category FROM v2_knowledge")
        .fetch_all(&state.db)
        .await?;
    Ok(ok(Value::Array(
        rows.into_iter().map(|(c,)| Value::String(c)).collect(),
    )))
}

pub async fn knowledge_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let language = str_field(&body, "language").unwrap_or_else(|| "zh-CN".into());
    let category = str_field(&body, "category").unwrap_or_default();
    let title = str_field(&body, "title").unwrap_or_default();
    let body_text = str_field(&body, "body").unwrap_or_default();
    let sort = body.get("sort").and_then(|v| v.as_i64()).unwrap_or(0);
    let show = crate::api::admin::json_bool(body.get("show").unwrap_or(&Value::Bool(false))) as i64;
    let now = util::now();
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        sqlx::query("UPDATE v2_knowledge SET language = ?, category = ?, title = ?, body = ?, sort = ?, `show` = ?, updated_at = ? WHERE id = ?")
            .bind(language)
            .bind(category)
            .bind(title)
            .bind(body_text)
            .bind(sort)
            .bind(show)
            .bind(now)
            .bind(id)
            .execute(&state.db)
            .await?;
    } else {
        sqlx::query("INSERT INTO v2_knowledge (language, category, title, body, sort, `show`, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(language)
            .bind(category)
            .bind(title)
            .bind(body_text)
            .bind(sort)
            .bind(show)
            .bind(now)
            .bind(now)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn knowledge_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("DELETE FROM v2_knowledge WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn knowledge_show(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let show = crate::api::admin::json_bool(body.get("show").unwrap_or(&Value::Bool(false))) as i64;
    sqlx::query("UPDATE v2_knowledge SET `show` = ?, updated_at = ? WHERE id = ?")
        .bind(show)
        .bind(util::now())
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn knowledge_sort(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    if let Some(ids) = body.get("ids").and_then(|v| v.as_array()) {
        for (index, id) in ids.iter().enumerate() {
            if let Some(id) = id.as_i64() {
                sqlx::query("UPDATE v2_knowledge SET sort = ? WHERE id = ?")
                    .bind(index as i64 + 1)
                    .bind(id)
                    .execute(&state.db)
                    .await?;
            }
        }
    }
    Ok(ok(json!(true)))
}

// ---------------------------------------------------------------------------
// Gift cards (templates / codes minimal)
// ---------------------------------------------------------------------------

pub async fn gift_templates(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query
        .get("page")
        .or_else(|| query.get("current"))
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(1)
        .max(1);
    let page_size = query
        .get("per_page")
        .or_else(|| query.get("pageSize"))
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(10)
        .clamp(1, 500);

    let mut where_sql = String::from(" WHERE 1=1");
    let mut binds: Vec<Value> = Vec::new();
    if let Some(search) = query.get("search").filter(|v| !v.is_empty()) {
        where_sql.push_str(" AND (name LIKE ? OR description LIKE ?)");
        binds.push(json!(util::like_pattern(search)));
        binds.push(json!(util::like_pattern(search)));
    }
    for key in ["type", "status"] {
        if let Some(value) = query.get(key).and_then(|v| v.parse::<i64>().ok()) {
            where_sql.push_str(&format!(" AND {key} = ?"));
            binds.push(json!(value));
        }
    }

    let count_sql = format!("SELECT COUNT(*) FROM v2_gift_card_template{where_sql}");
    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    for value in &binds {
        count_query = bind_value_count(count_query, value);
    }
    let total = count_query.fetch_one(&state.db).await?.0;

    let sql = format!(
        "SELECT id, name, description, type, status, rewards, conditions, limits, special_config, icon, background_image, theme_color, sort, admin_id, created_at, updated_at FROM v2_gift_card_template{where_sql} ORDER BY sort ASC, id DESC LIMIT ? OFFSET ?"
    );
    binds.push(json!(page_size));
    binds.push(json!((current - 1) * page_size));
    let mut query = sqlx::query(&sql);
    for value in &binds {
        query = bind_value(query, value);
    }
    let rows = query.fetch_all(&state.db).await?;
    use sqlx::Row;
    let mut data = Vec::new();
    for row in rows {
        let id: u64 = row.try_get(0).unwrap_or(0);
        let name: String = row.try_get(1).unwrap_or_default();
        let description: Option<Vec<u8>> = row.try_get(2).ok().flatten();
        let ty: i64 = row.try_get(3).unwrap_or(1);
        let status: i64 = row.try_get(4).unwrap_or(1);
        let rewards: Option<Vec<u8>> = row.try_get(5).ok().flatten();
        let conditions: Option<Vec<u8>> = row.try_get(6).ok().flatten();
        let limits: Option<Vec<u8>> = row.try_get(7).ok().flatten();
        let special: Option<Vec<u8>> = row.try_get(8).ok().flatten();
        let icon: Option<String> = row.try_get(9).ok().flatten();
        let background_image: Option<String> = row.try_get(10).ok().flatten();
        let theme_color: Option<String> = row.try_get(11).ok().flatten();
        let sort: i64 = row.try_get(12).unwrap_or(0);
        let admin_id: i64 = row.try_get(13).unwrap_or(0);
        let created: i64 = row.try_get(14).unwrap_or(0);
        let updated: i64 = row.try_get(15).unwrap_or(0);
        let parse_json = |raw: &Option<Vec<u8>>| {
            raw.as_deref()
                .and_then(|v| serde_json::from_slice::<Value>(v).ok())
        };
        data.push(json!({
            "id": id,
            "name": name,
            "description": description.as_deref().map(|v| String::from_utf8_lossy(v).to_string()),
            "type": ty,
            "status": status,
            "rewards": parse_json(&rewards),
            "conditions": parse_json(&conditions),
            "limits": parse_json(&limits),
            "special_config": parse_json(&special),
            "icon": icon,
            "background_image": background_image,
            "theme_color": theme_color,
            "sort": sort,
            "admin_id": admin_id,
            "created_at": created,
            "updated_at": updated,
        }));
    }
    Ok(paginate(data, total, current, page_size))
}

pub async fn gift_template_sort(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let ids = body
        .get("ids")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    for (index, id) in ids.iter().enumerate() {
        if let Some(id) = id.as_i64() {
            sqlx::query("UPDATE v2_gift_card_template SET sort = ?, updated_at = ? WHERE id = ?")
                .bind(index as i64)
                .bind(util::now())
                .bind(id)
                .execute(&state.db)
                .await?;
        }
    }
    Ok(ok(json!(true)))
}

pub async fn gift_template_save(
    State(state): State<SharedState>,
    AdminUser(admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    let description = str_field(&body, "description");
    let r#type = body.get("type").and_then(|v| v.as_i64()).unwrap_or(1);
    let status = body.get("status").and_then(|v| v.as_i64()).unwrap_or(1);
    let rewards = body.get("rewards").cloned().unwrap_or_else(|| json!({}));
    let conditions = body.get("conditions").map(|v| v.to_string());
    let limits = body.get("limits").map(|v| v.to_string());
    let special = body.get("special_config").map(|v| v.to_string());
    let icon = str_field(&body, "icon");
    let background_image = str_field(&body, "background_image");
    let theme_color = str_field(&body, "theme_color").unwrap_or_else(|| "#1890ff".into());
    let sort = body.get("sort").and_then(|v| v.as_i64()).unwrap_or(0);
    let now = util::now();
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        sqlx::query("UPDATE v2_gift_card_template SET name = ?, description = ?, type = ?, status = ?, rewards = ?, conditions = ?, limits = ?, special_config = ?, icon = ?, background_image = ?, theme_color = ?, sort = ?, updated_at = ? WHERE id = ?")
            .bind(name)
            .bind(description)
            .bind(r#type)
            .bind(status)
            .bind(rewards.to_string())
            .bind(conditions)
            .bind(limits)
            .bind(special)
            .bind(icon)
            .bind(background_image)
            .bind(theme_color)
            .bind(sort)
            .bind(now)
            .bind(id)
            .execute(&state.db)
            .await?;
    } else {
        sqlx::query("INSERT INTO v2_gift_card_template (name, description, type, status, rewards, conditions, limits, special_config, icon, background_image, theme_color, sort, admin_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(name)
            .bind(description)
            .bind(r#type)
            .bind(status)
            .bind(rewards.to_string())
            .bind(conditions)
            .bind(limits)
            .bind(special)
            .bind(icon)
            .bind(background_image)
            .bind(theme_color)
            .bind(sort)
            .bind(admin.id)
            .bind(now)
            .bind(now)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn gift_template_delete(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("DELETE FROM v2_gift_card_template WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

fn format_unix_local(ts: i64) -> String {
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|d| {
            chrono::DateTime::<chrono::Local>::from(d)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        })
        .unwrap_or_default()
}

pub async fn gift_codes(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query
        .get("page")
        .or_else(|| query.get("current"))
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(1)
        .max(1);
    let page_size = query
        .get("per_page")
        .or_else(|| query.get("pageSize"))
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(20)
        .clamp(1, 500);

    let mut sql = String::from("SELECT c.id, c.template_id, c.code, c.status, c.user_id, c.used_at, c.expires_at, c.usage_count, c.max_usage, c.created_at, t.name FROM v2_gift_card_code c LEFT JOIN v2_gift_card_template t ON t.id = c.template_id WHERE 1=1");
    let mut binds: Vec<Value> = Vec::new();
    for (key, value) in &query {
        match key.as_str() {
            "template_id" | "status" | "user_id" => {
                if let Ok(n) = value.parse::<i64>() {
                    sql.push_str(&format!(" AND c.{key} = ?"));
                    binds.push(json!(n));
                }
            }
            "batch_id" if !value.is_empty() => {
                sql.push_str(" AND c.batch_id = ?");
                binds.push(json!(value));
            }
            "search" if !value.is_empty() => {
                sql.push_str(" AND c.code LIKE ?");
                binds.push(json!(util::like_pattern(value)));
            }
            _ => {}
        }
    }
    let count_sql = sql.replace(
        "SELECT c.id, c.template_id, c.code, c.status, c.user_id, c.used_at, c.expires_at, c.usage_count, c.max_usage, c.created_at, t.name",
        "SELECT COUNT(*)",
    );
    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    for value in &binds {
        count_query = bind_value_count(count_query, value);
    }
    let total = count_query.fetch_one(&state.db).await?.0;

    sql.push_str(" ORDER BY c.id DESC LIMIT ? OFFSET ?");
    binds.push(json!(page_size));
    binds.push(json!((current - 1) * page_size));
    let mut query = sqlx::query(&sql);
    for value in &binds {
        query = bind_value(query, value);
    }
    let rows = query.fetch_all(&state.db).await?;
    use sqlx::Row;
    let mut data = Vec::new();
    for row in rows {
        let id: u64 = row.try_get(0).unwrap_or(0);
        let template_id: i64 = row.try_get(1).unwrap_or(0);
        let code: String = row.try_get(2).unwrap_or_default();
        let status: i64 = row.try_get(3).unwrap_or(0);
        let user_id: Option<i64> = row.try_get(4).ok().flatten();
        let used_at: Option<i64> = row.try_get(5).ok().flatten();
        let expires_at: Option<i64> = row.try_get(6).ok().flatten();
        let usage_count: i64 = row.try_get(7).unwrap_or(0);
        let max_usage: i64 = row.try_get(8).unwrap_or(1);
        let created_at: i64 = row.try_get(9).unwrap_or(0);
        let template_name: Option<String> = row.try_get(10).ok().flatten();
        data.push(json!({
            "id": id,
            "template_id": template_id,
            "template": if template_name.is_some() { json!({"id": template_id, "name": template_name}) } else { Value::Null },
            "code": code,
            "status": status,
            "user_id": user_id,
            "used_at": used_at,
            "expires_at": expires_at,
            "usage_count": usage_count,
            "max_usage": max_usage,
            "created_at": format_unix_local(created_at),
        }));
    }
    Ok(paginate(data, total, current, page_size))
}

pub async fn gift_code_toggle(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
    let action = str_field(&body, "action").unwrap_or_else(|| "disable".into());
    let status = if action == "enable" { 0 } else { 3 };
    sqlx::query("UPDATE v2_gift_card_code SET status = ?, updated_at = ? WHERE id = ?")
        .bind(status)
        .bind(util::now())
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn gift_code_update(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
    let mut sets = Vec::new();
    let mut values: Vec<Value> = Vec::new();
    if let Some(status) = body.get("status").and_then(|v| v.as_i64()) {
        sets.push("status = ?");
        values.push(json!(status));
    }
    if let Some(expires) = body.get("expires_at") {
        if expires.is_null() {
            sets.push("expires_at = NULL");
        } else if let Some(v) = expires.as_i64() {
            sets.push("expires_at = ?");
            values.push(json!(v));
        }
    }
    if let Some(max_usage) = body.get("max_usage").and_then(|v| v.as_i64()) {
        sets.push("max_usage = ?");
        values.push(json!(max_usage.max(1)));
    }
    if let Some(user_id) = body.get("user_id") {
        if user_id.is_null() {
            sets.push("user_id = NULL");
        } else if let Some(v) = user_id.as_i64() {
            sets.push("user_id = ?");
            values.push(json!(v));
        }
    }
    if sets.is_empty() {
        return Ok(ok(json!(true)));
    }
    sets.push("updated_at = ?");
    values.push(json!(util::now()));
    let sql = format!("UPDATE v2_gift_card_code SET {} WHERE id = ?", sets.join(", "));
    let mut query = sqlx::query(&sql);
    for value in &values {
        query = bind_value(query, value);
    }
    query.bind(id).execute(&state.db).await?;
    Ok(ok(json!(true)))
}

pub async fn gift_code_delete(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
    sqlx::query("DELETE FROM v2_gift_card_code WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn gift_codes_export(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let template_id = query.get("template_id").and_then(|v| v.parse::<i64>().ok());
    let rows: Vec<(String, i64, Option<i64>, i64)> = match template_id {
        Some(id) => sqlx::query_as(
            "SELECT code, status, expires_at, created_at FROM v2_gift_card_code WHERE template_id = ? ORDER BY id ASC",
        )
        .bind(id)
        .fetch_all(&state.db)
        .await?,
        None => sqlx::query_as(
            "SELECT code, status, expires_at, created_at FROM v2_gift_card_code ORDER BY id ASC",
        )
        .fetch_all(&state.db)
        .await?,
    };
    let mut csv = String::from("\u{feff}code,status,expires_at,created_at\n");
    for (code, status, expires, created) in rows {
        csv.push_str(&format!(
            "{},{},{},{}\n",
            code,
            status,
            expires.unwrap_or(0),
            created
        ));
    }
    Ok((axum::http::StatusCode::OK, [("content-type", "text/csv; charset=utf-8")], csv).into_response())
}

pub async fn gift_usages(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query
        .get("page")
        .or_else(|| query.get("current"))
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(1)
        .max(1);
    let page_size = query
        .get("per_page")
        .or_else(|| query.get("pageSize"))
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(20)
        .clamp(1, 500);

    let mut sql = String::from("SELECT CAST(u.id AS SIGNED), u.code_id, u.template_id, u.user_id, u.invite_user_id, u.rewards_given, u.invite_rewards, u.created_at, t.name, c.code, usr.email, u.ip_address FROM v2_gift_card_usage u LEFT JOIN v2_gift_card_template t ON t.id = u.template_id LEFT JOIN v2_gift_card_code c ON c.id = u.code_id LEFT JOIN v2_user usr ON usr.id = u.user_id WHERE 1=1");
    let mut binds: Vec<Value> = Vec::new();
    for (key, value) in &query {
        match key.as_str() {
            "template_id" | "user_id" | "code_id" => {
                if let Ok(n) = value.parse::<i64>() {
                    sql.push_str(&format!(" AND u.{key} = ?"));
                    binds.push(json!(n));
                }
            }
            "search" if !value.is_empty() => {
                sql.push_str(" AND (c.code LIKE ? OR usr.email LIKE ?)");
                binds.push(json!(util::like_pattern(value)));
                binds.push(json!(util::like_pattern(value)));
            }
            _ => {}
        }
    }
    let count_sql = sql.replace(
        "SELECT CAST(u.id AS SIGNED), u.code_id, u.template_id, u.user_id, u.invite_user_id, u.rewards_given, u.invite_rewards, u.created_at, t.name, c.code, usr.email, u.ip_address",
        "SELECT COUNT(*)",
    );
    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    for value in &binds {
        count_query = bind_value_count(count_query, value);
    }
    let total = count_query.fetch_one(&state.db).await?.0;

    sql.push_str(" ORDER BY u.id DESC LIMIT ? OFFSET ?");
    binds.push(json!(page_size));
    binds.push(json!((current - 1) * page_size));
    let mut query = sqlx::query(&sql);
    for value in &binds {
        query = bind_value(query, value);
    }
    let rows = query.fetch_all(&state.db).await?;
    use sqlx::Row;
    let mut data = Vec::new();
    for row in rows {
        let id: i64 = row.try_get(0).unwrap_or(0);
        let code_id: i64 = row.try_get(1).unwrap_or(0);
        let template_id: i64 = row.try_get(2).unwrap_or(0);
        let user_id: i64 = row.try_get(3).unwrap_or(0);
        let invite_user_id: Option<i64> = row.try_get(4).ok().flatten();
        let rewards: Option<Vec<u8>> = row.try_get(5).ok().flatten();
        let invite_rewards: Option<Vec<u8>> = row.try_get(6).ok().flatten();
        let created_at: i64 = row.try_get(7).unwrap_or(0);
        let template_name: Option<String> = row.try_get(8).ok().flatten();
        let code: Option<String> = row.try_get(9).ok().flatten();
        let email: Option<String> = row.try_get(10).ok().flatten();
        let ip_address: Option<String> = row.try_get(11).ok().flatten();
        let invite_email: Option<String> = match invite_user_id {
            Some(invite_id) => {
                sqlx::query_as("SELECT email FROM v2_user WHERE id = ?")
                    .bind(invite_id)
                    .fetch_optional(&state.db)
                    .await?
                    .map(|(email,): (String,)| email)
            }
            None => None,
        };
        let template_name_value = template_name.clone().unwrap_or_default();
        data.push(json!({
            "id": id,
            "code_id": code_id,
            "template_id": template_id,
            "template_name": template_name_value,
            "template": if template_name.is_some() { json!({"id": template_id, "name": template_name}) } else { Value::Null },
            "user_id": user_id,
            "user_email": email.clone().unwrap_or_default(),
            "invite_user_email": invite_email,
            "user": email.map(|e| json!({"id": user_id, "email": e})).unwrap_or(Value::Null),
            "invite_user_id": invite_user_id,
            "rewards_given": rewards.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
            "invite_rewards": invite_rewards.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
            "code": code,
            "ip_address": ip_address,
            "created_at": format_unix_local(created_at),
        }));
    }
    Ok(paginate(data, total, current, page_size))
}

pub async fn gift_statistics(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_gift_card_code")
        .fetch_one(&state.db)
        .await?;
    let used: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_gift_card_code WHERE status = 1")
        .fetch_one(&state.db)
        .await?;
    let disabled: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_gift_card_code WHERE status = 3")
        .fetch_one(&state.db)
        .await?;
    let usages: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_gift_card_usage")
        .fetch_one(&state.db)
        .await?;
    let templates: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_gift_card_template")
        .fetch_one(&state.db)
        .await?;
    Ok(ok(json!({
        "total": total.0,
        "used": used.0,
        "unused": total.0 - used.0 - disabled.0,
        "disabled": disabled.0,
        "usage_count": usages.0,
        "template_count": templates.0,
        "usage_rate": if total.0 > 0 { used.0 as f64 / total.0 as f64 } else { 0.0 }
    })))
}


pub async fn gift_types() -> ApiResult<Response> {
    Ok(ok(json!({
        "1": "余额", "2": "有效期", "3": "流量", "4": "重置包", "5": "套餐",
        "6": "组合", "7": "盲盒", "8": "任务", "9": "等级", "10": "节日"
    })))
}

// ---------------------------------------------------------------------------
// Withdrawals
// ---------------------------------------------------------------------------

pub async fn withdrawals_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(10).max(1);
    let status = query.get("status").and_then(|v| v.parse::<i64>().ok());
    let total: (i64,) = match status {
        Some(status) => sqlx::query_as("SELECT COUNT(*) FROM v2_withdrawal WHERE status = ?")
            .bind(status)
            .fetch_one(&state.db)
            .await?,
        None => sqlx::query_as("SELECT COUNT(*) FROM v2_withdrawal")
            .fetch_one(&state.db)
            .await?,
    };
    let rows: Vec<(i64, i64, String, String, i64, i64, Option<String>, Option<i64>, i64, i64)> = match status {
        Some(status) => sqlx::query_as(
            "SELECT id, user_id, withdraw_method, withdraw_account, amount, status, remark, operator_id, created_at, updated_at FROM v2_withdrawal WHERE status = ? ORDER BY id DESC LIMIT ? OFFSET ?",
        )
        .bind(status)
        .bind(page_size)
        .bind((current - 1) * page_size)
        .fetch_all(&state.db)
        .await?,
        None => sqlx::query_as(
            "SELECT id, user_id, withdraw_method, withdraw_account, amount, status, remark, operator_id, created_at, updated_at FROM v2_withdrawal ORDER BY id DESC LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind((current - 1) * page_size)
        .fetch_all(&state.db)
        .await?,
    };
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, user_id, method, account, amount, status, remark, operator, created, updated)| {
            json!({
                "id": id, "user_id": user_id, "withdraw_method": method,
                "withdraw_account": account, "amount": amount, "status": status,
                "remark": remark, "operator_id": operator,
                "created_at": created, "updated_at": updated,
            })
        })
        .collect();
    Ok(paginate(data, total.0, current, page_size))
}

pub async fn withdrawal_confirm(
    State(state): State<SharedState>,
    AdminUser(admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("UPDATE v2_withdrawal SET status = 1, operator_id = ?, updated_at = ? WHERE id = ?")
        .bind(admin.id)
        .bind(util::now())
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn withdrawal_close(
    State(state): State<SharedState>,
    AdminUser(admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let remark = str_field(&body, "remark");
    let mut tx = state.db.begin().await?;
    let row: Option<(i64, i64)> =
        sqlx::query_as("SELECT amount, status FROM v2_withdrawal WHERE id = ? FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?;
    if let Some((amount, status)) = row {
        if status == 0 {
            sqlx::query("UPDATE v2_user SET commission_balance = commission_balance + ? WHERE id = (SELECT user_id FROM v2_withdrawal WHERE id = ?)")
                .bind(amount)
                .bind(id)
                .execute(&mut *tx)
                .await?;
        }
        sqlx::query("UPDATE v2_withdrawal SET status = 2, remark = ?, operator_id = ?, updated_at = ? WHERE id = ?")
            .bind(remark)
            .bind(admin.id)
            .bind(util::now())
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(ok(json!(true)))
}

pub async fn withdrawal_messages(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> ApiResult<Response> {
    let rows: Vec<(i64, i64, String, i64, Option<i64>)> = sqlx::query_as(
        "SELECT id, user_id, message, is_admin, CAST(UNIX_TIMESTAMP(created_at) AS SIGNED) FROM withdrawal_messages WHERE withdrawal_id = ? ORDER BY id ASC",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;
    Ok(ok(Value::Array(
        rows.into_iter()
            .map(|(id, user_id, message, is_admin, created)| {
                json!({"id": id, "user_id": user_id, "message": message, "is_admin": is_admin != 0, "created_at": created})
            })
            .collect(),
    )))
}

pub async fn withdrawal_reply(
    State(state): State<SharedState>,
    AdminUser(admin): AdminUser,
    axum::extract::Path(id): axum::extract::Path<i64>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let message = str_field(&body, "message").unwrap_or_default();
    sqlx::query("INSERT INTO withdrawal_messages (withdrawal_id, user_id, message, is_admin, created_at, updated_at) VALUES (?, ?, ?, 1, NOW(), NOW())")
        .bind(id)
        .bind(admin.id)
        .bind(message)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

// ---------------------------------------------------------------------------
// Stats
// ---------------------------------------------------------------------------

fn start_of_day(now: i64) -> i64 {
    now - now % 86400
}

fn local_month_start(now: i64, offset_months: i32) -> i64 {
    let dt = chrono::Local.timestamp_opt(now, 0).single().unwrap_or_else(chrono::Local::now);
    let total = dt.year() * 12 + dt.month0() as i32 + offset_months;
    let year = total.div_euclid(12);
    let month = total.rem_euclid(12) as u32 + 1;
    chrono::Local
        .with_ymd_and_hms(year, month, 1, 0, 0, 0)
        .single()
        .map(|d| d.timestamp())
        .unwrap_or(now)
}

async fn traffic_sums(
    state: &SharedState,
    start: i64,
    end: i64,
) -> ApiResult<(i64, i64)> {
    let row: (i64, i64) = sqlx::query_as(
        "SELECT CAST(COALESCE(SUM(u),0) AS SIGNED), CAST(COALESCE(SUM(d),0) AS SIGNED) FROM v2_stat_server WHERE record_at >= ? AND record_at < ?",
    )
    .bind(start)
    .bind(end)
    .fetch_one(&state.db)
    .await?;
    Ok(row)
}

async fn overview_data(state: &SharedState) -> ApiResult<Value> {
    let now = util::now();
    let today = start_of_day(now);
    let yesterday = today - 86400;
    let month_start = local_month_start(now, 0);
    let last_month_start = local_month_start(now, -1);
    let two_months_start = local_month_start(now, -2);

    let income = |start: i64, end: i64| {
        let db = state.db.clone();
        async move {
            let row: (i64,) = sqlx::query_as(
                "SELECT CAST(COALESCE(SUM(total_amount),0) AS SIGNED) FROM v2_order WHERE created_at >= ? AND created_at < ? AND status NOT IN (0,2)",
            )
            .bind(start)
            .bind(end)
            .fetch_one(&db)
            .await
            .unwrap_or((0,));
            row.0
        }
    };

    let today_income = income(today, now + 1).await;
    let yesterday_income = income(yesterday, today).await;
    let month_income = income(month_start, now + 1).await;
    let last_month_income = income(last_month_start, month_start).await;
    let two_months_income = income(two_months_start, last_month_start).await;

    let commission = |start: i64, end: i64| {
        let db = state.db.clone();
        async move {
            let row: (i64,) = sqlx::query_as(
                "SELECT CAST(COALESCE(SUM(get_amount),0) AS SIGNED) FROM v2_commission_log WHERE created_at >= ? AND created_at < ?",
            )
            .bind(start)
            .bind(end)
            .fetch_one(&db)
            .await
            .unwrap_or((0,));
            row.0
        }
    };
    let month_commission = commission(month_start, now + 1).await;
    let last_month_commission = commission(last_month_start, month_start).await;
    let two_months_commission = commission(two_months_start, last_month_start).await;

    let (today_u, today_d) = traffic_sums(state, today, now + 1).await?;
    let (month_u, month_d) = traffic_sums(state, month_start, now + 1).await?;
    let (total_u, total_d) = traffic_sums(state, 0, now + 1).await?;

    let online_users: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM v2_user WHERE t >= ?")
            .bind(now - 600)
            .fetch_one(&state.db)
            .await
            .unwrap_or((0,));
    let online_devices: (i64,) =
        sqlx::query_as("SELECT CAST(COALESCE(SUM(online_count),0) AS SIGNED) FROM v2_user WHERE t >= ?")
            .bind(now - 600)
            .fetch_one(&state.db)
            .await
            .unwrap_or((0,));
    let online_nodes = state.ws.connected_node_ids().len() as i64;

    let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_user")
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));
    let active_users: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM v2_user WHERE expired_at >= ? OR expired_at IS NULL",
    )
    .bind(now)
    .fetch_one(&state.db)
    .await
    .unwrap_or((0,));
    let month_new: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM v2_user WHERE created_at >= ? AND created_at < ?")
            .bind(month_start)
            .bind(now + 1)
            .fetch_one(&state.db)
            .await
            .unwrap_or((0,));
    let last_month_new: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM v2_user WHERE created_at >= ? AND created_at < ?")
            .bind(last_month_start)
            .bind(month_start)
            .fetch_one(&state.db)
            .await
            .unwrap_or((0,));
    let tickets: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_ticket WHERE status = 0")
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));
    let commission_pending: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM v2_order WHERE commission_status = 0 AND invite_user_id IS NOT NULL AND status = 3 AND commission_balance > 0",
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or((0,));

    let pct = |current: i64, previous: i64| -> f64 {
        if previous > 0 {
            ((current - previous) as f64 / previous as f64 * 1000.0).round() / 10.0
        } else {
            0.0
        }
    };

    Ok(json!({
        "todayIncome": today_income,
        "dayIncomeGrowth": pct(today_income, yesterday_income),
        "currentMonthIncome": month_income,
        "lastMonthIncome": last_month_income,
        "monthIncomeGrowth": pct(month_income, last_month_income),
        "lastMonthIncomeGrowth": pct(last_month_income, two_months_income),
        "currentMonthCommissionPayout": month_commission,
        "lastMonthCommissionPayout": last_month_commission,
        "commissionGrowth": pct(last_month_commission, two_months_commission),
        "commissionPendingTotal": commission_pending.0,
        "currentMonthNewUsers": month_new.0,
        "totalUsers": total_users.0,
        "activeUsers": active_users.0,
        "userGrowth": pct(month_new.0, last_month_new.0),
        "onlineUsers": online_users.0,
        "onlineDevices": online_devices.0.to_string(),
        "onlineNodes": online_nodes,
        "ticketPendingTotal": tickets.0,
        "todayTraffic": {
            "upload": today_u,
            "download": today_d,
            "total": today_u + today_d,
        },
        "monthTraffic": {
            "upload": month_u,
            "download": month_d,
            "total": month_u + month_d,
        },
        "totalTraffic": {
            "upload": total_u,
            "download": total_d,
            "total": total_u + total_d,
        },
    }))
}

pub async fn stat_stats(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    Ok(ok(overview_data(&state).await?))
}

pub async fn stat_override(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let data = overview_data(&state).await?;
    Ok(ok(json!({
        "month_income": data.get("currentMonthIncome"),
        "month_register_total": data.get("currentMonthNewUsers"),
        "ticket_pending_total": data.get("ticketPendingTotal"),
        "commission_pending_total": data.get("commissionPendingTotal"),
        "day_income": data.get("todayIncome"),
        "last_month_income": data.get("lastMonthIncome"),
        "commission_month_payout": data.get("currentMonthCommissionPayout"),
        "commission_last_month_payout": data.get("lastMonthCommissionPayout"),
        "online_nodes": data.get("onlineNodes"),
        "online_devices": data.get("onlineDevices"),
        "online_users": data.get("onlineUsers"),
        "today_traffic": data.get("todayTraffic"),
        "month_traffic": data.get("monthTraffic"),
        "total_traffic": data.get("totalTraffic"),
    })))
}

pub async fn stat_order(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let start = query
        .get("start_date")
        .cloned()
        .unwrap_or_else(|| "1970-01-01".into());
    let end = query
        .get("end_date")
        .cloned()
        .unwrap_or_else(|| "2100-01-01".into());
    let ty = query.get("type").cloned();

    let rows: Vec<(String, i64, i64, i64, i64)> = sqlx::query_as(
        "SELECT CAST(DATE(FROM_UNIXTIME(created_at)) AS CHAR) AS day, CAST(COALESCE(SUM(total_amount),0) AS SIGNED), COUNT(*), CAST(COALESCE(SUM(CASE WHEN commission_status = 2 THEN commission_balance ELSE 0 END),0) AS SIGNED), CAST(COALESCE(SUM(CASE WHEN commission_status = 2 THEN 1 ELSE 0 END),0) AS SIGNED) FROM v2_order WHERE status = 3 AND DATE(FROM_UNIXTIME(created_at)) BETWEEN ? AND ? GROUP BY day ORDER BY day ASC",
    )
    .bind(&start)
    .bind(&end)
    .fetch_all(&state.db)
    .await?;

    let mut list = Vec::new();
    let mut paid_total = 0i64;
    let mut paid_count = 0i64;
    let mut commission_total = 0i64;
    let mut commission_count = 0i64;
    for (date, total, count, c_total, c_count) in rows {
        paid_total += total;
        paid_count += count;
        commission_total += c_total;
        commission_count += c_count;
        if let Some(ty) = &ty {
            let value = match ty.as_str() {
                "paid_total" => total,
                "paid_count" => count,
                "commission_total" => c_total,
                "commission_count" => c_count,
                _ => total,
            };
            list.push(json!({
                "date": date,
                "value": value,
                "type": ty,
            }));
        } else {
            list.push(json!({
                "date": date,
                "paid_total": total,
                "paid_count": count,
                "commission_total": c_total,
                "commission_count": c_count,
                "avg_order_amount": if count > 0 { (total as f64 / count as f64 * 100.0).round() / 100.0 } else { 0.0 },
                "avg_commission_amount": if c_count > 0 { (c_total as f64 / c_count as f64 * 100.0).round() / 100.0 } else { 0.0 },
            }));
        }
    }

    Ok(ok(json!({
        "list": list,
        "summary": {
            "paid_total": paid_total,
            "paid_count": paid_count,
            "commission_total": commission_total,
            "commission_count": commission_count,
            "start_date": start,
            "end_date": end,
            "avg_paid_amount": if paid_count > 0 { (paid_total as f64 / paid_count as f64 * 100.0).round() / 100.0 } else { 0.0 },
            "avg_commission_amount": if commission_count > 0 { (commission_total as f64 / commission_count as f64 * 100.0).round() / 100.0 } else { 0.0 },
            "commission_rate": if paid_total > 0 { (commission_total as f64 / paid_total as f64 * 10000.0).round() / 100.0 } else { 0.0 },
        }
    })))
}

pub async fn stat_ranking(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let now = util::now();
    let start: i64 = query
        .get("start_time")
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| start_of_day(now) - 30 * 86400);
    let end: i64 = query
        .get("end_time")
        .and_then(|v| v.parse().ok())
        .unwrap_or(now + 1);
    let ty = query.get("type").cloned().unwrap_or_else(|| "user".into());
    let rows: Vec<(i64, String, i64)> = if ty == "node" {
        sqlx::query_as(
            "SELECT s.server_id, COALESCE(sv.name, ''), CAST(COALESCE(SUM(s.u + s.d),0) AS SIGNED) FROM v2_stat_server s LEFT JOIN v2_server sv ON sv.id = s.server_id WHERE s.record_at >= ? AND s.record_at < ? GROUP BY s.server_id ORDER BY 3 DESC LIMIT 20",
        )
        .bind(start)
        .bind(end)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as(
            "SELECT s.user_id, COALESCE(u.email, ''), CAST(COALESCE(SUM(s.u + s.d),0) AS SIGNED) FROM v2_stat_user s LEFT JOIN v2_user u ON u.id = s.user_id WHERE s.record_at >= ? AND s.record_at < ? GROUP BY s.user_id ORDER BY 3 DESC LIMIT 20",
        )
        .bind(start)
        .bind(end)
        .fetch_all(&state.db)
        .await?
    };
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, name, value)| {
            json!({
                "id": id.to_string(), "name": name,
                "value": util::traffic_convert(value as f64),
                "previousValue": "0 B", "change": 0,
                "timestamp": chrono::DateTime::from_timestamp(end, 0)
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_default(),
            })
        })
        .collect();
    Ok(ok(json!({"data": data})))
}

async fn server_rank(state: &SharedState, start: i64, end: i64) -> ApiResult<Value> {
    let rows: Vec<(i64, String, String, i64, i64)> = sqlx::query_as(
        "SELECT CAST(sv.id AS SIGNED), sv.name, sv.type, CAST(COALESCE(SUM(s.u),0) AS SIGNED), CAST(COALESCE(SUM(s.d),0) AS SIGNED) FROM v2_stat_server s JOIN v2_server sv ON sv.id = s.server_id WHERE s.record_at >= ? AND s.record_at < ? AND s.record_type = 'd' GROUP BY sv.id ORDER BY (SUM(s.u)+SUM(s.d)) DESC",
    )
    .bind(start)
    .bind(end)
    .fetch_all(&state.db)
    .await?;
    Ok(Value::Array(
        rows.into_iter()
            .map(|(id, name, ty, u, d)| {
                json!({
                    "server_name": name,
                    "server_id": id,
                    "server_type": ty,
                    "u": u,
                    "d": d,
                    "total": u + d,
                })
            })
            .collect(),
    ))
}

pub async fn stat_record(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let now = util::now();
    let start = local_month_start(now, -1);
    let end = now + 1;
    let ty = query.get("type").cloned().unwrap_or_else(|| "paid_total".into());
    let rows: Vec<(String, i64, i64, i64)> = sqlx::query_as(
        "SELECT CAST(DATE(FROM_UNIXTIME(created_at)) AS CHAR) AS day, CAST(COALESCE(SUM(total_amount),0) AS SIGNED), COUNT(*), CAST(COALESCE(SUM(commission_balance),0) AS SIGNED) FROM v2_order WHERE created_at >= ? AND created_at < ? AND status = 3 GROUP BY day ORDER BY day ASC",
    )
    .bind(start)
    .bind(end)
    .fetch_all(&state.db)
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(day, total, count, commission)| {
            json!({
                "record_at": day,
                "paid_total": total,
                "paid_count": count,
                "commission_total": commission,
                "commission_count": count,
                "register_count": 0,
                "value": match ty.as_str() {
                    "paid_total" => total,
                    "commission_total" => commission,
                    "register_count" => 0,
                    _ => total,
                }
            })
        })
        .collect();
    Ok(ok(json!({"data": data})))
}

pub async fn stat_last_rank(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let now = util::now();
    Ok(ok(server_rank(&state, start_of_day(now), now + 1).await?))
}

pub async fn stat_yesterday_rank(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let now = util::now();
    let today = start_of_day(now);
    Ok(ok(server_rank(&state, today - 86400, today).await?))
}

pub async fn stat_user(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("user_id").and_then(|v| v.parse::<i64>().ok());
    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(10).max(1);
    let total: (i64,) = match id {
        Some(id) => sqlx::query_as("SELECT COUNT(*) FROM v2_stat_user WHERE user_id = ?")
            .bind(id)
            .fetch_one(&state.db)
            .await?,
        None => sqlx::query_as("SELECT COUNT(*) FROM v2_stat_user")
            .fetch_one(&state.db)
            .await?,
    };
    let rows: Vec<(i64, i64, i64, f64, i64)> = match id {
        Some(id) => sqlx::query_as(
            "SELECT user_id, u, d, CAST(server_rate AS DOUBLE), record_at FROM v2_stat_user WHERE user_id = ? ORDER BY record_at DESC LIMIT ? OFFSET ?",
        )
        .bind(id)
        .bind(page_size)
        .bind((current - 1) * page_size)
        .fetch_all(&state.db)
        .await?,
        None => sqlx::query_as(
            "SELECT user_id, u, d, CAST(server_rate AS DOUBLE), record_at FROM v2_stat_user ORDER BY record_at DESC LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind((current - 1) * page_size)
        .fetch_all(&state.db)
        .await?,
    };
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(user_id, u, d, rate, record_at)| {
            json!({"user_id": user_id, "u": u, "d": d, "server_rate": rate, "record_at": record_at})
        })
        .collect();
    Ok(paginate(data, total.0, current, page_size))
}


// System
// ---------------------------------------------------------------------------

pub async fn system_status(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let db_ok = sqlx::query("SELECT 1").fetch_one(&state.db).await.is_ok();
    let users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_user")
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));
    Ok(ok(json!({
        "php_version": null,
        "laravel_version": null,
        "fboard_version": env!("CARGO_PKG_VERSION"),
        "runtime": "rust",
        "database": if db_ok { "ok" } else { "error" },
        "users": users.0,
        "uptime": util::now() - state.started_at,
        "memory": null,
        "cpu": null,
        "schedule": true,
        "horizon": false,
    })))
}

pub async fn system_queue_stats(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let failed: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM failed_jobs")
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));
    // The Rust core does not run Laravel jobs, so the queue is always empty.
    // Keep the Horizon-compatible shape used by the dashboard.
    Ok(ok(json!({
        "status": true,
        "wait": {},
        "processes": 0,
        "failedJobs": failed.0,
        "recentJobs": 0,
        "jobsPerMinute": 0,
        "queueWithMaxRuntime": "",
        "queueWithMaxThroughput": "",
        "periods": {"recentJobs": 1},
        "waiting": 0,
        "active": 0,
        "failed": failed.0,
        "delayed": 0
    })))
}

pub async fn system_queue_workload(AdminUser(_admin): AdminUser) -> ApiResult<Response> {
    Ok(ok(json!([])))
}

pub async fn system_failed_jobs(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let rows: Vec<(i64, String, String, Option<String>, i64)> = sqlx::query_as(
        "SELECT id, connection, queue, LEFT(exception, 500), failed_at FROM failed_jobs ORDER BY id DESC LIMIT 50",
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();
    Ok(ok(Value::Array(
        rows.into_iter()
            .map(|(id, connection, queue, exception, failed_at)| {
                json!({"id": id, "connection": connection, "queue": queue, "exception": exception, "failed_at": failed_at})
            })
            .collect(),
    )))
}

pub async fn system_audit_log(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(20).max(1);
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_admin_audit_log")
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));
    let rows: Vec<(i64, i64, String, String, String, Option<String>, Option<String>, i64)> =
        sqlx::query_as(
            "SELECT id, admin_id, action, method, uri, request_data, ip, created_at FROM v2_admin_audit_log ORDER BY id DESC LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind((current - 1) * page_size)
        .fetch_all(&state.db)
        .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, admin_id, action, method, uri, data, ip, created)| {
            json!({
                "id": id, "admin_id": admin_id, "action": action, "method": method,
                "uri": uri, "request_data": data, "ip": ip, "created_at": created,
            })
        })
        .collect();
    Ok(paginate(data, total.0, current, page_size))
}

// ---------------------------------------------------------------------------
// Traffic reset
// ---------------------------------------------------------------------------

pub async fn traffic_reset_logs(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query.get("current").and_then(|v| v.parse::<i64>().ok()).unwrap_or(1).max(1);
    let page_size = query.get("pageSize").and_then(|v| v.parse::<i64>().ok()).unwrap_or(20).max(1);
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_traffic_reset_logs")
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));
    let rows: Vec<(i64, i64, String, i64, i64, i64, i64, String)> = sqlx::query_as(
        "SELECT CAST(id AS SIGNED), user_id, reset_type, CAST(UNIX_TIMESTAMP(reset_time) AS SIGNED), old_upload, old_download, old_total, trigger_source FROM v2_traffic_reset_logs ORDER BY id DESC LIMIT ? OFFSET ?",
    )
    .bind(page_size)
    .bind((current - 1) * page_size)
    .fetch_all(&state.db)
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, user_id, reset_type, reset_time, old_u, old_d, old_total, source)| {
            json!({
                "id": id, "user_id": user_id, "reset_type": reset_type,
                "reset_time": reset_time, "old_upload": old_u, "old_download": old_d,
                "old_total": old_total, "trigger_source": source,
            })
        })
        .collect();
    Ok(paginate(data, total.0, current, page_size))
}

pub async fn traffic_reset_stats(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_traffic_reset_logs")
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));
    Ok(ok(json!({"total": total.0, "today": 0, "month": 0})))
}

pub async fn traffic_reset_user_history(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    axum::extract::Path(user_id): axum::extract::Path<i64>,
) -> ApiResult<Response> {
    let rows: Vec<(i64, String, i64, i64, i64, i64, String)> = sqlx::query_as(
        "SELECT CAST(id AS SIGNED), reset_type, CAST(UNIX_TIMESTAMP(reset_time) AS SIGNED), old_upload, old_download, old_total, trigger_source FROM v2_traffic_reset_logs WHERE user_id = ? ORDER BY id DESC LIMIT 100",
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await?;
    Ok(ok(Value::Array(
        rows.into_iter()
            .map(|(id, ty, at, u, d, total, source)| {
                json!({
                    "id": id, "reset_type": ty, "reset_time": at,
                    "old_upload": u, "old_download": d, "old_total": total,
                    "trigger_source": source,
                })
            })
            .collect(),
    )))
}

pub async fn traffic_reset_user(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let user_id = body.get("user_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let user: Option<(i64, i64)> = sqlx::query_as("SELECT u, d FROM v2_user WHERE id = ?")
        .bind(user_id)
        .fetch_optional(&state.db)
        .await?;
    let Some((u, d)) = user else {
        return Err(ApiError::not_found("用户不存在"));
    };
    let now = util::now();
    sqlx::query("UPDATE v2_user SET u = 0, d = 0, last_reset_at = ?, reset_count = reset_count + 1, updated_at = ? WHERE id = ?")
        .bind(now)
        .bind(now)
        .bind(user_id)
        .execute(&state.db)
        .await?;
    sqlx::query(
        "INSERT INTO v2_traffic_reset_logs (user_id, reset_type, reset_time, old_upload, old_download, old_total, new_upload, new_download, new_total, trigger_source, created_at, updated_at) VALUES (?, 'manual', NOW(), ?, ?, ?, 0, 0, 0, 'admin', NOW(), NOW())",
    )
    .bind(user_id)
    .bind(u)
    .bind(d)
    .bind(u + d)
    .execute(&state.db)
    .await?;
    Ok(ok(json!(true)))
}

// ---------------------------------------------------------------------------
// Plugins
// ---------------------------------------------------------------------------

pub async fn plugin_types(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let mut types: Vec<String> = state
        .plugin
        .all()
        .iter()
        .map(|plugin| plugin.meta().kind.as_str().to_string())
        .collect();
    types.sort();
    types.dedup();
    let out: Vec<Value> = types
        .into_iter()
        .map(|kind| {
            let (label, description, icon) = match kind.as_str() {
                "payment" => ("支付方式", "提供支付接口的插件", "💳"),
                "protocol" => ("协议", "提供协议或订阅输出的插件", "🔌"),
                _ => ("功能", "提供功能扩展的插件", "🔧"),
            };
            json!({"value": kind, "label": label, "description": description, "icon": icon})
        })
        .collect();
    Ok(ok(Value::Array(out)))
}

pub async fn plugins_list(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let page = query
        .get("page")
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(1)
        .max(1);
    let page_size = query
        .get("pageSize")
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(20)
        .clamp(1, 100);
    let type_filter = query.get("type").cloned().unwrap_or_default();
    let search = query
        .get("search")
        .or_else(|| query.get("q"))
        .cloned()
        .unwrap_or_default()
        .to_lowercase();

    let rows: Vec<(String, i64, Option<Vec<u8>>)> =
        sqlx::query_as("SELECT code, is_enabled, config FROM v2_plugins")
            .fetch_all(&state.db)
            .await
            .unwrap_or_default();
    let db: HashMap<String, (i64, Value)> = rows
        .into_iter()
        .map(|(code, enabled, config)| {
            (
                code.to_ascii_lowercase(),
                (
                    enabled,
                    config
                        .as_deref()
                        .and_then(|v| serde_json::from_slice(v).ok())
                        .unwrap_or_else(|| json!({})),
                ),
            )
        })
        .collect();

    let mut all = Vec::new();
    for plugin in state.plugin.all() {
        let meta = plugin.meta();
        if !type_filter.is_empty() && meta.kind.as_str() != type_filter {
            continue;
        }
        if !search.is_empty()
            && !meta.name.to_lowercase().contains(&search)
            && !meta.description.to_lowercase().contains(&search)
            && !meta.code.to_lowercase().contains(&search)
        {
            continue;
        }
        let stored = db.get(&meta.code.to_ascii_lowercase());
        let default_enabled =
            meta.builtin && matches!(meta.kind, crate::plugin::PluginKind::Protocol | crate::plugin::PluginKind::Output);
        let enabled = stored
            .map(|(enabled, _)| *enabled != 0)
            .unwrap_or(default_enabled);
        let config = stored
            .map(|(_, config)| config.clone())
            .unwrap_or_else(|| json!({}));
        let path = meta.path.as_ref().map(|p| p.display().to_string()).unwrap_or_default();
        let has_static = meta
            .path
            .as_ref()
            .map(|path| path.join("public").is_dir())
            .unwrap_or(false);
        all.push(json!({
            "name": meta.name,
            "code": meta.code,
            "type": meta.kind.as_str(),
            "version": meta.version,
            "description": meta.description,
            "author": meta.author,
            "is_installed": true,
            "is_enabled": enabled,
            "config": config,
            "is_core": meta.builtin,
            "is_protected": meta.builtin,
            "can_be_deleted": !meta.builtin,
            "builtin": meta.builtin,
            "has_config": meta.config_fields.as_object().map(|v| !v.is_empty()).unwrap_or(false),
            "need_upgrade": false,
            "has_readme": !meta.readme.trim().is_empty(),
            "has_static_files": has_static,
            "admin_menus": Value::Null,
            "admin_crud": Value::Null,
            "actions": [],
            "path": path,
        }));
    }

    let total = all.len() as i64;
    let start = ((page - 1) * page_size).max(0) as usize;
    let items: Vec<Value> = all
        .into_iter()
        .skip(start)
        .take(page_size as usize)
        .collect();
    Ok(paginate(items, total, page, page_size))
}

pub async fn plugin_enable(
    State(state): State<SharedState>,
    admin: AdminUser,
    body: Json<Value>,
) -> ApiResult<Response> {
    plugin_set_enabled_impl(&state, admin, body, true).await
}

pub async fn plugin_disable(
    State(state): State<SharedState>,
    admin: AdminUser,
    body: Json<Value>,
) -> ApiResult<Response> {
    plugin_set_enabled_impl(&state, admin, body, false).await
}

async fn plugin_set_enabled_impl(
    state: &SharedState,
    _admin: AdminUser,
    body: Json<Value>,
    enabled_default: bool,
) -> ApiResult<Response> {
    let code = str_field(&body, "code")
        .or_else(|| str_field(&body, "plugin"))
        .ok_or_else(|| ApiError::bad_request("插件代码不能为空"))?
        .to_ascii_lowercase();
    let enabled = body
        .get("is_enabled")
        .or_else(|| body.get("enabled"))
        .and_then(|v| v.as_bool())
        .unwrap_or(enabled_default);
    let plugin = state
        .plugin
        .get(&code)
        .ok_or_else(|| ApiError::not_found("插件不存在"))?;
    let meta = plugin.meta();

    let exists: Option<(u64,)> = sqlx::query_as("SELECT id FROM v2_plugins WHERE code = ?")
        .bind(&meta.code)
        .fetch_optional(&state.db)
        .await?;
    if exists.is_none() {
        sqlx::query("INSERT INTO v2_plugins (name, code, type, version, is_enabled, config, installed_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, '{}', NOW(), NOW(), NOW())")
            .bind(&meta.name)
            .bind(&meta.code)
            .bind(meta.kind.as_str())
            .bind(&meta.version)
            .bind(enabled as i64)
            .execute(&state.db)
            .await?;
    } else {
        sqlx::query("UPDATE v2_plugins SET is_enabled = ?, updated_at = NOW() WHERE code = ?")
            .bind(enabled as i64)
            .bind(&meta.code)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn plugin_config_get(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let code = query.get("code").cloned().unwrap_or_default();
    let row: Option<(Option<Vec<u8>>,)> =
        sqlx::query_as("SELECT config FROM v2_plugins WHERE code = ?")
            .bind(&code)
            .fetch_optional(&state.db)
            .await?;
    let config = row
        .and_then(|(c,)| c)
        .and_then(|c| serde_json::from_slice::<Value>(&c).ok())
        .unwrap_or_else(|| json!({}));
    let form = state.plugin.config_fields(&code);
    Ok(ok(json!({
        "config": config,
        "form": form,
        "code": code,
    })))
}

pub async fn plugin_config_update(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let code = str_field(&body, "code")
        .ok_or_else(|| ApiError::bad_request("插件代码不能为空"))?
        .to_ascii_lowercase();
    let config = body
        .get("config")
        .or_else(|| body.get("data"))
        .cloned()
        .unwrap_or_else(|| json!({}));
    let exists: Option<(u64,)> = sqlx::query_as("SELECT id FROM v2_plugins WHERE code = ?")
        .bind(&code)
        .fetch_optional(&state.db)
        .await?;
    if exists.is_some() {
        sqlx::query("UPDATE v2_plugins SET config = ?, updated_at = NOW() WHERE code = ?")
            .bind(config.to_string())
            .bind(&code)
            .execute(&state.db)
            .await?;
    } else {
        let plugin = state
            .plugin
            .get(&code)
            .ok_or_else(|| ApiError::not_found("插件不存在"))?;
        let meta = plugin.meta();
        sqlx::query("INSERT INTO v2_plugins (name, code, type, version, is_enabled, config, installed_at, created_at, updated_at) VALUES (?, ?, ?, ?, 1, ?, NOW(), NOW(), NOW())")
            .bind(&meta.name)
            .bind(&meta.code)
            .bind(meta.kind.as_str())
            .bind(&meta.version)
            .bind(config.to_string())
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn plugin_readme(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let code = query.get("code").cloned().unwrap_or_default();
    let content = state.plugin.readme(&code);
    Ok(ok(json!({"content": content})))
}

pub async fn plugin_ui(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let settings = state.settings().await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = crate::plugin::PluginContext::new(&state.db, &settings, &app_url, json!({}));
    Ok(ok(state.plugin.ui_extensions(&ctx).await))
}

pub async fn plugin_ui_nav(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let settings = state.settings().await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = crate::plugin::PluginContext::new(&state.db, &settings, &app_url, json!({}));
    Ok(ok(state.plugin.ui_navigation(&ctx).await))
}

pub async fn plugin_action(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let code = str_field(&body, "code")
        .or_else(|| str_field(&body, "plugin"))
        .ok_or_else(|| ApiError::bad_request("插件代码不能为空"))?;
    let action = str_field(&body, "action").ok_or_else(|| ApiError::bad_request("动作不能为空"))?;
    let params = body.get("params").cloned().unwrap_or_else(|| json!({}));
    let settings = state.settings().await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let config = state.plugin.config(&state.db, &code).await;
    let ctx = crate::plugin::PluginContext::new(&state.db, &settings, &app_url, config);
    let result = state
        .plugin
        .execute_action(&code, &action, params, &ctx)
        .await
        .unwrap_or_else(|| json!({"success": true}));
    Ok(ok(result))
}

pub async fn plugin_upload(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    mut multipart: Multipart,
) -> ApiResult<Response> {
    let mut data: Option<Vec<u8>> = None;
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            let bytes = field
                .bytes()
                .await
                .map_err(|_| ApiError::bad_request("上传失败"))?;
            data = Some(bytes.to_vec());
            break;
        }
    }
    let Some(data) = data else {
        return Err(ApiError::bad_request("请选择插件压缩包"));
    };
    let dir = state
        .cfg
        .plugin_dirs
        .last()
        .cloned()
        .unwrap_or_else(|| PathBuf::from("plugins"));
    tokio::fs::create_dir_all(&dir).await.ok();
    let archive_path = dir.join(format!("upload_{}.zip", util::random_chars(8)));
    tokio::fs::write(&archive_path, &data)
        .await
        .map_err(|e| ApiError::server(format!("保存失败: {e}")))?;

    let result = tokio::task::spawn_blocking({
        let archive_path = archive_path.clone();
        let dir = dir.clone();
        move || extract_zip(&archive_path, &dir)
    })
    .await
    .map_err(|e| ApiError::server(format!("解压失败: {e}")))?;
    let _ = tokio::fs::remove_file(&archive_path).await;
    result.map_err(|e| ApiError::bad_request(format!("插件包无效: {e}")))?;
    state.plugin.reload();
    state.publish_invalidation("plugins.reloaded");
    Ok(ok(json!(true)))
}

fn extract_zip(path: &PathBuf, dest: &PathBuf) -> anyhow::Result<()> {
    let file = std::fs::File::open(path)?;
    let mut zip = zip::ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let Some(name) = entry.enclosed_name() else {
            continue;
        };
        let name_str = name.to_string_lossy().to_lowercase();
        if name_str.contains("..") {
            continue;
        }
        let allowed = [
            ".lua", ".json", ".js", ".css", ".html", ".htm", ".svg", ".png", ".jpg",
            ".jpeg", ".gif", ".webp", ".woff", ".woff2", ".ttf", ".eot", ".md", ".txt",
            ".yaml", ".yml", ".map", ".ico",
        ];
        if !allowed.iter().any(|ext| name_str.ends_with(ext)) {
            continue;
        }
        let out_path = dest.join(&*name);
        if entry.is_dir() {
            std::fs::create_dir_all(&out_path)?;
            continue;
        }
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut out = std::fs::File::create(&out_path)?;
        std::io::copy(&mut entry, &mut out)?;
    }
    Ok(())
}

pub async fn plugin_delete(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let code = str_field(&body, "code")
        .ok_or_else(|| ApiError::bad_request("插件代码不能为空"))?
        .to_ascii_lowercase();
    let plugin = state
        .plugin
        .get(&code)
        .ok_or_else(|| ApiError::not_found("插件不存在"))?;
    let meta = plugin.meta();
    if meta.builtin {
        return Err(ApiError::bad_request("内置插件不能删除"));
    }
    let Some(path) = meta.path else {
        return Err(ApiError::bad_request("插件路径不存在"));
    };
    tokio::fs::remove_dir_all(&path).await.ok();
    sqlx::query("DELETE FROM v2_plugins WHERE code = ?")
        .bind(&meta.code)
        .execute(&state.db)
        .await?;
    state.plugin.reload();
    state.publish_invalidation("plugins.reloaded");
    Ok(ok(json!(true)))
}

pub async fn plugin_install(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    plugin_set_enabled_impl(
        &state,
        AdminUser::empty(),
        Json(json!({"code": str_field(&body, "code")})),
        true,
    )
    .await
}

pub async fn plugin_uninstall(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let code = str_field(&body, "code").ok_or_else(|| ApiError::bad_request("插件代码不能为空"))?;
    sqlx::query("UPDATE v2_plugins SET is_enabled = 0, updated_at = NOW() WHERE code = ?")
        .bind(code)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn plugin_upgrade(
    State(_state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    Ok(ok(json!(true)))
}

pub async fn plugin_static_files(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let code = query.get("code").cloned().unwrap_or_default();
    let Some(plugin) = state.plugin.get(&code) else {
        return Ok(ok(json!([])));
    };
    let Some(path) = plugin.meta().path else {
        return Ok(ok(json!([])));
    };
    let public = path.join("public");
    let mut files = Vec::new();
    collect_static_files(&public, &public, &code, &mut files, 0);
    Ok(ok(Value::Array(files)))
}

fn collect_static_files(
    root: &Path,
    dir: &Path,
    code: &str,
    out: &mut Vec<Value>,
    depth: usize,
) {
    if depth > 4 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_static_files(root, &path, code, out, depth + 1);
            continue;
        }
        let rel = path.strip_prefix(root).unwrap_or(&path);
        out.push(json!({
            "name": rel.display().to_string(),
            "url": format!("/plugins/{code}/{}", rel.display()),
        }));
    }
}

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

pub async fn gift_generate_codes(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let template_id = body.get("template_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let count = body
        .get("count")
        .or_else(|| body.get("quantity"))
        .and_then(|v| v.as_i64())
        .unwrap_or(1)
        .clamp(1, 1000);
    let expires_at = body.get("expires_at").and_then(|v| v.as_i64());
    let max_usage = body.get("max_usage").and_then(|v| v.as_i64()).unwrap_or(1).max(1);
    let batch_id = util::random_chars(16);
    let now = util::now();
    let mut codes = Vec::new();
    for _ in 0..count {
        let code = util::random_chars(16).to_uppercase();
        sqlx::query(
            "INSERT INTO v2_gift_card_code (template_id, code, batch_id, status, usage_count, max_usage, expires_at, created_at, updated_at) VALUES (?, ?, ?, 0, 0, ?, ?, ?, ?)",
        )
        .bind(template_id)
        .bind(&code)
        .bind(&batch_id)
        .bind(max_usage)
        .bind(expires_at)
        .bind(now)
        .bind(now)
        .execute(&state.db)
        .await?;
        codes.push(code);
    }
    Ok(ok(json!({"codes": codes, "batch_id": batch_id})))
}




#[cfg(test)]
mod plugin_upload_tests {
    use super::extract_zip;
    use zip::write::SimpleFileOptions;

    #[test]
    fn extracts_lua_plugin_and_rejects_php() {
        let root = std::env::temp_dir().join(format!(
            "fboard-upload-{}-{}",
            std::process::id(),
            crate::util::random_chars(6)
        ));
        let archive = root.join("plugin.zip");
        let dest = root.join("plugins");
        std::fs::create_dir_all(&dest).unwrap();
        {
            let file = std::fs::File::create(&archive).unwrap();
            let mut zip = zip::ZipWriter::new(file);
            let opts = SimpleFileOptions::default();
            zip.start_file("Demo/plugin.json", opts).unwrap();
            use std::io::Write;
            zip.write_all(br#"{"code":"demo","name":"Demo","type":"feature"}"#).unwrap();
            zip.start_file("Demo/init.lua", opts).unwrap();
            zip.write_all(b"function handle() return true end").unwrap();
            zip.start_file("Demo/evil.php", opts).unwrap();
            zip.write_all(b"<?php echo 1; ?>").unwrap();
            zip.finish().unwrap();
        }
        extract_zip(&archive, &dest).unwrap();
        assert!(dest.join("Demo/plugin.json").is_file());
        assert!(dest.join("Demo/init.lua").is_file());
        assert!(!dest.join("Demo/evil.php").exists());
        let _ = std::fs::remove_dir_all(root);
    }
}
