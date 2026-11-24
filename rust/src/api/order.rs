use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};

use crate::api::guest::plan_json;
use crate::api::subscribe::build_servers;
use crate::error::{ApiError, ApiResult};
use crate::models::{OrderRow, PaymentRow, PlanRow, UserRow};
use crate::plugin::PluginContext;
use crate::response::{ok, str_field};
use crate::state::{AuthUser, SharedState};
use crate::util;

const STATUS_PENDING: i64 = 0;
const STATUS_PROCESSING: i64 = 1;
const STATUS_CANCELLED: i64 = 2;
const STATUS_COMPLETED: i64 = 3;
const STATUS_DISCOUNTED: i64 = 4;

const TYPE_NEW_PURCHASE: i64 = 1;
const TYPE_RENEWAL: i64 = 2;
const TYPE_UPGRADE: i64 = 3;
const TYPE_RESET_TRAFFIC: i64 = 4;
const TYPE_DEPOSIT: i64 = 5;

const PERIOD_DEPOSIT: &str = "deposit";

fn legacy_period(period: &str) -> &str {
    match period {
        "hourly" => "hour_price",
        "daily" => "day_price",
        "monthly" => "month_price",
        "quarterly" => "quarter_price",
        "half_yearly" => "half_year_price",
        "yearly" => "year_price",
        "two_yearly" => "two_year_price",
        "three_yearly" => "three_year_price",
        "onetime" => "onetime_price",
        "reset_traffic" => "reset_price",
        other => other,
    }
}

fn new_period(period: &str) -> String {
    match period {
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
    }
    .to_string()
}

fn order_json(order: &OrderRow, plan: Option<&PlanRow>, payment: Option<&PaymentRow>, settings: &crate::settings::Settings) -> Value {
    let is_deposit = order.period == PERIOD_DEPOSIT || order.r#type == TYPE_DEPOSIT;
    json!({
        "id": order.id,
        "invite_user_id": order.invite_user_id,
        "user_id": order.user_id,
        "plan_id": order.plan_id,
        "coupon_id": order.coupon_id,
        "payment_id": order.payment_id,
        "type": order.r#type,
        "period": if is_deposit { PERIOD_DEPOSIT } else { legacy_period(&order.period) },
        "trade_no": order.trade_no,
        "callback_no": order.callback_no,
        "total_amount": order.total_amount,
        "handling_amount": order.handling_amount,
        "discount_amount": order.discount_amount,
        "surplus_amount": order.surplus_amount,
        "surplus_credit": order.surplus_credit,
        "balance_amount": order.balance_amount,
        "surplus_order_ids": order.surplus_order_ids,
        "status": order.status,
        "commission_status": order.commission_status,
        "commission_balance": order.commission_balance,
        "actual_commission_balance": order.actual_commission_balance,
        "paid_at": order.paid_at,
        "created_at": order.created_at,
        "updated_at": order.updated_at,
        "plan": if is_deposit {
            json!({"id": 0, "name": "deposit"})
        } else {
            plan.map(|p| plan_json(p, settings)).unwrap_or(Value::Null)
        },
        "payment": payment.map(|p| json!({
            "id": p.id, "name": p.name, "payment": p.payment, "icon": p.icon
        })).unwrap_or(Value::Null),
    })
}

pub async fn fetch(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let status: Option<i64> = query.get("status").and_then(|v| v.parse().ok());
    let rows: Vec<OrderRow> = match status {
        Some(status) => {
            sqlx::query_as("SELECT * FROM v2_order WHERE user_id = ? AND status = ? ORDER BY created_at DESC")
                .bind(user.id)
                .bind(status)
                .fetch_all(&state.db)
                .await?
        }
        None => {
            sqlx::query_as("SELECT * FROM v2_order WHERE user_id = ? ORDER BY created_at DESC")
                .bind(user.id)
                .fetch_all(&state.db)
                .await?
        }
    };
    let settings = state.settings().await;
    let mut out = Vec::new();
    for order in rows {
        let plan: Option<PlanRow> = sqlx::query_as("SELECT * FROM v2_plan WHERE id = ?")
            .bind(order.plan_id)
            .fetch_optional(&state.db)
            .await?;
        let payment: Option<PaymentRow> = match order.payment_id {
            Some(id) => sqlx::query_as("SELECT * FROM v2_payment WHERE id = ?")
                .bind(id)
                .fetch_optional(&state.db)
                .await?,
            None => None,
        };
        out.push(order_json(&order, plan.as_ref(), payment.as_ref(), &settings));
    }
    Ok(ok(Value::Array(out)))
}

