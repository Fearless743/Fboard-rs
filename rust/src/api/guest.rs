use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use std::collections::HashMap as StdHashMap;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};

use crate::error::{ApiError, ApiResult};
use crate::models::PlanRow;
use crate::plugin::PluginContext;
use crate::response::{ok, str_field};
use crate::state::SharedState;

pub async fn plans(State(state): State<SharedState>, Query(_q): Query<HashMap<String, String>>) -> ApiResult<Response> {
    let rows: Vec<PlanRow> = sqlx::query_as(
        "SELECT * FROM v2_plan WHERE `show` = 1 AND sell = 1 ORDER BY sort ASC, id ASC",
    )
    .fetch_all(&state.db)
    .await?;

    let settings = state.settings().await;
    let mut out = Vec::new();
    for plan in rows {
        let capacity: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM v2_user WHERE plan_id = ? AND (expired_at >= ? OR expired_at IS NULL)",
        )
        .bind(plan.id)
        .bind(crate::util::now())
        .fetch_one(&state.db)
        .await?;
        if let Some(limit) = plan.capacity_limit {
            if (limit as i64) - capacity.0 <= 0 {
                continue;
            }
        }
        out.push(plan_json(&plan, &settings));
    }
    Ok(ok(Value::Array(out)))
}

pub fn plan_json(plan: &PlanRow, settings: &crate::settings::Settings) -> Value {
    let prices = plan.prices_value();
    let mut periods = serde_json::Map::new();
    for (legacy, new_period) in [
        ("hour_price", "hourly"),
        ("day_price", "daily"),
        ("month_price", "monthly"),
        ("quarter_price", "quarterly"),
        ("half_year_price", "half_yearly"),
        ("year_price", "yearly"),
        ("two_year_price", "two_yearly"),
        ("three_year_price", "three_yearly"),
        ("onetime_price", "onetime"),
        ("reset_price", "reset_traffic"),
    ] {
        let price = prices
            .get(new_period)
            .and_then(|v| v.as_f64())
            .map(|v| v * 100.0);
        periods.insert(legacy.to_string(), price.map(Value::from).unwrap_or(Value::Null));
    }

    let transfer = plan.transfer_enable.unwrap_or(0);
    let speed = plan
        .speed_limit
        .map(|v| v.to_string())
        .unwrap_or_else(|| "无限制".to_string());
    let devices = plan
        .device_limit
        .map(|v| v.to_string())
        .unwrap_or_else(|| "无限制".to_string());
    let reset_method = match plan.reset_traffic_method {
        None => settings.i64("reset_traffic_method", 1),
        Some(v) => v,
    };
    let reset_text = match reset_method {
        0 => "每月1号",
        2 => "不重置",
        3 => "每年1月1日",
        4 => "按年重置",
        _ => "按月重置",
    };
    let content = plan
        .content
        .clone()
        .unwrap_or_default()
        .replace("{{transfer}}", &transfer.to_string())
        .replace("{{speed}}", &speed)
        .replace("{{devices}}", &devices)
        .replace("{{reset_method}}", reset_text);

    let capacity = match plan.capacity_limit {
        None => Value::Null,
        Some(v) if v <= 0 => Value::String("Sold out".into()),
        Some(v) => Value::from(v),
    };

    let mut map = serde_json::Map::new();
    map.insert("id".into(), Value::from(plan.id));
    map.insert("group_id".into(), plan.group_id.map(Value::from).unwrap_or(Value::Null));
    map.insert("name".into(), Value::from(plan.name.clone()));
    map.insert("tags".into(), plan.tags_value());
    map.insert("content".into(), Value::from(content));
    for (k, v) in periods {
        map.insert(k, v);
    }
    map.insert("capacity_limit".into(), capacity);
    map.insert("transfer_enable".into(), Value::from(transfer));
    map.insert("speed_limit".into(), plan.speed_limit.map(Value::from).unwrap_or(Value::Null));
    map.insert("device_limit".into(), plan.device_limit.map(Value::from).unwrap_or(Value::Null));
    map.insert("show".into(), Value::from(plan.show != 0));
    map.insert("sell".into(), Value::from(plan.sell != 0));
    map.insert("renew".into(), Value::from(plan.renew != 0));
    map.insert(
        "reset_traffic_method".into(),
        plan.reset_traffic_method.map(Value::from).unwrap_or(Value::Null),
    );
    map.insert("sort".into(), plan.sort.map(Value::from).unwrap_or(Value::Null));
    map.insert("created_at".into(), Value::from(plan.created_at));
    map.insert("updated_at".into(), Value::from(plan.updated_at));
    Value::Object(map)
}

