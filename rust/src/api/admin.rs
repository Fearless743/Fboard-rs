use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;
use serde_json::{json, Map, Value};

use crate::error::{ApiError, ApiResult};
use crate::models::{PlanRow, ServerRow};
use crate::plugin::PluginContext;
use crate::response::{ok, paginate, str_field};
use crate::state::{AdminUser, SharedState};
use crate::util;

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

pub async fn config_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let s = state.settings().await;
    if let Some(key) = query.get("key") {
        if key == "subscribe_template" {
            if let Some(name) = query.get("name") {
                let field = format!("subscribe_template_{}", name.to_lowercase());
                let content: Option<(Option<String>,)> =
                    sqlx::query_as("SELECT content FROM v2_subscribe_templates WHERE name = ? LIMIT 1")
                        .bind(name.to_lowercase())
                        .fetch_optional(&state.db)
                        .await?;
                let value = content.and_then(|(c,)| c).unwrap_or_default();
                return Ok(ok(json!({
                    "subscribe_template": { field: value }
                })));
            }
        }
        let mut map = Map::new();
        map.insert(key.clone(), s.value(key));
        return Ok(ok(Value::Object(map)));
    }
    Ok(ok(s.public_map()))
}

pub async fn config_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let payload = body.get("data").cloned().unwrap_or(body);
    crate::settings::Settings::save(&state.db, &payload)
        .await
        .map_err(|e| ApiError::server(format!("保存失败: {e}")))?;
    state.refresh_settings().await;
    Ok(ok(json!(true)))
}

pub async fn config_email_template(
    State(_state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(_query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    // Mirrors ConfigController::getEmailTemplate(): the list of view themes.
    Ok(ok(json!(["default"])))
}

pub async fn config_set_telegram_webhook(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let bot_token = str_field(&body, "bot_token").unwrap_or_default();
    let webhook_url = str_field(&body, "webhook_url").unwrap_or_default();
    if bot_token.is_empty() || webhook_url.is_empty() {
        return Err(ApiError::bad_request("参数错误"));
    }
    let url = format!(
        "https://api.telegram.org/bot{bot_token}/setWebhook?url={}",
        urlencoding::encode(&webhook_url)
    );
    let _ = reqwest::get(url).await;
    Ok(ok(json!(true)))
}

pub async fn config_test_mail(
    State(state): State<SharedState>,
    AdminUser(admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let settings = state.settings().await;
    let to = str_field(&body, "to")
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| admin.email.clone());
    if to.is_empty() {
        return Err(ApiError::bad_request("收件人不能为空"));
    }
    let app_name = settings.str("app_name", "Fboard");
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let subject = str_field(&body, "subject")
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "This is fboard test email".into());
    let content = str_field(&body, "content")
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "This is fboard test email".into());
    let mut vars = HashMap::new();
    vars.insert("name".into(), app_name);
    vars.insert("url".into(), app_url);
    vars.insert("content".into(), crate::mail::text_to_html(&content));
    crate::mail::send_template(&state.db, &settings, &to, "notify", &subject, &vars)
        .await
        .map_err(|e| ApiError::server(format!("发送失败: {e}")))?;
    Ok(ok(json!(true)))
}

// ---------------------------------------------------------------------------
// Mail templates
// ---------------------------------------------------------------------------

pub async fn mail_templates(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let settings = state.settings().await;
    let app_name = settings.str("app_name", "Fboard");

    if let Some(name) = query.get("name") {
        let meta = crate::mail::template_meta(name)
            .ok_or_else(|| ApiError::not_found("模板不存在"))?;
        let row: Option<(String, String, Option<i64>)> = sqlx::query_as(
            "SELECT subject, content, CAST(UNIX_TIMESTAMP(updated_at) AS SIGNED) FROM v2_mail_templates WHERE name = ? LIMIT 1",
        )
        .bind(name)
        .fetch_optional(&state.db)
        .await?;
        let customized = row.is_some();
        let (subject, content) = match row {
            Some((subject, content, _)) => (subject, content),
            None => (
                crate::mail::default_subject(name, &app_name),
                crate::mail::default_content(name),
            ),
        };
        return Ok(ok(json!({
            "name": meta.name,
            "label": meta.label,
            "required_vars": meta.required_vars,
            "optional_vars": meta.optional_vars,
            "customized": customized,
            "subject": subject,
            "content": content,
        })));
    }

    let rows: Vec<(String, String, Option<i64>)> = sqlx::query_as(
        "SELECT name, subject, CAST(UNIX_TIMESTAMP(updated_at) AS SIGNED) FROM v2_mail_templates",
    )
    .fetch_all(&state.db)
    .await?;
    let mut by_name: HashMap<String, (String, Option<i64>)> = HashMap::new();
    for (name, subject, updated) in rows {
        by_name.insert(name, (subject, updated));
    }
    let list: Vec<Value> = crate::mail::TEMPLATES
        .iter()
        .map(|meta| {
            let stored = by_name.get(meta.name);
            json!({
                "name": meta.name,
                "label": meta.label,
                "customized": stored.is_some(),
                "subject": stored.map(|v| v.0.clone()),
                "updated_at": stored.and_then(|v| v.1),
            })
        })
        .collect();
    Ok(ok(Value::Array(list)))
}

pub async fn mail_template_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    let subject = str_field(&body, "subject").unwrap_or_default();
    let content = str_field(&body, "content").unwrap_or_default();
    if crate::mail::template_meta(&name).is_none() {
        return Err(ApiError::not_found("模板不存在"));
    }
    if subject.trim().is_empty() || content.trim().is_empty() {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "主题和内容不能为空",
        ));
    }
    let errors = crate::mail::validate_content(&name, &content);
    if !errors.is_empty() {
        return Err(ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            errors.join("; "),
        ));
    }
    let existing: Option<(i64,)> =
        sqlx::query_as("SELECT id FROM v2_mail_templates WHERE name = ? LIMIT 1")
            .bind(&name)
            .fetch_optional(&state.db)
            .await?;
    if let Some((id,)) = existing {
        sqlx::query("UPDATE v2_mail_templates SET subject = ?, content = ?, updated_at = NOW() WHERE id = ?")
            .bind(subject)
            .bind(content)
            .bind(id)
            .execute(&state.db)
            .await?;
    } else {
        sqlx::query("INSERT INTO v2_mail_templates (name, subject, content, created_at, updated_at) VALUES (?, ?, ?, NOW(), NOW())")
            .bind(name)
            .bind(subject)
            .bind(content)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn mail_template_reset(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    if crate::mail::template_meta(&name).is_none() {
        return Err(ApiError::not_found("模板不存在"));
    }
    sqlx::query("DELETE FROM v2_mail_templates WHERE name = ?")
        .bind(name)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn mail_template_test(
    State(state): State<SharedState>,
    AdminUser(admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    if crate::mail::template_meta(&name).is_none() {
        return Err(ApiError::not_found("模板不存在"));
    }
    let settings = state.settings().await;
    let app_name = settings.str("app_name", "Fboard");
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let to = str_field(&body, "email")
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| admin.email.clone());
    let vars = crate::mail::test_vars(&name, &app_name, &app_url);
    let subject = crate::mail::test_subject(&name, &app_name);
    crate::mail::send_template(&state.db, &settings, &to, &name, &subject, &vars)
        .await
        .map_err(|e| ApiError::server(format!("发送失败: {e}")))?;
    Ok(ok(json!(true)))
}

// ---------------------------------------------------------------------------
// Plans
// ---------------------------------------------------------------------------

pub async fn plan_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current: i64 = query.get("current").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let page_size: i64 = query.get("pageSize").and_then(|v| v.parse().ok()).unwrap_or(10).max(1);
    let search = query.get("search").cloned().unwrap_or_default();

    let total: (i64,) = if search.is_empty() {
        sqlx::query_as("SELECT COUNT(*) FROM v2_plan")
            .fetch_one(&state.db)
            .await?
    } else {
        sqlx::query_as("SELECT COUNT(*) FROM v2_plan WHERE name LIKE ? OR pinyin_index LIKE ?")
            .bind(util::like_pattern(&search))
            .bind(util::like_pattern(&search))
            .fetch_one(&state.db)
            .await?
    };
    let rows: Vec<PlanRow> = if search.is_empty() {
        sqlx::query_as("SELECT * FROM v2_plan ORDER BY sort ASC, id ASC LIMIT ? OFFSET ?")
            .bind(page_size)
            .bind((current - 1) * page_size)
            .fetch_all(&state.db)
            .await?
    } else {
        sqlx::query_as("SELECT * FROM v2_plan WHERE name LIKE ? OR pinyin_index LIKE ? ORDER BY sort ASC, id ASC LIMIT ? OFFSET ?")
            .bind(util::like_pattern(&search))
            .bind(util::like_pattern(&search))
            .bind(page_size)
            .bind((current - 1) * page_size)
            .fetch_all(&state.db)
            .await?
    };

    let mut data = Vec::new();
    for plan in rows {
        let users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_user WHERE plan_id = ?")
            .bind(plan.id)
            .fetch_one(&state.db)
            .await?;
        let active: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM v2_user WHERE plan_id = ? AND (expired_at > ? OR expired_at IS NULL)",
        )
        .bind(plan.id)
        .bind(util::now())
        .fetch_one(&state.db)
        .await?;
        let group: Option<(i64, String)> = match plan.group_id {
            Some(id) => sqlx::query_as("SELECT id, name FROM v2_server_group WHERE id = ?")
                .bind(id)
                .fetch_optional(&state.db)
                .await?,
            None => None,
        };
        let mut item = plan_admin_json(&plan);
        item["users_count"] = json!(users.0);
        item["active_users_count"] = json!(active.0);
        item["group"] = group
            .map(|(id, name)| json!({"id": id, "name": name}))
            .unwrap_or(Value::Null);
        data.push(item);
    }
    Ok(paginate(data, total.0, current, page_size))
}