pub async fn detail(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let trade_no = query
        .get("trade_no")
        .cloned()
        .ok_or_else(|| ApiError::bad_request("订单不存在或已支付"))?;
    let order: Option<OrderRow> = sqlx::query_as(
        "SELECT * FROM v2_order WHERE user_id = ? AND trade_no = ? LIMIT 1",
    )
    .bind(user.id)
    .bind(&trade_no)
    .fetch_optional(&state.db)
    .await?;
    let order = order.ok_or_else(|| ApiError::bad_request("订单不存在或已支付"))?;
    let plan: Option<PlanRow> = sqlx::query_as("SELECT * FROM v2_plan WHERE id = ?")
        .bind(order.plan_id)
        .fetch_optional(&state.db)
        .await?;
    let payment: Option<PaymentRow> = match order.payment_id {
        Some(id) => sqlx::query_as("SELECT * FROM v2_payment WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?,
        None => None,
    };
    let settings = state.settings().await;
    let mut data = order_json(&order, plan.as_ref(), payment.as_ref(), &settings);
    data["try_out_plan_id"] = json!(settings.i64("try_out_plan_id", 0));
    Ok(ok(data))
}

pub async fn save(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let plan_id = body
        .get("plan_id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| ApiError::bad_request("订阅不存在"))?;
    let period = str_field(&body, "period").ok_or_else(|| ApiError::bad_request("周期不能为空"))?;
    let period = new_period(&period);
    let coupon_code = str_field(&body, "coupon_code").filter(|v| !v.is_empty());

    let existing: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM v2_order WHERE user_id = ? AND status IN (0, 1)",
    )
    .bind(user.id)
    .fetch_one(&state.db)
    .await?;
    if existing.0 > 0 {
        return Err(ApiError::bad_request("您有未支付或处理中的订单，请稍后再试或取消"));
    }

    let plan: Option<PlanRow> = sqlx::query_as("SELECT * FROM v2_plan WHERE id = ?")
        .bind(plan_id)
        .fetch_optional(&state.db)
        .await?;
    let plan = plan.ok_or_else(|| ApiError::bad_request("订阅不存在"))?;

    let mut tx = state.db.begin().await?;
    let locked: Option<UserRow> = sqlx::query_as("SELECT * FROM v2_user WHERE id = ? FOR UPDATE")
        .bind(user.id)
        .fetch_optional(&mut *tx)
        .await?;
    let locked = locked.ok_or_else(|| ApiError::bad_request("用户不存在"))?;

    let price = plan
        .prices_value()
        .get(&period)
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    if price <= 0.0 {
        return Err(ApiError::bad_request("该周期暂不可购买"));
    }
    let mut total_amount = util::yuan_to_cents(price);
    let mut discount_amount: i64 = 0;
    let mut coupon_id: Option<i64> = None;

    if let Some(code) = coupon_code {
        let coupon = check_coupon(&state, &code, plan_id, &period, user.id).await?;
        discount_amount = match coupon.r#type {
            1 => coupon.value,
            2 => (total_amount * coupon.value / 100).max(0),
            _ => 0,
        }
        .min(total_amount);
        coupon_id = Some(coupon.id);
        total_amount -= discount_amount;
        if let Some(limit) = coupon.limit_use {
            if limit <= 0 {
                return Err(ApiError::bad_request("优惠券已用完"));
            }
        }
    }

    // VIP discount
    if let Some(rate) = locked.discount {
        let vip = (total_amount * rate / 100).max(0);
        discount_amount += vip;
        total_amount = (total_amount - vip).max(0);
    }

    // Order type
    let order_type = if period == "reset_traffic" {
        TYPE_RESET_TRAFFIC
    } else if locked.plan_id.is_some()
        && locked.plan_id != Some(plan.id)
        && (locked.expired_at.is_none() || locked.expired_at.unwrap_or(0) > util::now())
    {
        if !state.settings().await.bool("plan_change_enable", true) {
            return Err(ApiError::bad_request("目前不允许更改订阅，请联系客服"));
        }
        TYPE_UPGRADE
    } else if (locked.expired_at.is_none() || locked.expired_at.unwrap_or(0) > util::now())
        && locked.plan_id == Some(plan.id)
    {
        TYPE_RENEWAL
    } else {
        TYPE_NEW_PURCHASE
    };

    // Balance deduction
    let mut balance_amount = 0i64;
    if locked.balance > 0 && total_amount > 0 {
        balance_amount = locked.balance.min(total_amount);
        total_amount -= balance_amount;
        sqlx::query("UPDATE v2_user SET balance = balance - ?, updated_at = ? WHERE id = ?")
            .bind(balance_amount)
            .bind(util::now())
            .bind(locked.id)
            .execute(&mut *tx)
            .await?;
    }

    // Invite commission (first order or every period).
    let mut invite_user_id = locked.invite_user_id;
    let mut commission_balance = 0i64;
    if let Some(inviter_id) = invite_user_id {
        let inviter: Option<(i64, Option<i64>, i64)> =
            sqlx::query_as("SELECT id, commission_rate, commission_type FROM v2_user WHERE id = ?")
                .bind(inviter_id)
                .fetch_optional(&mut *tx)
                .await?;
        if let Some((_, rate, commission_type)) = inviter {
            let settings = state.settings().await;
            let commission_type = if commission_type == 0 {
                if settings.bool("commission_first_time_enable", true) {
                    2
                } else {
                    1
                }
            } else {
                commission_type
            };
            let should = if commission_type == 1 {
                true
            } else {
                let prior: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM v2_order WHERE user_id = ? AND status NOT IN (0, 2) AND period != ? AND type != ?",
                )
                .bind(locked.id)
                .bind(PERIOD_DEPOSIT)
                .bind(TYPE_DEPOSIT)
                .fetch_one(&mut *tx)
                .await?;
                prior.0 == 0
            };
            if should && total_amount > 0 {
                let pct = rate.unwrap_or(settings.i64("invite_commission", 10));
                commission_balance = total_amount * pct / 100;
            }
        }
    } else {
        invite_user_id = None;
    }

    // Credit old orders (upgrade)
    let mut surplus_amount = 0i64;
    let mut surplus_order_ids: Option<String> = None;
    if order_type == TYPE_UPGRADE {
        if let Some(expired) = locked.expired_at {
            let remaining_days = ((expired - util::now()).max(0) as f64) / 86400.0;
            let old_total: (i64,) = sqlx::query_as(
                "SELECT COALESCE(SUM(total_amount + balance_amount), 0) FROM v2_order WHERE user_id = ? AND status = 3 AND period NOT IN (?, 'reset_traffic', 'onetime') AND type != ?",
            )
            .bind(locked.id)
            .bind(PERIOD_DEPOSIT)
            .bind(TYPE_DEPOSIT)
            .fetch_one(&mut *tx)
            .await?;
            if remaining_days > 0.0 && old_total.0 > 0 {
                surplus_amount = ((old_total.0 as f64) * (remaining_days / 30.0)).round() as i64;
                surplus_amount = surplus_amount.min(total_amount);
            }
            let ids: Vec<(i64,)> = sqlx::query_as(
                "SELECT id FROM v2_order WHERE user_id = ? AND status = 3 AND period NOT IN ('reset_traffic', 'onetime', ?) AND type != ?",
            )
            .bind(locked.id)
            .bind(PERIOD_DEPOSIT)
            .bind(TYPE_DEPOSIT)
            .fetch_all(&mut *tx)
            .await?;
            if !ids.is_empty() {
                surplus_order_ids = Some(
                    serde_json::to_string(&ids.into_iter().map(|(id,)| id).collect::<Vec<_>>())
                        .unwrap_or_else(|_| "[]".into()),
                );
            }
        }
        total_amount = (total_amount - surplus_amount).max(0);
    }

    let trade_no = util::generate_order_no();
    let now = util::now();
    sqlx::query(
        r#"INSERT INTO v2_order
           (invite_user_id, user_id, plan_id, coupon_id, payment_id, type, period, trade_no,
            total_amount, handling_amount, discount_amount, surplus_amount, surplus_credit,
            balance_amount, surplus_order_ids, status, commission_status, commission_balance,
            created_at, updated_at)
           VALUES (?, ?, ?, ?, NULL, ?, ?, ?, ?, NULL, ?, ?, NULL, ?, ?, ?, 0, ?, ?, ?)"#,
    )
    .bind(invite_user_id)
    .bind(locked.id)
    .bind(plan.id)
    .bind(coupon_id)
    .bind(order_type)
    .bind(&period)
    .bind(&trade_no)
    .bind(total_amount)
    .bind(discount_amount)
    .bind(surplus_amount)
    .bind(balance_amount)
    .bind(surplus_order_ids)
    .bind(STATUS_PENDING)
    .bind(commission_balance)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    if let Some(id) = coupon_id {
        sqlx::query("UPDATE v2_coupon SET limit_use = limit_use - 1, updated_at = ? WHERE id = ? AND limit_use IS NOT NULL")
            .bind(now)
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;

    Ok(ok(json!(trade_no)))
}