pub async fn comm_config(State(state): State<SharedState>) -> ApiResult<Response> {
    let s = state.settings().await;
    let captcha_type = s.str("captcha_type", "recaptcha");
    let mut data = json!({
        "tos_url": s.value("tos_url"),
        "is_email_verify": if s.bool("email_verify", false) { 1 } else { 0 },
        "is_invite_force": if s.bool("invite_force", false) { 1 } else { 0 },
        "email_whitelist_suffix": if s.bool("email_whitelist_enable", false) {
            Value::Array(s.arr("email_whitelist_suffix"))
        } else { Value::from(0) },
        "is_captcha": if s.bool("captcha_enable", false) { 1 } else { 0 },
        "captcha_type": captcha_type.clone(),
        "app_description": s.value("app_description"),
        "app_url": s.value("app_url"),
        "logo": s.value("logo"),
        "is_recaptcha": if s.bool("captcha_enable", false) { 1 } else { 0 },
    });
    // 仅下发当前启用的验证码类型所需的公钥，不暴露未使用类型的配置
    let captcha_public = match captcha_type.as_str() {
        "turnstile" => json!({ "turnstile_site_key": s.value("turnstile_site_key") }),
        "recaptcha-v3" => json!({
            "recaptcha_v3_site_key": s.value("recaptcha_v3_site_key"),
            "recaptcha_v3_score_threshold": s.f64("recaptcha_v3_score_threshold", 0.5),
        }),
        "recaptcha" => json!({ "recaptcha_site_key": s.value("recaptcha_site_key") }),
        _ => json!({}),
    };
    if let (Some(map), Some(extra)) = (data.as_object_mut(), captcha_public.as_object()) {
        for (key, value) in extra {
            map.insert(key.clone(), value.clone());
        }
    }
    let app_url = if s.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        s.str("app_url", "")
    };
    let ctx = PluginContext::new(&state.db, &s, &app_url, json!({}));
    let filtered = state
        .plugin
        .filter_hook("guest_comm_config", data.clone(), json!([]), &ctx)
        .await;
    Ok(ok(if filtered.is_object() { filtered } else { data }))
}

/// Payment gateway callback. Signature verification happens in the native
/// payment plugin (with the built-in ePay verifier as a fallback).
pub async fn payment_notify(
    State(state): State<SharedState>,
    Path((method, uuid)): Path<(String, String)>,
    Query(query): Query<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> ApiResult<Response> {
    let raw_body = String::from_utf8_lossy(&body).to_string();
    let header_map: serde_json::Map<String, Value> = headers
        .iter()
        .filter_map(|(key, value)| {
            value.to_str().ok().map(|value| {
                (
                    key.as_str().to_ascii_lowercase(),
                    Value::String(value.to_string()),
                )
            })
        })
        .collect();
    let mut params: HashMap<String, Value> = query
        .into_iter()
        .map(|(k, v)| (k, Value::String(v)))
        .collect();
    let content_type = headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if content_type.contains("application/json") {
        if let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(&body) {
            for (k, v) in map {
                params.insert(k, v);
            }
        }
    } else if let Ok(pairs) = serde_urlencoded::from_bytes::<Vec<(String, String)>>(&body) {
        for (k, v) in pairs {
            params.insert(k, Value::String(v));
        }
    }

    let payment: Option<crate::models::PaymentRow> =
        sqlx::query_as("SELECT * FROM v2_payment WHERE uuid = ? LIMIT 1")
            .bind(&uuid)
            .fetch_optional(&state.db)
            .await?;
    let payment = payment.ok_or_else(|| ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "payment not found"))?;
    if payment.enable == 0 {
        return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "gate is not enable"));
    }
    if !payment.payment.eq_ignore_ascii_case(&method) {
        return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "payment mismatch"));
    }

    let config = payment.config_value();
    let settings = state.settings().await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = PluginContext::new(&state.db, &settings, &app_url, json!({}));
    let verified = state
        .plugin
        .payment_notify(
            &payment.payment,
            config.clone(),
            params.clone(),
            &raw_body,
            Value::Object(header_map),
            &ctx,
        )
        .await
        .or_else(|| builtin_epay_notify(&config, &params));

    let Some(verified) = verified else {
        return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "verify error"));
    };
    if verified == Value::Bool(false) || verified.is_null() {
        return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "verify error"));
    }
    if verified
        .get("pending")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        let custom = verified
            .get("custom_result")
            .and_then(|v| v.as_str())
            .unwrap_or("pending");
        return Ok((StatusCode::OK, custom.to_string()).into_response());
    }

    let trade_no = verified
        .get("trade_no")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let callback_no = verified
        .get("callback_no")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    if trade_no.is_empty() {
        return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "verify error"));
    }

    if let Some(paid) = verified.get("paid_amount").and_then(|v| v.as_i64()) {
        let order: Option<(i64, Option<i64>)> =
            sqlx::query_as("SELECT total_amount, handling_amount FROM v2_order WHERE trade_no = ?")
                .bind(&trade_no)
                .fetch_optional(&state.db)
                .await?;
        if let Some((total, handling)) = order {
            let expected = total + handling.unwrap_or(0);
            if paid < expected {
                return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "amount mismatch"));
            }
        }
    }

    if let Some(custom) = verified.get("custom_result").and_then(|v| v.as_str()) {
        // Custom success body (some gateways expect plain text).
        if !crate::api::order::mark_order_paid(&state, &trade_no, &callback_no).await? {
            return Err(ApiError::bad_request("handle error"));
        }
        return Ok((StatusCode::OK, custom.to_string()).into_response());
    }

    if !crate::api::order::mark_order_paid(&state, &trade_no, &callback_no).await? {
        return Err(ApiError::bad_request("handle error"));
    }
    Ok((StatusCode::OK, "success").into_response())
}