fn plan_admin_json(plan: &PlanRow) -> Value {
    json!({
        "id": plan.id,
        "group_id": plan.group_id,
        "transfer_enable": plan.transfer_enable,
        "name": plan.name,
        "prices": plan.prices_value(),
        "sell": plan.sell,
        "speed_limit": plan.speed_limit,
        "device_limit": plan.device_limit,
        "show": plan.show,
        "sort": plan.sort,
        "renew": plan.renew,
        "content": plan.content,
        "tags": plan.tags_value(),
        "reset_traffic_method": plan.reset_traffic_method,
        "capacity_limit": plan.capacity_limit,
        "created_at": plan.created_at,
        "updated_at": plan.updated_at,
    })
}

pub async fn plan_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    if name.is_empty() {
        return Err(ApiError::bad_request("订阅名称不能为空"));
    }
    let prices = body.get("prices").cloned().unwrap_or_else(|| json!({}));
    let tags = body.get("tags").cloned().unwrap_or_else(|| json!([]));
    let now = util::now();
    let group_id = body.get("group_id").and_then(|v| v.as_i64());
    let transfer_enable = body.get("transfer_enable").and_then(|v| v.as_i64()).unwrap_or(0);
    let speed_limit = body.get("speed_limit").and_then(|v| v.as_i64());
    let device_limit = body.get("device_limit").and_then(|v| v.as_i64());
    let show = body.get("show").map(|v| json_bool(v) as i64).unwrap_or(0);
    let sell = body.get("sell").map(|v| json_bool(v) as i64).unwrap_or(0);
    let renew = body.get("renew").map(|v| json_bool(v) as i64).unwrap_or(1);
    let sort = body.get("sort").and_then(|v| v.as_i64()).unwrap_or(0);
    let content = str_field(&body, "content");
    let reset_traffic_method = body.get("reset_traffic_method").and_then(|v| v.as_i64());
    let capacity_limit = body.get("capacity_limit").and_then(|v| v.as_i64());

    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM v2_plan WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;
        if existing.is_none() {
            return Err(ApiError::not_found("该订阅不存在"));
        }
        sqlx::query(
            "UPDATE v2_plan SET group_id = ?, transfer_enable = ?, name = ?, prices = ?, sell = ?, speed_limit = ?, device_limit = ?, `show` = ?, sort = ?, renew = ?, content = ?, tags = ?, reset_traffic_method = ?, capacity_limit = ?, updated_at = ? WHERE id = ?",
        )
        .bind(group_id)
        .bind(transfer_enable)
        .bind(&name)
        .bind(prices.to_string())
        .bind(sell)
        .bind(speed_limit)
        .bind(device_limit)
        .bind(show)
        .bind(sort)
        .bind(renew)
        .bind(content)
        .bind(tags.to_string())
        .bind(reset_traffic_method)
        .bind(capacity_limit)
        .bind(now)
        .bind(id)
        .execute(&state.db)
        .await?;
        if json_bool(body.get("force_update").unwrap_or(&Value::Bool(false))) {
            sqlx::query("UPDATE v2_user SET group_id = ?, transfer_enable = ?, speed_limit = ?, device_limit = ?, updated_at = ? WHERE plan_id = ?")
                .bind(group_id)
                .bind(transfer_enable * 1_073_741_824)
                .bind(speed_limit)
                .bind(device_limit)
                .bind(now)
                .bind(id)
                .execute(&state.db)
                .await?;
        }
        return Ok(ok(json!(true)));
    }

    sqlx::query(
        "INSERT INTO v2_plan (group_id, transfer_enable, name, prices, sell, speed_limit, device_limit, `show`, sort, renew, content, tags, reset_traffic_method, capacity_limit, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(group_id)
    .bind(transfer_enable)
    .bind(&name)
    .bind(prices.to_string())
    .bind(sell)
    .bind(speed_limit)
    .bind(device_limit)
    .bind(show)
    .bind(sort)
    .bind(renew)
    .bind(content)
    .bind(tags.to_string())
    .bind(reset_traffic_method)
    .bind(capacity_limit)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;
    Ok(ok(json!(true)))
}

pub async fn plan_update(
    State(state): State<SharedState>,
    admin: AdminUser,
    body: Json<Value>,
) -> ApiResult<Response> {
    plan_save(State(state), admin, body).await
}

pub async fn plan_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let active_orders: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM v2_order WHERE plan_id = ? AND status IN (0,1)")
            .bind(id)
            .fetch_one(&state.db)
            .await?;
    if active_orders.0 > 0 {
        return Err(ApiError::bad_request("该订阅下存在未完成订单无法删除"));
    }
    let active_users: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM v2_user WHERE plan_id = ? AND (expired_at > ? OR expired_at IS NULL)",
    )
    .bind(id)
    .bind(util::now())
    .fetch_one(&state.db)
    .await?;
    if active_users.0 > 0 {
        return Err(ApiError::bad_request("该订阅下存在未过期用户无法删除"));
    }
    sqlx::query("DELETE FROM v2_plan WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn plan_sort(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    if let Some(ids) = body.get("ids").and_then(|v| v.as_array()) {
        for (index, id) in ids.iter().enumerate() {
            if let Some(id) = id.as_i64() {
                sqlx::query("UPDATE v2_plan SET sort = ? WHERE id = ?")
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
// Server groups / routes / templates
// ---------------------------------------------------------------------------

pub async fn groups_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let rows: Vec<(i64, String)> =
        sqlx::query_as("SELECT id, name FROM v2_server_group ORDER BY id ASC")
            .fetch_all(&state.db)
            .await?;
    let mut data = Vec::new();
    for (id, name) in rows {
        let users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM v2_user WHERE group_id = ?")
            .bind(id)
            .fetch_one(&state.db)
            .await?;
        let servers: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM v2_server WHERE JSON_CONTAINS(COALESCE(group_ids, '[]'), ?)",
        )
        .bind(id.to_string())
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));
        data.push(json!({"id": id, "name": name, "users_count": users.0, "server_count": servers.0}));
    }
    Ok(ok(Value::Array(data)))
}

pub async fn group_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    if name.is_empty() {
        return Err(ApiError::bad_request("权限组名称不能为空"));
    }
    let now = util::now();
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        sqlx::query("UPDATE v2_server_group SET name = ?, updated_at = ? WHERE id = ?")
            .bind(name)
            .bind(now)
            .bind(id)
            .execute(&state.db)
            .await?;
    } else {
        sqlx::query("INSERT INTO v2_server_group (name, created_at, updated_at) VALUES (?, ?, ?)")
            .bind(name)
            .bind(now)
            .bind(now)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn group_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("DELETE FROM v2_server_group WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn routes_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let rows: Vec<(i64, String, String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, remarks, `match`, action, action_value FROM v2_server_route ORDER BY id ASC",
    )
    .fetch_all(&state.db)
    .await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, remarks, matches, action, action_value)| {
            json!({
                "id": id,
                "remarks": remarks,
                "match": serde_json::from_str::<Value>(&matches).unwrap_or_else(|_| json!([])),
                "action": action,
                "action_value": action_value,
            })
        })
        .collect();
    Ok(ok(Value::Array(data)))
}