async fn check_coupon(
    state: &SharedState,
    code: &str,
    plan_id: i64,
    period: &str,
    user_id: i64,
) -> ApiResult<CouponRow> {
    let coupon: Option<CouponRow> = sqlx::query_as("SELECT * FROM v2_coupon WHERE code = ? LIMIT 1")
        .bind(code)
        .fetch_optional(&state.db)
        .await?;
    let coupon = coupon.ok_or_else(|| ApiError::bad_request("优惠券无效"))?;
    let now = util::now();
    if coupon.show == 0 {
        return Err(ApiError::bad_request("优惠券无效"));
    }
    if let Some(limit) = coupon.limit_use {
        if limit <= 0 {
            return Err(ApiError::bad_request("优惠券已用完"));
        }
    }
    if now < coupon.started_at {
        return Err(ApiError::bad_request("优惠券尚未开始"));
    }
    if now > coupon.ended_at {
        return Err(ApiError::bad_request("优惠券已过期"));
    }
    if let Some(ids) = &coupon.limit_plan_ids {
        if let Ok(list) = serde_json::from_str::<Vec<i64>>(ids) {
            if !list.is_empty() && !list.contains(&plan_id) {
                return Err(ApiError::bad_request("该优惠券不能用于此订阅"));
            }
        }
    }
    if let Some(list) = &coupon.limit_period {
        if let Ok(list) = serde_json::from_str::<Vec<String>>(list) {
            if !list.is_empty() && !list.contains(&period.to_string()) {
                return Err(ApiError::bad_request("该优惠券不能用于此周期"));
            }
        }
    }
    if let Some(limit) = coupon.limit_use_with_user {
        let used: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM v2_order WHERE coupon_id = ? AND user_id = ? AND status != 2",
        )
        .bind(coupon.id)
        .bind(user_id)
        .fetch_one(&state.db)
        .await?;
        if used.0 >= limit {
            return Err(ApiError::bad_request("该优惠券已达到使用上限"));
        }
    }
    Ok(coupon)
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct CouponRow {
    id: i64,
    code: String,
    name: String,
    r#type: i64,
    value: i64,
    show: i64,
    limit_use: Option<i64>,
    limit_use_with_user: Option<i64>,
    limit_plan_ids: Option<String>,
    limit_period: Option<String>,
    started_at: i64,
    ended_at: i64,
}

