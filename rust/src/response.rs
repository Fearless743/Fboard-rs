use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};

use crate::error::{ApiError, ApiResult};

/// Successful API envelope, compatible with Laravel's `ApiResponse::success()`.
pub fn ok(data: Value) -> Response {
    envelope(StatusCode::OK, "success", "操作成功", data)
}

pub fn ok_msg(data: Value, message: &str) -> Response {
    envelope(StatusCode::OK, "success", message, data)
}

pub fn created(data: Value) -> Response {
    envelope(StatusCode::CREATED, "success", "操作成功", data)
}

pub fn text(body: impl Into<String>) -> Response {
    (
        StatusCode::OK,
        [("content-type", "text/plain; charset=utf-8")],
        body.into(),
    )
        .into_response()
}

fn envelope(status: StatusCode, state: &str, message: &str, data: Value) -> Response {
    (
        status,
        Json(json!({
            "status": state,
            "message": message,
            "data": data,
            "error": null,
        })),
    )
        .into_response()
}

/// Laravel paginator payload (admin SPA expects these exact keys).
pub fn paginate(data: Vec<Value>, total: i64, current_page: i64, per_page: i64) -> Response {
    let last_page = if per_page > 0 {
        ((total + per_page - 1) / per_page).max(1)
    } else {
        1
    };
    Json(json!({
        "data": data,
        "total": total,
        "current_page": current_page,
        "per_page": per_page,
        "last_page": last_page,
    }))
    .into_response()
}

/// Read an optional string field from a JSON object.
pub fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(|v| match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(if *b { "1".into() } else { "0".into() }),
        _ => None,
    })
}

pub fn i64_field(v: &Value, key: &str) -> Option<i64> {
    v.get(key).and_then(|v| match v {
        Value::Number(n) => n.as_i64(),
        Value::String(s) => s.parse().ok(),
        Value::Bool(b) => Some(if *b { 1 } else { 0 }),
        _ => None,
    })
}

pub fn f64_field(v: &Value, key: &str) -> Option<f64> {
    v.get(key).and_then(|v| match v {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.parse().ok(),
        _ => None,
    })
}

pub fn bool_field(v: &Value, key: &str) -> Option<bool> {
    v.get(key).map(|v| match v {
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_i64().unwrap_or(0) != 0,
        Value::String(s) => matches!(s.as_str(), "1" | "true" | "yes" | "on"),
        _ => false,
    })
}

/// Parse the `filter` query parameter used by the admin SPA:
/// `filter[0][id]=email&filter[0][value]=x` (Laravel style) or JSON.
pub fn parse_filters(params: &std::collections::HashMap<String, String>) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (k, v) in params {
        // filter[0][id] / filter[0][value]
        if let Some(rest) = k.strip_prefix("filter[") {
            if let Some((idx, field)) = rest.split_once("][") {
                let field = field.trim_end_matches(']');
                if field == "id" || field == "value" {
                    out.push((format!("{idx}:{field}"), v.clone()));
                }
            }
        }
    }
    // group by index
    let mut map: std::collections::BTreeMap<String, (Option<String>, Option<String>)> =
        Default::default();
    for (k, v) in out {
        if let Some((idx, field)) = k.split_once(':') {
            let entry = map.entry(idx.to_string()).or_default();
            if field == "id" {
                entry.0 = Some(v);
            } else {
                entry.1 = Some(v);
            }
        }
    }
    map.into_values()
        .filter_map(|(id, value)| match (id, value) {
            (Some(id), Some(value)) => Some((id, value)),
            _ => None,
        })
        .collect()
}

/// Parse `?sort=field&order=desc` or the SPA's `sort[0][field]` form.
pub fn parse_sort(params: &std::collections::HashMap<String, String>) -> (String, String) {
    let field = params
        .get("sort")
        .cloned()
        .or_else(|| params.get("orderBy").cloned())
        .unwrap_or_else(|| "id".to_string());
    let dir = params
        .get("order")
        .cloned()
        .or_else(|| params.get("orderDirection").cloned())
        .unwrap_or_else(|| "desc".to_string());
    (
        sanitize_ident(&field),
        if dir.eq_ignore_ascii_case("asc") {
            "ASC"
        } else {
            "DESC"
        }
        .to_string(),
    )
}

pub fn sanitize_ident(s: &str) -> String {
    if !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        s.to_string()
    } else {
        "id".to_string()
    }
}

/// Shorthand for 404 when an affected row is zero.
pub fn require_affected(result: sqlx::mysql::MySqlQueryResult, what: &str) -> ApiResult<()> {
    if result.rows_affected() == 0 {
        Err(ApiError::not_found(format!("{what}不存在")))
    } else {
        Ok(())
    }
}