pub async fn route_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let remarks = str_field(&body, "remarks").unwrap_or_default();
    let matches = body.get("match").cloned().unwrap_or_else(|| json!([]));
    let action = str_field(&body, "action").unwrap_or_else(|| "proxy".into());
    let action_value = str_field(&body, "action_value");
    let now = util::now();
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        sqlx::query("UPDATE v2_server_route SET remarks = ?, `match` = ?, action = ?, action_value = ?, updated_at = ? WHERE id = ?")
            .bind(remarks)
            .bind(matches.to_string())
            .bind(action)
            .bind(action_value)
            .bind(now)
            .bind(id)
            .execute(&state.db)
            .await?;
    } else {
        sqlx::query("INSERT INTO v2_server_route (remarks, `match`, action, action_value, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(remarks)
            .bind(matches.to_string())
            .bind(action)
            .bind(action_value)
            .bind(now)
            .bind(now)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn route_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("DELETE FROM v2_server_route WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn network_templates_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let rows: Vec<(i64, String, Option<String>, Option<String>, String)> = match query.get("network") {
        Some(network) if !network.is_empty() => {
            sqlx::query_as("SELECT id, name, description, network, settings FROM v2_network_settings_templates WHERE network = ? ORDER BY id ASC")
                .bind(network)
                .fetch_all(&state.db)
                .await?
        }
        _ => sqlx::query_as("SELECT id, name, description, network, settings FROM v2_network_settings_templates ORDER BY id ASC")
            .fetch_all(&state.db)
            .await?,
    };
    let data: Vec<Value> = rows
        .into_iter()
        .map(|(id, name, description, network, settings)| {
            json!({
                "id": id, "name": name, "description": description, "network": network,
                "settings": serde_json::from_str::<Value>(&settings).unwrap_or_else(|_| json!({})),
            })
        })
        .collect();
    Ok(ok(Value::Array(data)))
}

pub async fn network_template_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    let description = str_field(&body, "description");
    let network = str_field(&body, "network");
    let settings = body.get("settings").cloned().unwrap_or_else(|| json!({}));
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        sqlx::query("UPDATE v2_network_settings_templates SET name = ?, description = ?, network = ?, settings = ?, updated_at = NOW() WHERE id = ?")
            .bind(name)
            .bind(description)
            .bind(network)
            .bind(settings.to_string())
            .bind(id)
            .execute(&state.db)
            .await?;
    } else {
        sqlx::query("INSERT INTO v2_network_settings_templates (name, description, network, settings, created_at, updated_at) VALUES (?, ?, ?, ?, NOW(), NOW())")
            .bind(name)
            .bind(description)
            .bind(network)
            .bind(settings.to_string())
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn network_template_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("DELETE FROM v2_network_settings_templates WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn cert_templates_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let search = query.get("search").cloned().unwrap_or_default();
    let rows: Vec<(i64, String, Option<String>, String, String)> = if search.is_empty() {
        sqlx::query_as("SELECT id, name, description, cert_content, key_content FROM v2_cert_templates ORDER BY id ASC")
            .fetch_all(&state.db)
            .await?
    } else {
        sqlx::query_as("SELECT id, name, description, cert_content, key_content FROM v2_cert_templates WHERE name LIKE ? ORDER BY id ASC")
            .bind(util::like_pattern(&search))
            .fetch_all(&state.db)
            .await?
    };
    Ok(ok(Value::Array(
        rows.into_iter()
            .map(|(id, name, description, cert, key)| {
                json!({
                    "id": id, "name": name, "description": description,
                    "cert_content": cert, "key_content": key
                })
            })
            .collect(),
    )))
}

pub async fn cert_template_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    let description = str_field(&body, "description");
    let cert_content = str_field(&body, "cert_content").unwrap_or_default();
    let key_content = str_field(&body, "key_content").unwrap_or_default();
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        sqlx::query("UPDATE v2_cert_templates SET name = ?, description = ?, cert_content = ?, key_content = ?, updated_at = NOW() WHERE id = ?")
            .bind(name)
            .bind(description)
            .bind(cert_content)
            .bind(key_content)
            .bind(id)
            .execute(&state.db)
            .await?;
    } else {
        sqlx::query("INSERT INTO v2_cert_templates (name, description, cert_content, key_content, created_at, updated_at) VALUES (?, ?, ?, ?, NOW(), NOW())")
            .bind(name)
            .bind(description)
            .bind(cert_content)
            .bind(key_content)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn cert_template_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("DELETE FROM v2_cert_templates WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

// ---------------------------------------------------------------------------
// Server nodes
// ---------------------------------------------------------------------------

pub async fn servers_get_nodes(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current: i64 = query.get("current").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let page_size: i64 = query
        .get("pageSize")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20)
        .clamp(1, 200);
    let search = query.get("search").cloned().unwrap_or_default();
    let type_filter = query.get("type").cloned().unwrap_or_default();
    let machine_filter = query.get("machine_id").and_then(|v| v.parse::<u64>().ok());

    let mut servers: Vec<ServerRow> =
        sqlx::query_as("SELECT * FROM v2_server WHERE type != 'virtual' ORDER BY sort ASC, id ASC")
            .fetch_all(&state.db)
            .await?;
    servers.retain(|s| {
        if !type_filter.is_empty() && s.r#type != type_filter {
            return false;
        }
        if let Some(machine_id) = machine_filter {
            if s.machine_id != Some(machine_id) {
                return false;
            }
        }
        if !search.is_empty() {
            let needle = search.to_lowercase();
            if !s.name.to_lowercase().contains(&needle)
                && !s.host.to_lowercase().contains(&needle)
                && !s.id.to_string().contains(&needle)
            {
                return false;
            }
        }
        true
    });
    let total = servers.len() as i64;
    let start = ((current - 1) * page_size) as usize;
    let slice = servers
        .into_iter()
        .skip(start)
        .take(page_size as usize)
        .collect::<Vec<_>>();

    let mut data = Vec::new();
    for server in slice {
        let groups: Vec<Value> = {
            let ids = server.group_ids();
            if ids.is_empty() {
                vec![]
            } else {
                let mut out = Vec::new();
                for id in ids {
                    let group: Option<(i64, String)> =
                        sqlx::query_as("SELECT id, name FROM v2_server_group WHERE id = ?")
                            .bind(id)
                            .fetch_optional(&state.db)
                            .await?;
                    if let Some((group_id, group_name)) = group {
                        out.push(json!({"id": group_id, "name": group_name}));
                    }
                }
                out
            }
        };
        let last_check = state
            .cache
            .get(&format!(
                "SERVER_{}_LAST_CHECK_AT_{}",
                server.r#type.to_uppercase(),
                server.id
            ))
            .and_then(|v| v.parse::<i64>().ok());
        let online = state
            .cache
            .get(&crate::cache::keys::server_online_user(&server.r#type, server.id))
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(0);
        let mut item = server_admin_json(&server);
        item["groups"] = Value::Array(groups);
        item["parent"] = Value::Null;
        item["online"] = json!(online);
        item["last_check_at"] = json!(last_check);
        item["available_status"] = json!(if last_check.is_some() { 2 } else { 0 });
        item["status"] = json!(if last_check.is_some() { 2 } else { 0 });
        data.push(item);
    }
    Ok(paginate(data, total, current, page_size))
}

fn server_admin_json(server: &ServerRow) -> Value {
    json!({
        "id": server.id,
        "type": server.r#type,
        "code": server.code,
        "parent_id": server.parent_id,
        "machine_id": server.machine_id,
        "group_ids": server.group_ids(),
        "route_ids": server.route_ids(),
        "name": server.name,
        "rate": server.rate.as_ref().and_then(|v| v.to_string().parse::<f64>().ok()),
        "transfer_enable": server.transfer_enable,
        "u": server.u,
        "d": server.d,
        "rate_time_enable": server.rate_time_enable != 0,
        "rate_time_ranges": server.rate_time_ranges.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
        "tags": server.tags_value(),
        "host": server.host,
        "port": server.port,
        "server_port": server.server_port,
        "protocol_settings": server.settings(),
        "custom_outbounds": server.custom_outbounds.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
        "custom_routes": server.custom_routes.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
        "cert_config": server.cert_config.as_deref().and_then(|v| serde_json::from_slice::<Value>(v).ok()),
        "show": server.show,
        "enabled": server.enabled.unwrap_or(1),
        "sort": server.sort,
        "created_at": server.created_at.map(|t| t.timestamp()),
        "updated_at": server.updated_at.map(|t| t.timestamp()),
    })
}

pub async fn server_sort_nodes(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let rows: Vec<(i64, Option<i64>, String, Option<i64>, String)> = sqlx::query_as(
        "SELECT CAST(id AS SIGNED), CAST(parent_id AS SIGNED), name, sort, type FROM v2_server ORDER BY sort ASC, id ASC",
    )
    .fetch_all(&state.db)
    .await?;
    Ok(ok(Value::Array(
        rows.into_iter()
            .map(|(id, parent_id, name, sort, ty)| {
                json!({"id": id, "parent_id": parent_id, "name": name, "sort": sort, "type": ty})
            })
            .collect(),
    )))
}

pub async fn server_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let truthful_save = body.get("type").and_then(|v| v.as_str()).is_some();
    if truthful_save {
        save_server_full(&state, &body).await
    } else {
        update_server_partial(&state, &body).await
    }
}

pub async fn server_update(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    update_server_partial(&state, &body).await
}

async fn save_server_full(state: &SharedState, body: &Value) -> ApiResult<Response> {
    let now = util::now();
    let typ = str_field(body, "type").unwrap_or_default();
    if typ.is_empty() {
        return Err(ApiError::bad_request("协议类型不能为空"));
    }
    let name = str_field(body, "name").unwrap_or_default();
    let host = str_field(body, "host").unwrap_or_default();
    let port = str_field(body, "port").unwrap_or_default();
    let server_port = body.get("server_port").and_then(|v| v.as_i64()).unwrap_or(0);
    if name.is_empty() || host.is_empty() || port.is_empty() {
        return Err(ApiError::bad_request("节点名称/地址/端口不能为空"));
    }
    let code = str_field(body, "code").or_else(|| str_field(body, "spectific_key"));
    let group_ids = json_array_string(body.get("group_ids"));
    let route_ids = json_array_string(body.get("route_ids"));
    let tags = json_array_string(body.get("tags"));
    let protocol_settings = body
        .get("protocol_settings")
        .cloned()
        .unwrap_or_else(|| json!({}))
        .to_string();
    let rate = body.get("rate").and_then(|v| v.as_f64()).unwrap_or(1.0);
    let rate_time_enable = json_bool(body.get("rate_time_enable").unwrap_or(&Value::Bool(false))) as i64;
    let rate_time_ranges = body
        .get("rate_time_ranges")
        .filter(|v| !v.is_null())
        .map(|v| v.to_string());
    let custom_outbounds = body
        .get("custom_outbounds")
        .filter(|v| !v.is_null())
        .map(|v| v.to_string());
    let custom_routes = body
        .get("custom_routes")
        .filter(|v| !v.is_null())
        .map(|v| v.to_string());
    let cert_config = body
        .get("cert_config")
        .filter(|v| !v.is_null())
        .map(|v| v.to_string());
    let show = json_bool(body.get("show").unwrap_or(&Value::Bool(false))) as i64;
    let enabled = json_bool(body.get("enabled").unwrap_or(&Value::Bool(true))) as i64;
    let sort = body.get("sort").and_then(|v| v.as_i64()).unwrap_or(0);
    let transfer_enable = body.get("transfer_enable").and_then(|v| v.as_i64());
    let parent_id = body.get("parent_id").and_then(|v| v.as_i64());
    let machine_id = body.get("machine_id").and_then(|v| v.as_i64());

    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        let exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM v2_server WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;
        if exists.is_none() {
            return Err(ApiError::not_found("服务器不存在"));
        }
        sqlx::query(
            "UPDATE v2_server SET type = ?, code = ?, parent_id = ?, machine_id = ?, group_ids = ?, route_ids = ?, name = ?, rate = ?, transfer_enable = ?, rate_time_enable = ?, rate_time_ranges = ?, tags = ?, host = ?, port = ?, server_port = ?, protocol_settings = ?, custom_outbounds = ?, custom_routes = ?, cert_config = ?, `show` = ?, enabled = ?, sort = ?, updated_at = FROM_UNIXTIME(?) WHERE id = ?",
        )
        .bind(typ)
        .bind(code)
        .bind(parent_id)
        .bind(machine_id)
        .bind(group_ids)
        .bind(route_ids)
        .bind(name)
        .bind(rate)
        .bind(transfer_enable)
        .bind(rate_time_enable)
        .bind(rate_time_ranges)
        .bind(tags)
        .bind(host)
        .bind(port)
        .bind(server_port)
        .bind(protocol_settings)
        .bind(custom_outbounds)
        .bind(custom_routes)
        .bind(cert_config)
        .bind(show)
        .bind(enabled)
        .bind(sort)
        .bind(now)
        .bind(id)
        .execute(&state.db)
        .await?;
    } else {
        sqlx::query(
            "INSERT INTO v2_server (type, code, parent_id, machine_id, group_ids, route_ids, name, rate, transfer_enable, u, d, rate_time_enable, rate_time_ranges, tags, host, port, server_port, protocol_settings, custom_outbounds, custom_routes, cert_config, `show`, enabled, sort, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, FROM_UNIXTIME(?), FROM_UNIXTIME(?))",
        )
        .bind(typ)
        .bind(code)
        .bind(parent_id)
        .bind(machine_id)
        .bind(group_ids)
        .bind(route_ids)
        .bind(name)
        .bind(rate)
        .bind(transfer_enable)
        .bind(rate_time_enable)
        .bind(rate_time_ranges)
        .bind(tags)
        .bind(host)
        .bind(port)
        .bind(server_port)
        .bind(protocol_settings)
        .bind(custom_outbounds)
        .bind(custom_routes)
        .bind(cert_config)
        .bind(show)
        .bind(enabled)
        .bind(sort)
        .bind(now)
        .bind(now)
        .execute(&state.db)
        .await?;
    }
    if let Some(id) = body.get("id").and_then(|v| v.as_u64()) {
        crate::ws::push_node_sync(state, id).await;
    }
    Ok(ok(json!(true)))
}