pub async fn coupon_check(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let code = str_field(&body, "code").ok_or_else(|| ApiError::bad_request("优惠券不能为空"))?;
    let plan_id = body.get("plan_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let period = new_period(&str_field(&body, "period").unwrap_or_default());
    let coupon = check_coupon(&state, &code, plan_id, &period, user.id).await?;
    Ok(ok(json!({
        "id": coupon.id,
        "code": coupon.code,
        "name": coupon.name,
        "type": coupon.r#type,
        "value": coupon.value,
        "show": coupon.show != 0,
        "limit_use": coupon.limit_use,
        "limit_use_with_user": coupon.limit_use_with_user,
        "started_at": coupon.started_at,
        "ended_at": coupon.ended_at,
    })))
}

pub async fn checkout(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let trade_no = str_field(&body, "trade_no").ok_or_else(|| ApiError::bad_request("订单不存在"))?;
    let order: Option<OrderRow> = sqlx::query_as(
        "SELECT * FROM v2_order WHERE trade_no = ? AND user_id = ? AND status = 0 LIMIT 1",
    )
    .bind(&trade_no)
    .bind(user.id)
    .fetch_optional(&state.db)
    .await?;
    let order = order.ok_or_else(|| ApiError::bad_request("订单不存在或已支付"))?;

    if order.total_amount <= 0 {
        mark_order_paid(&state, &order.trade_no, &order.trade_no).await?;
        return Ok(axum::Json(json!({"type": -1, "data": true})).into_response());
    }

    let method = body
        .get("method")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| ApiError::bad_request("请选择支付方式"))?;
    let payment: Option<PaymentRow> = sqlx::query_as("SELECT * FROM v2_payment WHERE id = ?")
        .bind(method)
        .fetch_optional(&state.db)
        .await?;
    let payment = payment.ok_or_else(|| ApiError::bad_request("支付方式不可用"))?;
    if payment.enable == 0 {
        return Err(ApiError::bad_request("支付方式不可用"));
    }

    let handling = if payment.handling_fee_fixed.unwrap_or(0) != 0
        || payment.handling_fee_percent_f64() != 0.0
    {
        Some(
            ((order.total_amount as f64) * (payment.handling_fee_percent_f64() / 100.0)
                + payment.handling_fee_fixed.unwrap_or(0) as f64)
                .round() as i64,
        )
    } else {
        None
    };
    sqlx::query("UPDATE v2_order SET payment_id = ?, handling_amount = ?, updated_at = ? WHERE id = ?")
        .bind(payment.id)
        .bind(handling)
        .bind(util::now())
        .bind(order.id)
        .execute(&state.db)
        .await?;

    let settings = state.settings().await;
    let payable = order.total_amount + handling.unwrap_or(0);
    let notify_base = payment
        .notify_domain
        .clone()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| {
            if settings.str("app_url", "").is_empty() {
                state.cfg.app_url.clone()
            } else {
                settings.str("app_url", "")
            }
        });
    let notify_url = format!(
        "{}/api/v1/guest/payment/notify/{}/{}",
        notify_base.trim_end_matches('/'),
        payment.payment,
        payment.uuid
    );
    let return_url = format!(
        "{}/#/order/{}",
        notify_base.trim_end_matches('/'),
        order.trade_no
    );

    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let ctx = PluginContext::new(&state.db, &settings, &app_url, json!({}));
    let result = state
        .plugin
        .payment_pay(
            &payment.payment,
            payment.config_value(),
            json!({
                "notify_url": notify_url,
                "return_url": return_url,
                "trade_no": order.trade_no,
                "total_amount": payable,
                "user_id": order.user_id,
                "stripe_token": body.get("token").cloned().unwrap_or(Value::Null),
            }),
            &ctx,
        )
        .await;

    let Some(result) = result else {
        tracing::error!(method = %payment.payment, "payment gateway call failed; failing closed");
        return Err(ApiError::bad_request("支付网关调用失败，请稍后重试"));
    };
    let ty = result.get("type").and_then(|v| v.as_i64()).unwrap_or(1);
    Ok(axum::Json(json!({"type": ty, "data": result.get("data").cloned().unwrap_or(Value::Null)}))
        .into_response())
}