fn builtin_epay_notify(config: &Value, params: &HashMap<String, Value>) -> Option<Value> {
    let key = config.get("key").and_then(|v| v.as_str())?;
    let sign = params.get("sign")?.as_str()?.to_string();
    let mut filtered: Vec<(String, String)> = params
        .iter()
        .filter(|(k, _)| k.as_str() != "sign" && k.as_str() != "sign_type")
        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
        .collect();
    filtered.sort_by(|a, b| a.0.cmp(&b.0));
    let query = filtered
        .iter()
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let expected = crate::util::md5_hex(&format!("{query}{key}"));
    if expected != sign {
        return None;
    }
    let status = params.get("trade_status")?.as_str()?;
    if status != "TRADE_SUCCESS" {
        return None;
    }
    Some(json!({
        "trade_no": params.get("out_trade_no")?.as_str()?,
        "callback_no": params.get("trade_no").and_then(|v| v.as_str()).unwrap_or(""),
    }))
}

pub async fn telegram_webhook(
    State(state): State<SharedState>,
    Query(query): Query<StdHashMap<String, String>>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let settings = state.settings().await;
    let bot_token = settings.str("telegram_bot_token", "");
    if bot_token.is_empty() {
        return Err(ApiError::new(StatusCode::UNAUTHORIZED, "telegram bot is not configured"));
    }
    let expected = crate::util::md5_hex(&bot_token);
    let provided = query.get("access_token").cloned().unwrap_or_default();
    if provided != expected {
        return Err(ApiError::new(StatusCode::UNAUTHORIZED, "access_token is error"));
    }
    let enabled = state
        .plugin
        .is_enabled_or(&state.db, "telegram", false)
        .await;
    let config = state.plugin.config(&state.db, "telegram").await;
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = PluginContext::new(&state.db, &settings, &app_url, config);
    let result = state
        .plugin
        .telegram_webhook(&body, enabled, &ctx)
        .await;
    Ok(ok(result))
}

pub async fn get_payment_methods(State(state): State<SharedState>) -> ApiResult<Response> {
    Ok(ok(state.plugin.payment_methods().await))
}

pub async fn app_config(State(state): State<SharedState>) -> ApiResult<Response> {
    let s = state.settings().await;
    Ok(ok(json!({
        "app_name": s.str("app_name", "Fboard"),
        "app_description": s.str("app_description", ""),
        "app_url": s.str("app_url", ""),
        "logo": s.str("logo", ""),
        "tos_url": s.value("tos_url"),
        "is_email_verify": s.bool("email_verify", false),
    })))
}

pub async fn app_version() -> ApiResult<Response> {
    Ok(ok(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "channel": "rust",
    })))
}

pub async fn stripe_public_key(
    State(state): State<SharedState>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let id = body.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let payment: Option<(String,)> =
        sqlx::query_as("SELECT config FROM v2_payment WHERE id = ? AND payment = 'StripeCredit'")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;
    let Some((config,)) = payment else {
        return Err(ApiError::bad_request("payment is not found"));
    };
    let config: Value = serde_json::from_str(&config).unwrap_or_default();
    Ok(ok(config.get("stripe_pk_live").cloned().unwrap_or(Value::Null)))
}

pub async fn try_get_str(body: &Value, key: &str) -> Option<String> {
    str_field(body, key)
}