async fn update_server_partial(state: &SharedState, body: &Value) -> ApiResult<Response> {
    let id = body
        .get("id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| ApiError::bad_request("节点 ID 不能为空"))?;
    let mut sets: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    macro_rules! set_str {
        ($field:literal) => {
            if let Some(v) = str_field(body, $field) {
                sets.push(format!("{} = ?", $field));
                values.push(Value::String(v));
            }
        };
    }
    fn push_num(sets: &mut Vec<String>, values: &mut Vec<Value>, field: &str, v: Value) {
        sets.push(format!("{field} = ?"));
        values.push(v);
    }

    set_str!("name");
    set_str!("code");
    set_str!("host");
    set_str!("port");
    if let Some(v) = body.get("server_port").and_then(|v| v.as_i64()) {
        push_num(&mut sets, &mut values, "server_port", json!(v));
    }
    if let Some(v) = body.get("rate").and_then(|v| v.as_f64()) {
        push_num(&mut sets, &mut values, "rate", json!(v));
    }
    if let Some(v) = body.get("group_ids") {
        push_num(&mut sets, &mut values, "group_ids", json!(json_array_string(Some(v))));
    }
    if let Some(v) = body.get("route_ids") {
        push_num(&mut sets, &mut values, "route_ids", json!(json_array_string(Some(v))));
    }
    if let Some(v) = body.get("tags") {
        push_num(&mut sets, &mut values, "tags", json!(json_array_string(Some(v))));
    }
    if let Some(v) = body.get("protocol_settings") {
        push_num(&mut sets, &mut values, "protocol_settings", json!(v.to_string()));
    }
    if let Some(v) = body.get("show") {
        push_num(&mut sets, &mut values, "`show`", json!(json_bool(v) as i64));
    }
    if let Some(v) = body.get("enabled") {
        push_num(&mut sets, &mut values, "enabled", json!(json_bool(v) as i64));
    }
    if let Some(v) = body.get("sort").and_then(|v| v.as_i64()) {
        push_num(&mut sets, &mut values, "sort", json!(v));
    }
    if let Some(v) = body.get("transfer_enable").and_then(|v| v.as_i64()) {
        push_num(&mut sets, &mut values, "transfer_enable", json!(v));
    }
    if let Some(v) = body.get("machine_id").and_then(|v| v.as_i64()) {
        push_num(&mut sets, &mut values, "machine_id", json!(v));
    }
    if let Some(v) = body.get("parent_id").and_then(|v| v.as_i64()) {
        push_num(&mut sets, &mut values, "parent_id", json!(v));
    }
    if let Some(v) = body.get("rate_time_enable") {
        push_num(&mut sets, &mut values, "rate_time_enable", json!(json_bool(v) as i64));
    }
    if let Some(v) = body.get("rate_time_ranges") {
        push_num(&mut sets, &mut values, "rate_time_ranges", json!(v.to_string()));
    }
    if let Some(v) = body.get("custom_outbounds") {
        push_num(&mut sets, &mut values, "custom_outbounds", json!(v.to_string()));
    }
    if let Some(v) = body.get("custom_routes") {
        push_num(&mut sets, &mut values, "custom_routes", json!(v.to_string()));
    }
    if let Some(v) = body.get("cert_config") {
        push_num(&mut sets, &mut values, "cert_config", json!(v.to_string()));
    }
    if sets.is_empty() {
        return Ok(ok(json!(true)));
    }
    sets.push("updated_at = NOW()".into());
    let sql = format!("UPDATE v2_server SET {} WHERE id = ?", sets.join(", "));
    let mut query = sqlx::query(&sql);
    for value in values {
        query = match value {
            Value::String(s) => query.bind(s),
            Value::Number(n) if n.is_f64() => query.bind(n.as_f64().unwrap_or(0.0)),
            Value::Number(n) => query.bind(n.as_i64().unwrap_or(0)),
            Value::Bool(b) => query.bind(b as i64),
            other => query.bind(other.to_string()),
        };
    }
    query.bind(id).execute(&state.db).await?;
    if id > 0 {
        crate::ws::push_node_sync(state, id as u64).await;
    }
    Ok(ok(json!(true)))
}

pub async fn server_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    // Cascade virtual children like the legacy observer.
    sqlx::query("DELETE FROM v2_server WHERE parent_id = ? AND type = 'virtual'")
        .bind(id)
        .execute(&state.db)
        .await?;
    sqlx::query("DELETE FROM v2_server WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    if let Some(machine_id) = body.get("machine_id").and_then(|v| v.as_u64()) {
        push_machine_nodes_now(&state, machine_id as u64).await;
    }
    Ok(ok(json!(true)))
}