pub async fn check(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let trade_no = query
        .get("trade_no")
        .cloned()
        .ok_or_else(|| ApiError::bad_request("订单不存在"))?;
    let row: Option<(i64,)> =
        sqlx::query_as("SELECT status FROM v2_order WHERE trade_no = ? AND user_id = ?")
            .bind(&trade_no)
            .bind(user.id)
            .fetch_optional(&state.db)
            .await?;
    let (status,) = row.ok_or_else(|| ApiError::bad_request("订单不存在"))?;
    Ok(ok(json!(status)))
}

pub async fn payment_methods(State(state): State<SharedState>, AuthUser(_user): AuthUser) -> ApiResult<Response> {
    let rows: Vec<PaymentRow> = sqlx::query_as(
        "SELECT * FROM v2_payment WHERE enable = 1 ORDER BY sort ASC, id ASC",
    )
    .fetch_all(&state.db)
    .await?;
    let out: Vec<Value> = rows
        .into_iter()
        .map(|p| {
            json!({
                "id": p.id, "name": p.name, "payment": p.payment, "icon": p.icon,
                "handling_fee_fixed": p.handling_fee_fixed,
                "handling_fee_percent": p.handling_fee_percent.as_ref().and_then(|v| v.to_string().parse::<f64>().ok()),
            })
        })
        .collect();
    Ok(ok(Value::Array(out)))
}