pub async fn server_batch_delete(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    if let Some(ids) = body.get("ids").and_then(|v| v.as_array()) {
        for id in ids {
            if let Some(id) = id.as_i64() {
                sqlx::query("DELETE FROM v2_server WHERE parent_id = ? AND type = 'virtual'")
                    .bind(id)
                    .execute(&state.db)
                    .await?;
                sqlx::query("DELETE FROM v2_server WHERE id = ?")
                    .bind(id)
                    .execute(&state.db)
                    .await?;
            }
        }
    }
    Ok(ok(json!(true)))
}

pub async fn server_sort(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    if let Some(ids) = body.get("ids").and_then(|v| v.as_array()) {
        for (index, id) in ids.iter().enumerate() {
            if let Some(id) = id.as_i64() {
                sqlx::query("UPDATE v2_server SET sort = ? WHERE id = ?")
                    .bind(index as i64 + 1)
                    .bind(id)
                    .execute(&state.db)
                    .await?;
            }
        }
    } else if let Some(items) = body.as_array() {
        for item in items {
            if let (Some(id), Some(order)) = (
                item.get("id").and_then(|v| v.as_i64()),
                item.get("order").and_then(|v| v.as_i64()),
            ) {
                sqlx::query("UPDATE v2_server SET sort = ? WHERE id = ?")
                    .bind(order)
                    .bind(id)
                    .execute(&state.db)
                    .await?;
            }
        }
    }
    Ok(ok(json!(true)))
}

pub async fn server_copy(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let row: Option<ServerRow> = sqlx::query_as("SELECT * FROM v2_server WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    let Some(server) = row else {
        return Err(ApiError::not_found("服务器不存在"));
    };
    let now = util::now();
    sqlx::query(
        "INSERT INTO v2_server (type, code, parent_id, machine_id, group_ids, route_ids, name, rate, transfer_enable, u, d, rate_time_enable, rate_time_ranges, tags, host, port, server_port, protocol_settings, custom_outbounds, custom_routes, cert_config, `show`, enabled, sort, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, FROM_UNIXTIME(?), FROM_UNIXTIME(?))",
    )
    .bind(server.r#type)
    .bind(server.code)
    .bind(server.parent_id)
    .bind(server.machine_id)
    .bind(server.group_ids)
    .bind(server.route_ids)
    .bind(format!("{} 副本", server.name))
    .bind(server.rate)
    .bind(server.transfer_enable)
    .bind(server.rate_time_enable)
    .bind(server.rate_time_ranges)
    .bind(server.tags)
    .bind(server.host)
    .bind(server.port)
    .bind(server.server_port)
    .bind(server.protocol_settings)
    .bind(server.custom_outbounds)
    .bind(server.custom_routes)
    .bind(server.cert_config)
    .bind(server.show)
    .bind(server.enabled.unwrap_or(1))
    .bind(server.sort.unwrap_or(0))
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;
    Ok(ok(json!(true)))
}

pub async fn server_reset_traffic(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("UPDATE v2_server SET u = 0, d = 0, updated_at = NOW() WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn server_batch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let ids: Vec<i64> = body
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_i64()).collect())
        .unwrap_or_default();
    let action = str_field(&body, "action").unwrap_or_else(|| "delete".into());
    for id in ids {
        match action.as_str() {
            "delete" => {
                let _ = sqlx::query("DELETE FROM v2_server WHERE id = ?")
                    .bind(id)
                    .execute(&state.db)
                    .await;
            }
            "resetTraffic" => {
                let _ = sqlx::query("UPDATE v2_server SET u = 0, d = 0 WHERE id = ?")
                    .bind(id)
                    .execute(&state.db)
                    .await;
            }
            _ => {}
        }
    }
    Ok(ok(json!(true)))
}

pub async fn server_restart(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    node_submit_op(&state, &body, "sync.restart")
}

pub async fn server_upgrade(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    node_submit_op(&state, &body, "sync.upgrade")
}

fn node_submit_op(state: &SharedState, body: &Value, event: &str) -> ApiResult<Response> {
    let mut ids: Vec<u64> = body
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_u64()).collect())
        .unwrap_or_default();
    if let Some(id) = body.get("id").and_then(|v| v.as_u64()) {
        ids.push(id);
    }
    if ids.is_empty() {
        ids = state.ws.connected_node_ids();
    }
    let mut submitted = 0;
    for id in ids {
        if state.ws.send_node(id, event, json!({})) {
            submitted += 1;
        }
    }
    Ok(ok(json!({"submitted": submitted})))
}