pub async fn cancel(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let trade_no = str_field(&body, "trade_no").ok_or_else(|| ApiError::bad_request("参数错误"))?;
    let order: Option<OrderRow> =
        sqlx::query_as("SELECT * FROM v2_order WHERE trade_no = ? AND user_id = ?")
            .bind(&trade_no)
            .bind(user.id)
            .fetch_optional(&state.db)
            .await?;
    let order = order.ok_or_else(|| ApiError::bad_request("订单不存在"))?;
    if order.status != STATUS_PENDING {
        return Err(ApiError::bad_request("只能取消待支付订单"));
    }
    let mut tx = state.db.begin().await?;
    sqlx::query("UPDATE v2_order SET status = 2, updated_at = ? WHERE id = ?")
        .bind(util::now())
        .bind(order.id)
        .execute(&mut *tx)
        .await?;
    if let Some(credit) = order.balance_amount {
        if credit > 0 {
            sqlx::query("UPDATE v2_user SET balance = balance + ?, updated_at = ? WHERE id = ?")
                .bind(credit)
                .bind(util::now())
                .bind(user.id)
                .execute(&mut *tx)
                .await?;
        }
    }
    if let Some(coupon_id) = order.coupon_id {
        sqlx::query("UPDATE v2_coupon SET limit_use = limit_use + 1, updated_at = ? WHERE id = ? AND limit_use IS NOT NULL")
            .bind(util::now())
            .bind(coupon_id)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(ok(json!(true)))
}

/// Mark an order as paid and open it. Idempotent: already-paid orders return true.
pub async fn mark_order_paid(state: &SharedState, trade_no: &str, callback_no: &str) -> ApiResult<bool> {
    let mut tx = state.db.begin().await?;
    let order: Option<OrderRow> =
        sqlx::query_as("SELECT * FROM v2_order WHERE trade_no = ? FOR UPDATE")
            .bind(trade_no)
            .fetch_optional(&mut *tx)
            .await?;
    let Some(order) = order else {
        tracing::warn!(trade_no, "payment notify: order not found");
        return Ok(false);
    };
    if order.status != STATUS_PENDING {
        tx.commit().await?;
        return Ok(true);
    }
    sqlx::query("UPDATE v2_order SET status = 1, paid_at = ?, callback_no = ?, updated_at = ? WHERE id = ?")
        .bind(util::now())
        .bind(callback_no)
        .bind(util::now())
        .bind(order.id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    open_order(state, order.id).await?;
    {
        let state = state.clone();
        let order_id = order.id;
        tokio::spawn(async move {
            crate::plugin::notify_telegram(
                &state,
                "telegram.payment_notify",
                json!({"order_id": order_id}),
            )
            .await;
        });
    }
    Ok(true)
}

async fn open_order(state: &SharedState, order_id: i64) -> ApiResult<()> {
    let mut tx = state.db.begin().await?;
    let order: Option<OrderRow> =
        sqlx::query_as("SELECT * FROM v2_order WHERE id = ? FOR UPDATE")
            .bind(order_id)
            .fetch_optional(&mut *tx)
            .await?;
    let Some(order) = order else { return Ok(()) };
    if order.status == STATUS_COMPLETED {
        return Ok(());
    }
    if order.status != STATUS_PROCESSING {
        return Ok(());
    }
    let user: Option<UserRow> =
        sqlx::query_as("SELECT * FROM v2_user WHERE id = ? FOR UPDATE")
            .bind(order.user_id)
            .fetch_optional(&mut *tx)
            .await?;
    let Some(mut user) = user else {
        return Err(ApiError::server("用户不存在"));
    };

    if order.period == PERIOD_DEPOSIT || order.r#type == TYPE_DEPOSIT {
        let bonus = deposit_bonus(state, order.total_amount).await;
        let credit = order.total_amount.max(0) + bonus;
        sqlx::query("UPDATE v2_user SET balance = balance + ?, updated_at = ? WHERE id = ?")
            .bind(credit)
            .bind(util::now())
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
    } else {
        let plan: Option<PlanRow> = sqlx::query_as("SELECT * FROM v2_plan WHERE id = ?")
            .bind(order.plan_id)
            .fetch_optional(&mut *tx)
            .await?;
        let plan = plan.ok_or_else(|| ApiError::server("订阅计划不存在"))?;

        if let Some(surplus_credit) = order.surplus_credit {
            if surplus_credit > 0 {
                sqlx::query("UPDATE v2_user SET balance = balance + ? WHERE id = ?")
                    .bind(surplus_credit)
                    .bind(user.id)
                    .execute(&mut *tx)
                    .await?;
            }
        }
        if let Some(ids) = &order.surplus_order_ids {
            if let Ok(list) = serde_json::from_str::<Vec<i64>>(ids) {
                for id in list {
                    sqlx::query(
                        "UPDATE v2_order SET status = 4, updated_at = ? WHERE id = ? AND period != 'deposit' AND type != 5",
                    )
                    .bind(util::now())
                    .bind(id)
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }

        match order.period.as_str() {
            "reset_traffic" => {
                reset_user_traffic(&mut tx, &user).await?;
            }
            "onetime" => {
                reset_user_traffic(&mut tx, &user).await?;
                sqlx::query(
                    "UPDATE v2_user SET transfer_enable = ?, plan_id = ?, group_id = ?, expired_at = NULL, speed_limit = ?, device_limit = ?, updated_at = ? WHERE id = ?",
                )
                .bind((plan.transfer_enable.unwrap_or(0) as i64).saturating_mul(1_073_741_824))
                .bind(plan.id)
                .bind(plan.group_id)
                .bind(plan.speed_limit)
                .bind(plan.device_limit)
                .bind(util::now())
                .bind(user.id)
                .execute(&mut *tx)
                .await?;
            }
            _ => {
                let was_one_time = user.expired_at.is_none();
                if order.r#type == TYPE_UPGRADE {
                    user.expired_at = Some(util::now());
                }
                if was_one_time || order.r#type == TYPE_NEW_PURCHASE {
                    reset_user_traffic(&mut tx, &user).await?;
                }
                let base = user.expired_at.unwrap_or_else(util::now).max(util::now());
                let new_expiry = add_period(base, &order.period)?;
                sqlx::query(
                    "UPDATE v2_user SET transfer_enable = ?, plan_id = ?, group_id = ?, expired_at = ?, speed_limit = ?, device_limit = ?, updated_at = ? WHERE id = ?",
                )
                .bind((plan.transfer_enable.unwrap_or(0) as i64).saturating_mul(1_073_741_824))
                .bind(plan.id)
                .bind(plan.group_id)
                .bind(new_expiry)
                .bind(plan.speed_limit)
                .bind(plan.device_limit)
                .bind(util::now())
                .bind(user.id)
                .execute(&mut *tx)
                .await?;
            }
        }

        // Commission log is recorded when the inviter confirms; keep the
        // order pending for manual confirmation just like the legacy panel.
    }

    sqlx::query("UPDATE v2_order SET status = 3, updated_at = ? WHERE id = ?")
        .bind(util::now())
        .bind(order.id)
        .execute(&mut *tx)
        .await?;
    let user_id = user.id;
    tx.commit().await?;
    crate::traffic::refresh_next_reset(&state.db, user_id).await.ok();
    Ok(())
}

async fn reset_user_traffic(
    tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
    user: &UserRow,
) -> ApiResult<()> {
    let now = util::now();
    sqlx::query("UPDATE v2_user SET u = 0, d = 0, last_reset_at = ?, reset_count = reset_count + 1, updated_at = ? WHERE id = ?")
        .bind(now)
        .bind(now)
        .bind(user.id)
        .execute(&mut **tx)
        .await?;
    let _ = sqlx::query(
        "INSERT INTO v2_traffic_reset_logs (user_id, reset_type, reset_time, old_upload, old_download, old_total, new_upload, new_download, new_total, trigger_source, created_at, updated_at) VALUES (?, 'order', NOW(), ?, ?, ?, 0, 0, 0, 'order', NOW(), NOW())",
    )
    .bind(user.id)
    .bind(user.u)
    .bind(user.d)
    .bind(user.u + user.d)
    .execute(&mut **tx)
    .await;
    Ok(())
}

fn add_period(base: i64, period: &str) -> ApiResult<i64> {
    let dt = chrono::DateTime::from_timestamp(base, 0).unwrap_or_else(chrono::Utc::now);
    let next = match period {
        "hourly" => dt + chrono::Duration::hours(1),
        "daily" => dt + chrono::Duration::days(1),
        "monthly" => dt + chrono::Duration::days(30),
        "quarterly" => dt + chrono::Duration::days(90),
        "half_yearly" => dt + chrono::Duration::days(180),
        "yearly" => dt + chrono::Duration::days(365),
        "two_yearly" => dt + chrono::Duration::days(730),
        "three_yearly" => dt + chrono::Duration::days(1095),
        _ => return Err(ApiError::bad_request("无效的套餐周期")),
    };
    Ok(next.timestamp())
}

async fn deposit_bonus(state: &SharedState, amount: i64) -> i64 {
    let settings = state.settings().await;
    let mut tiers = settings.arr("deposit_bonus");
    if tiers.is_empty() {
        tiers = settings.arr("deposit_bounus");
    }
    let mut bonus = 0i64;
    for tier in tiers {
        let Some(text) = tier.as_str() else { continue };
        let Some((threshold, value)) = text.split_once(':') else {
            continue;
        };
        let Ok(threshold) = threshold.parse::<f64>() else {
            continue;
        };
        let Ok(value) = value.parse::<f64>() else {
            continue;
        };
        if amount >= (threshold * 100.0).round() as i64 {
            bonus = bonus.max((value * 100.0).round() as i64);
        }
    }
    bonus
}

pub async fn balance_deposit_create(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let amount = body
        .get("amount")
        .and_then(|v| v.as_i64())
        .or_else(|| body.get("total_amount").and_then(|v| v.as_i64()))
        .unwrap_or(0);
    let settings = state.settings().await;
    if !settings.bool("deposit_enable", true) {
        return Err(ApiError::bad_request("余额充值未开启"));
    }
    let min = settings.i64("deposit_min_amount", 100);
    let max = settings.i64("deposit_max_amount", 999999900);
    if amount < min || amount > max {
        return Err(ApiError::bad_request("充值金额不在允许范围内"));
    }
    let bonus = deposit_bonus(&state, amount).await;
    let trade_no = util::generate_order_no();
    let now = util::now();
    sqlx::query(
        r#"INSERT INTO v2_order
           (invite_user_id, user_id, plan_id, coupon_id, payment_id, type, period, trade_no,
            total_amount, handling_amount, discount_amount, surplus_amount, surplus_credit,
            balance_amount, surplus_order_ids, status, commission_status, commission_balance,
            created_at, updated_at)
           VALUES (NULL, ?, 0, NULL, NULL, 5, 'deposit', ?, ?, NULL, NULL, ?, NULL, NULL, NULL, 0, 0, 0, ?, ?)"#,
    )
    .bind(user.id)
    .bind(&trade_no)
    .bind(amount)
    .bind(bonus)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;
    Ok(ok(json!({"trade_no": trade_no})))
}

pub async fn balance_deposit_detail(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let trade_no = query
        .get("trade_no")
        .cloned()
        .ok_or_else(|| ApiError::bad_request("订单不存在"))?;
    let order: Option<OrderRow> = sqlx::query_as(
        "SELECT * FROM v2_order WHERE trade_no = ? AND user_id = ? AND period = 'deposit' LIMIT 1",
    )
    .bind(&trade_no)
    .bind(user.id)
    .fetch_optional(&state.db)
    .await?;
    let order = order.ok_or_else(|| ApiError::bad_request("订单不存在"))?;
    Ok(ok(json!({
        "id": order.id,
        "trade_no": order.trade_no,
        "total_amount": order.total_amount,
        "status": order.status,
        "period": "deposit",
        "type": order.r#type,
        "created_at": order.created_at,
    })))
}

pub async fn available_servers_preview(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
) -> ApiResult<Response> {
    Ok(ok(Value::Array(build_servers(&state, &user).await?)))
}