pub async fn server_get_virtual_nodes(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Path(id): Path<i64>,
) -> ApiResult<Response> {
    let row: Option<(Option<Vec<u8>>,)> =
        sqlx::query_as("SELECT protocol_settings FROM v2_server WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;
    let settings: Value = row
        .and_then(|(v,)| v)
        .and_then(|v| serde_json::from_slice(&v).ok())
        .unwrap_or_else(|| json!({}));
    if let Some(nodes) = settings.get("virtual_nodes") {
        return Ok(ok(nodes.clone()));
    }
    let rows: Vec<ServerRow> =
        sqlx::query_as("SELECT * FROM v2_server WHERE parent_id = ? AND type = 'virtual' ORDER BY sort ASC")
            .bind(id)
            .fetch_all(&state.db)
            .await?;
    Ok(ok(Value::Array(
        rows.iter().map(server_admin_json).collect(),
    )))
}

pub async fn server_save_virtual_nodes(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Path(id): Path<i64>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let row: Option<(Option<Vec<u8>>, Option<Vec<u8>>)> =
        sqlx::query_as("SELECT protocol_settings, group_ids FROM v2_server WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;
    let Some((settings_raw, parent_groups)) = row else {
        return Err(ApiError::not_found("节点不存在"));
    };
    let mut settings: Value = settings_raw
        .as_deref()
        .and_then(|v| serde_json::from_slice(v).ok())
        .unwrap_or_else(|| json!({}));
    let parent_groups: Vec<i64> = parent_groups
        .as_deref()
        .and_then(|v| serde_json::from_slice::<Vec<Value>>(v).ok())
        .map(|v| v.into_iter().filter_map(|x| x.as_i64()).collect())
        .unwrap_or_default();
    let mut nodes = body
        .get("virtual_nodes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    for node in nodes.iter_mut() {
        if let Some(groups) = node.get("group_ids").and_then(|v| v.as_array()) {
            let filtered: Vec<Value> = groups
                .iter()
                .filter(|v| {
                    v.as_i64()
                        .map(|id| parent_groups.is_empty() || parent_groups.contains(&id))
                        .unwrap_or(false)
                })
                .cloned()
                .collect();
            if let Some(obj) = node.as_object_mut() {
                obj.insert("group_ids".into(), Value::Array(filtered));
            }
        }
    }
    if let Some(obj) = settings.as_object_mut() {
        obj.insert("virtual_nodes".into(), Value::Array(nodes.clone()));
    }
    sqlx::query("UPDATE v2_server SET protocol_settings = ?, updated_at = NOW() WHERE id = ?")
        .bind(settings.to_string())
        .bind(id)
        .execute(&state.db)
        .await?;
    crate::ws::push_node_sync(&state, id as u64).await;
    Ok(ok(Value::Array(nodes)))
}

pub async fn server_create_child(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let parent_id = body
        .get("parent_id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| ApiError::bad_request("父节点不能为空"))?;
    let parent: Option<(Option<Vec<u8>>,)> =
        sqlx::query_as("SELECT group_ids FROM v2_server WHERE id = ?")
            .bind(parent_id)
            .fetch_optional(&state.db)
            .await?;
    let Some((parent_groups,)) = parent else {
        return Err(ApiError::not_found("父节点不存在"));
    };
    let parent_groups: Vec<i64> = parent_groups
        .as_deref()
        .and_then(|v| serde_json::from_slice::<Vec<Value>>(v).ok())
        .map(|v| v.into_iter().filter_map(|x| x.as_i64()).collect())
        .unwrap_or_default();
    let groups: Vec<i64> = body
        .get("group_ids")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_i64()).collect::<Vec<i64>>())
        .unwrap_or_default()
        .into_iter()
        .filter(|id| parent_groups.is_empty() || parent_groups.contains(id))
        .collect();
    let name = str_field(&body, "name").unwrap_or_default();
    let host = str_field(&body, "host").unwrap_or_default();
    let port = body.get("port").and_then(|v| v.as_i64()).unwrap_or(0);
    let tags = json_array_string(body.get("tags"));
    let show = crate::api::admin::json_bool(body.get("show").unwrap_or(&Value::Bool(true))) as i64;
    let now = util::now();
    let settings = json!({"virtual_nodes": []});
    let result = sqlx::query(
        "INSERT INTO v2_server (type, parent_id, group_ids, route_ids, name, rate, u, d, rate_time_enable, tags, host, port, server_port, protocol_settings, `show`, enabled, sort, created_at, updated_at) VALUES ('virtual', ?, ?, '[]', ?, 1, 0, 0, 0, ?, ?, ?, 0, ?, ?, 1, 0, FROM_UNIXTIME(?), FROM_UNIXTIME(?))",
    )
    .bind(parent_id)
    .bind(serde_json::to_string(&groups).unwrap_or_else(|_| "[]".into()))
    .bind(name)
    .bind(tags)
    .bind(host)
    .bind(port.to_string())
    .bind(settings.to_string())
    .bind(show)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;
    let id = result.last_insert_id() as i64;
    let node: Option<ServerRow> = sqlx::query_as("SELECT * FROM v2_server WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    Ok(ok(node.map(|n| server_admin_json(&n)).unwrap_or(Value::Null)))
}

pub async fn server_update_child(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body
        .get("id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| ApiError::bad_request("子节点 ID 不能为空"))?;
    let node: Option<ServerRow> = sqlx::query_as("SELECT * FROM v2_server WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    let Some(node) = node else {
        return Err(ApiError::not_found("子节点不存在"));
    };
    if node.r#type != "virtual" {
        return Err(ApiError::bad_request("子节点不存在"));
    }
    let name = str_field(&body, "name").unwrap_or_else(|| node.name.clone());
    let host = str_field(&body, "host").unwrap_or_else(|| node.host.clone());
    let port = str_field(&body, "port").unwrap_or_else(|| node.port.clone());
    let tags = body
        .get("tags")
        .map(|v| json_array_string(Some(v)))
        .unwrap_or_else(|| node.tags_value().to_string());
    let groups = body
        .get("group_ids")
        .map(|v| json_array_string(Some(v)))
        .unwrap_or_else(|| node.group_ids_string());
    let show = body
        .get("show")
        .map(|v| crate::api::admin::json_bool(v) as i64)
        .unwrap_or(node.show);
    sqlx::query("UPDATE v2_server SET name = ?, host = ?, port = ?, tags = ?, group_ids = ?, `show` = ?, updated_at = NOW() WHERE id = ?")
        .bind(name)
        .bind(host)
        .bind(port)
        .bind(tags)
        .bind(groups)
        .bind(show)
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn server_batch_update(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let ids: Vec<i64> = body
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_i64()).collect())
        .unwrap_or_default();
    let mut sets = Vec::new();
    let mut values: Vec<Value> = Vec::new();
    if let Some(v) = body.get("show") {
        sets.push("`show` = ?".to_string());
        values.push(json!(crate::api::admin::json_bool(v) as i64));
    }
    if let Some(v) = body.get("enabled").or_else(|| body.get("banned")) {
        sets.push("enabled = ?".to_string());
        values.push(json!(crate::api::admin::json_bool(v) as i64));
    }
    if sets.is_empty() || ids.is_empty() {
        return Ok(ok(json!({"changed": 0})));
    }
    let mut changed = 0i64;
    for id in ids {
        let sql = format!("UPDATE v2_server SET {}, updated_at = NOW() WHERE id = ?", sets.join(", "));
        let mut query = sqlx::query(&sql);
        for value in &values {
            query = match value {
                Value::Number(n) => query.bind(n.as_i64().unwrap_or(0)),
                other => query.bind(other.to_string()),
            };
        }
        let result = query.bind(id).execute(&state.db).await?;
        changed += result.rows_affected() as i64;
        crate::ws::push_node_sync(&state, id as u64).await;
    }
    Ok(ok(json!({"changed": changed})))
}

pub async fn server_batch_reset_traffic(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    if let Some(ids) = body.get("ids").and_then(|v| v.as_array()) {
        for id in ids.iter().filter_map(|v| v.as_i64()) {
            sqlx::query("UPDATE v2_server SET u = 0, d = 0, updated_at = NOW() WHERE id = ?")
                .bind(id)
                .execute(&state.db)
                .await?;
        }
    }
    Ok(ok(json!(true)))
}

pub async fn server_batch_replace(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let field = str_field(&body, "field").unwrap_or_default();
    let search = str_field(&body, "search").unwrap_or_default();
    let replace = str_field(&body, "replace").unwrap_or_default();
    let allowed = ["name", "host", "port"];
    if !allowed.contains(&field.as_str()) || search.is_empty() {
        return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "参数有误"));
    }
    let sql = format!("UPDATE v2_server SET {field} = REPLACE({field}, ?, ?), updated_at = NOW()");
    let result = sqlx::query(&sql)
        .bind(search)
        .bind(replace)
        .execute(&state.db)
        .await?;
    Ok(ok(json!({"changed": result.rows_affected()})))
}

pub async fn server_children(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Path(id): Path<i64>,
) -> ApiResult<Response> {
    let rows: Vec<ServerRow> = sqlx::query_as(
        "SELECT * FROM v2_server WHERE parent_id = ? AND type = 'virtual' ORDER BY sort ASC",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;
    Ok(ok(Value::Array(
        rows.iter().map(server_admin_json).collect(),
    )))
}

// ---------------------------------------------------------------------------
// Protocol definitions (plugin-provided)
// ---------------------------------------------------------------------------

pub async fn protocol_definitions(State(state): State<SharedState>, _admin: AdminUser) -> ApiResult<Response> {
    let base = builtin_protocol_definitions();
    let merged = merged_protocol_definitions(&state, base.clone()).await;
    Ok(ok(if merged.is_object() { merged } else { base }))
}

pub async fn protocol_types(State(state): State<SharedState>, _admin: AdminUser) -> ApiResult<Response> {
    let base = builtin_protocol_definitions();
    let merged = merged_protocol_definitions(&state, base).await;
    let types: Vec<String> = merged
        .as_object()
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default();
    Ok(ok(json!(types)))
}

pub async fn protocol_definition(
    State(state): State<SharedState>,
    _admin: AdminUser,
    Path(typ): Path<String>,
) -> ApiResult<Response> {
    let base = builtin_protocol_definitions();
    let merged = merged_protocol_definitions(&state, base).await;
    Ok(ok(merged.get(&typ).cloned().unwrap_or(Value::Null)))
}

pub async fn protocol_fields(
    State(state): State<SharedState>,
    _admin: AdminUser,
    Path(typ): Path<String>,
) -> ApiResult<Response> {
    let base = builtin_protocol_definitions();
    let merged = merged_protocol_definitions(&state, base).await;
    Ok(ok(merged
        .get(&typ)
        .and_then(|v| v.get("config_fields"))
        .cloned()
        .unwrap_or_else(|| json!({}))))
}

pub async fn protocol_validation(
    State(state): State<SharedState>,
    _admin: AdminUser,
    Path(typ): Path<String>,
) -> ApiResult<Response> {
    let base = builtin_protocol_definitions();
    let merged = merged_protocol_definitions(&state, base).await;
    Ok(ok(merged
        .get(&typ)
        .and_then(|v| v.get("validation_rules"))
        .cloned()
        .unwrap_or_else(|| json!({}))))
}

async fn merged_protocol_definitions(state: &SharedState, base: Value) -> Value {
    let settings = state.settings().await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = PluginContext::new(&state.db, &settings, &app_url, json!({}));
    state.plugin.protocol_definitions(base, &ctx).await
}

pub(crate) fn builtin_protocol_definitions() -> Value {
    let simple = |name: &str, fields: Value| {
        json!({
            "name": name,
            "config_fields": fields,
            "validation_rules": {},
            "prefix": Value::Null,
            "aliases": [],
            "transform_node_user_uuid": false,
        })
    };
    json!({
        "shadowsocks": simple("Shadowsocks", json!({
            "cipher": {"type": "string", "label": "加密方式", "default": "aes-256-gcm"},
            "plugin": {"type": "string", "label": "插件", "default": null},
            "plugin_opts": {"type": "string", "label": "插件参数", "default": null}
        })),
        "vmess": simple("VMess", json!({
            "tls": {"type": "integer", "label": "TLS", "default": 0},
            "network": {"type": "string", "label": "传输协议", "default": "tcp"},
            "network_settings": {"type": "array", "label": "网络设置", "default": null},
            "tls_settings": {"type": "object", "label": "TLS设置", "default": null}
        })),
        "vless": simple("VLESS", json!({
            "tls": {"type": "integer", "label": "TLS", "default": 0},
            "network": {"type": "string", "label": "传输协议", "default": "tcp"},
            "flow": {"type": "string", "label": "流控", "default": null},
            "reality_settings": {"type": "object", "label": "REALITY", "default": null}
        })),
        "trojan": simple("Trojan", json!({
            "tls": {"type": "integer", "label": "TLS", "default": 1},
            "network": {"type": "string", "label": "传输协议", "default": "tcp"},
            "tls_settings": {"type": "object", "label": "TLS设置", "default": null}
        })),
        "hysteria": simple("Hysteria", json!({
            "version": {"type": "integer", "label": "版本", "default": 2},
            "tls": {"type": "object", "label": "TLS", "default": null},
            "obfs": {"type": "object", "label": "混淆", "default": null}
        })),
        "tuic": simple("TUIC", json!({
            "tls": {"type": "object", "label": "TLS", "default": null},
            "alpn": {"type": "array", "label": "ALPN", "default": null}
        })),
        "anytls": simple("AnyTLS", json!({
            "tls": {"type": "object", "label": "TLS", "default": null}
        })),
        "socks": simple("SOCKS", json!({})),
        "http": simple("HTTP", json!({})),
        "naive": simple("Naive", json!({})),
        "mieru": simple("Mieru", json!({})),
        "sudoku": simple("Sudoku", json!({})),
        "shadowquic": simple("ShadowQUIC", json!({})),
        "trusttunnel": simple("TrustTunnel", json!({})),
        "masque": simple("MASQUE", json!({})),
    })
}

// ---------------------------------------------------------------------------
// Machines
// ---------------------------------------------------------------------------

pub async fn machines_fetch(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let current = query
        .get("current")
        .or_else(|| query.get("page"))
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(1)
        .max(1);
    let page_size = query
        .get("pageSize")
        .or_else(|| query.get("per_page"))
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(10)
        .clamp(1, 200);
    let search = query.get("q").or_else(|| query.get("search")).cloned().unwrap_or_default().to_lowercase();
    let status_filter = query.get("status").cloned().unwrap_or_else(|| "all".into());

    let counts: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT CAST(machine_id AS SIGNED), COUNT(*) FROM v2_server WHERE machine_id IS NOT NULL GROUP BY machine_id",
    )
    .fetch_all(&state.db)
    .await?;
    let counts: HashMap<i64, i64> = counts.into_iter().collect();

    let rows: Vec<(i64, String, String, Option<String>, i64, Option<i64>, Option<Vec<u8>>, Option<i64>)> =
        sqlx::query_as(
            "SELECT CAST(id AS SIGNED), name, token, notes, is_active, CAST(last_seen_at AS SIGNED), load_status, CAST(UNIX_TIMESTAMP(created_at) AS SIGNED) FROM v2_server_machine ORDER BY id ASC",
        )
        .fetch_all(&state.db)
        .await?;

    let now = util::now();
    let mut all: Vec<Value> = Vec::new();
    let mut summary_total = 0i64;
    let mut summary_online = 0i64;
    let mut summary_high_load = 0i64;
    let mut summary_nodes = 0i64;
    for (id, name, token, notes, active, last_seen, load, created) in rows {
        let load_status = load
            .as_deref()
            .and_then(|v| serde_json::from_slice::<Value>(v).ok());
        let ws_online = state.ws.machine_connected(id as u64);
        let recent = last_seen.map(|v| v > now - 300).unwrap_or(false);
        let is_online = ws_online || recent;
        let version = load_status
            .as_ref()
            .and_then(|v| v.get("version"))
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let high_load = load_status.as_ref().map(machine_high_load).unwrap_or(false);
        let servers_count = counts.get(&id).copied().unwrap_or(0);

        summary_total += 1;
        summary_nodes += servers_count;
        if active != 0 && is_online {
            summary_online += 1;
        }
        if active != 0 && high_load {
            summary_high_load += 1;
        }

        if !search.is_empty()
            && !name.to_lowercase().contains(&search)
            && !notes
                .as_deref()
                .unwrap_or("")
                .to_lowercase()
                .contains(&search)
        {
            continue;
        }
        let matches_status = match status_filter.as_str() {
            "online" => active != 0 && is_online,
            "offline" => active != 0 && !is_online,
            "inactive" => active == 0,
            "high_load" => active != 0 && high_load,
            _ => true,
        };
        if !matches_status {
            continue;
        }

        all.push(json!({
            "id": id,
            "name": name,
            "token": token,
            "notes": notes,
            "is_active": active != 0,
            "is_online": is_online,
            "last_seen_at": last_seen,
            "load_status": load_status,
            "version": version,
            "servers_count": servers_count,
            "created_at": created,
        }));
    }

    let total = all.len() as i64;
    let start = ((current - 1) * page_size).max(0) as usize;
    let data: Vec<Value> = all
        .into_iter()
        .skip(start)
        .take(page_size as usize)
        .collect();
    let last_page = ((total + page_size - 1) / page_size).max(1);
    Ok(ok(json!({
        "data": data,
        "total": total,
        "current_page": current,
        "per_page": page_size,
        "last_page": last_page,
        "summary": {
            "total": summary_total,
            "online": summary_online,
            "offline": (summary_total - summary_online).max(0),
            "high_load": summary_high_load,
            "nodes": summary_nodes,
        },
    })))
}

fn machine_high_load(load: &Value) -> bool {
    let cpu = load.get("cpu").and_then(|v| v.as_f64()).unwrap_or(0.0);
    if cpu >= 70.0 {
        return true;
    }
    for key in ["mem", "disk"] {
        if let Some(resource) = load.get(key) {
            let total = resource
                .get("total")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let used = resource
                .get("used")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            if total > 0.0 && used / total * 100.0 >= 70.0 {
                return true;
            }
        }
    }
    false
}

pub async fn machine_save(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let name = str_field(&body, "name").unwrap_or_default();
    let notes = str_field(&body, "notes");
    let active = json_bool(body.get("is_active").unwrap_or(&Value::Bool(true))) as i64;
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()).filter(|v| *v > 0) {
        sqlx::query("UPDATE v2_server_machine SET name = ?, notes = ?, is_active = ?, updated_at = NOW() WHERE id = ?")
            .bind(name)
            .bind(notes)
            .bind(active)
            .bind(id)
            .execute(&state.db)
            .await?;
    } else {
        let token = util::random_chars(48);
        sqlx::query("INSERT INTO v2_server_machine (name, token, notes, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, NOW(), NOW())")
            .bind(name)
            .bind(token)
            .bind(notes)
            .bind(active)
            .execute(&state.db)
            .await?;
    }
    Ok(ok(json!(true)))
}

pub async fn machine_drop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    sqlx::query("UPDATE v2_server SET machine_id = NULL WHERE machine_id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    sqlx::query("DELETE FROM v2_server_machine WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(true)))
}

pub async fn machine_reset_token(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let token = util::random_chars(48);
    sqlx::query("UPDATE v2_server_machine SET token = ?, updated_at = NOW() WHERE id = ?")
        .bind(&token)
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(ok(json!(token)))
}

pub async fn machine_get_token(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let row: Option<(String,)> = sqlx::query_as("SELECT token FROM v2_server_machine WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    Ok(ok(json!(row.map(|(t,)| t).unwrap_or_default())))
}

pub async fn machine_install_command(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let row: Option<(String, String)> =
        sqlx::query_as("SELECT name, token FROM v2_server_machine WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;
    let settings = state.settings().await;
    let app_url = settings.str("app_url", &state.cfg.app_url);
    let (_, token) = row.unwrap_or_default();
    Ok(ok(json!(format!(
        "curl -fsSL {app_url}/install.sh | bash -s -- --panel={app_url} --machine-id={id} --token={token}"
    ))))
}

pub async fn machine_nodes(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let rows: Vec<(i64, String, String)> = sqlx::query_as(
        "SELECT CAST(id AS SIGNED), name, type FROM v2_server WHERE machine_id = ? ORDER BY sort ASC",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;
    Ok(ok(Value::Array(
        rows.into_iter()
            .map(|(id, name, ty)| json!({"id": id, "name": name, "type": ty}))
            .collect(),
    )))
}

pub async fn machine_available_nodes(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let rows: Vec<(i64, String, String)> = sqlx::query_as(
        "SELECT CAST(id AS SIGNED), name, type FROM v2_server WHERE (machine_id IS NULL OR machine_id = ?) AND type != 'virtual' ORDER BY sort ASC",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;
    Ok(ok(Value::Array(
        rows.into_iter()
            .map(|(id, name, ty)| json!({"id": id, "name": name, "type": ty}))
            .collect(),
    )))
}

pub async fn machine_bind_nodes(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let machine_id = body.get("machine_id").and_then(|v| v.as_i64()).unwrap_or(0);
    if let Some(ids) = body.get("node_ids").and_then(|v| v.as_array()) {
        for id in ids {
            if let Some(id) = id.as_i64() {
                sqlx::query("UPDATE v2_server SET machine_id = ?, updated_at = NOW() WHERE id = ?")
                    .bind(machine_id)
                    .bind(id)
                    .execute(&state.db)
                    .await?;
            }
        }
    }
    push_machine_nodes_now(&state, machine_id as u64).await;
    Ok(ok(json!(true)))
}

pub async fn machine_unbind_node(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("node_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let machine: Option<(Option<u64>,)> = sqlx::query_as("SELECT machine_id FROM v2_server WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    sqlx::query("UPDATE v2_server SET machine_id = NULL, updated_at = NOW() WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    if let Some((Some(machine_id),)) = machine {
        push_machine_nodes_now(&state, machine_id as u64).await;
    }
    Ok(ok(json!(true)))
}

pub async fn machine_history(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let rows: Vec<(i64, f64, i64, i64, i64, i64, f64, f64, i64)> = sqlx::query_as(
        "SELECT CAST(id AS SIGNED), cpu, CAST(mem_total AS SIGNED), CAST(mem_used AS SIGNED), CAST(disk_total AS SIGNED), CAST(disk_used AS SIGNED), net_in_speed, net_out_speed, CAST(recorded_at AS SIGNED) FROM v2_server_machine_load_history WHERE machine_id = ? ORDER BY recorded_at DESC LIMIT 500",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;
    Ok(ok(Value::Array(
        rows.into_iter()
            .map(|(id, cpu, mt, mu, dt, du, ni, no, at)| {
                json!({
                    "id": id, "cpu": cpu, "mem_total": mt, "mem_used": mu,
                    "disk_total": dt, "disk_used": du, "net_in_speed": ni,
                    "net_out_speed": no, "recorded_at": at
                })
            })
            .collect(),
    )))
}

pub async fn machine_ops_ack(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<u64>().ok()).unwrap_or(0);
    let acks = state.ws.acks_for(id);
    Ok(ok(json!({"machine_id": id, "acks": acks})))
}

pub async fn machine_logs(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let id = query.get("id").and_then(|v| v.parse::<u64>().ok()).unwrap_or(0);
    let connected = state.ws.machine_connected(id);
    if connected {
        let req_id = util::random_chars(12);
        let limit = query
            .get("limit")
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(200);
        let _ = state.ws.send_machine(
            id,
            "sync.logs",
            json!({"limit": limit, "req_id": req_id}),
        );
    }
    let cached = state.ws.get_logs(id);
    let lines = cached
        .as_ref()
        .and_then(|v| v.get("lines").cloned())
        .unwrap_or_else(|| json!([]));
    let updated_at = cached
        .as_ref()
        .and_then(|v| v.get("updated_at").cloned())
        .unwrap_or(Value::Null);
    Ok(ok(json!({
        "online": connected,
        "lines": lines,
        "updated_at": updated_at,
        "stale": cached.is_none(),
        "message": if connected { Value::Null } else { json!("机器未通过 WebSocket 连接") },
    })))
}

pub async fn machine_upgrade(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    machine_submit_op(&state, &body, "sync.upgrade", json!({}))
}

pub async fn machine_restart(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    machine_submit_op(&state, &body, "sync.restart", json!({}))
}

pub async fn machine_stop(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    machine_submit_op(&state, &body, "sync.kernel", json!({"action": "stop"}))
}

pub async fn machine_start(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    machine_submit_op(&state, &body, "sync.kernel", json!({"action": "start"}))
}

pub async fn machine_reload(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    machine_submit_op(&state, &body, "sync.kernel", json!({"action": "reload"}))
}

pub async fn machine_batch_upgrade(
    State(state): State<SharedState>,
    AdminUser(_admin): AdminUser,
) -> ApiResult<Response> {
    let rows: Vec<(u64,)> = sqlx::query_as("SELECT CAST(id AS SIGNED) FROM v2_server_machine WHERE is_active = 1")
        .fetch_all(&state.db)
        .await?;
    let mut submitted = 0i64;
    let mut offline = 0i64;
    for (id,) in rows {
        if state
            .ws
            .send_machine(id, "sync.upgrade", json!({}))
        {
            submitted += 1;
        } else {
            offline += 1;
        }
    }
    Ok(ok(json!({
        "submitted": submitted,
        "skipped": {"inactive": 0, "offline": offline},
    })))
}

fn machine_submit_op(
    state: &SharedState,
    body: &Value,
    event: &str,
    data: Value,
) -> ApiResult<Response> {
    let id = body
        .get("id")
        .and_then(|v| v.as_u64())
        .or_else(|| body.get("machine_id").and_then(|v| v.as_u64()))
        .ok_or_else(|| ApiError::bad_request("机器 ID 不能为空"))?;
    crate::ws::send_machine_op(state, id, event, data.clone())?;
    Ok(ok(json!({
        "submitted": true,
        "machine_id": id,
        "action": data.get("action").cloned().unwrap_or(Value::Null),
    })))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

pub fn json_bool(value: &Value) -> bool {
    match value {
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_i64().unwrap_or(0) != 0,
        Value::String(s) => matches!(s.as_str(), "1" | "true" | "yes" | "on"),
        _ => false,
    }
}

pub fn json_array_string(value: Option<&Value>) -> String {
    match value {
        Some(Value::Array(items)) => {
            let normalized: Vec<Value> = items
                .iter()
                .map(|v| match v {
                    Value::String(s) => Value::String(s.clone()),
                    Value::Number(n) => Value::String(n.to_string()),
                    other => other.clone(),
                })
                .collect();
            serde_json::to_string(&normalized).unwrap_or_else(|_| "[]".into())
        }
        Some(Value::String(s)) => {
            if s.is_empty() {
                "[]".into()
            } else if s.starts_with('[') {
                s.clone()
            } else {
                serde_json::to_string(&vec![s.clone()]).unwrap_or_else(|_| "[]".into())
            }
        }
        Some(Value::Null) | None => "[]".into(),
        Some(other) => other.to_string(),
    }
}


async fn push_machine_nodes_now(state: &SharedState, machine_id: u64) {
    let nodes: Vec<ServerRow> = sqlx::query_as(
        "SELECT * FROM v2_server WHERE machine_id = ? AND (enabled IS NULL OR enabled = 1) ORDER BY sort ASC",
    )
    .bind(machine_id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();
    crate::ws::push_machine_nodes(state, machine_id, &nodes);
}
