use std::time::Duration;

use chrono::{Datelike, TimeZone, Timelike};
use serde_json::Value;
use sqlx::MySqlPool;

use crate::models::PlanRow;
use crate::settings::Settings;
use crate::state::SharedState;
use crate::util;

/// Reset method values mirrored from `Plan` / panel settings.
pub const RESET_FIRST_DAY_MONTH: i64 = 0;
pub const RESET_MONTHLY: i64 = 1;
pub const RESET_NEVER: i64 = 2;
pub const RESET_FIRST_DAY_YEAR: i64 = 3;
pub const RESET_YEARLY: i64 = 4;

pub fn resolve_method(plan_method: Option<i64>, settings: &Settings) -> i64 {
    plan_method.unwrap_or_else(|| settings.i64("reset_traffic_method", RESET_MONTHLY))
}

/// Calculate the next reset timestamp for a user based on their plan method.
pub fn next_reset_at(from: i64, method: i64, anchor_day: Option<u32>) -> Option<i64> {
    let dt = chrono::Local
        .timestamp_opt(from, 0)
        .single()
        .unwrap_or_else(chrono::Local::now);
    let next = match method {
        RESET_NEVER => return None,
        RESET_FIRST_DAY_MONTH => {
            let (year, month) = if dt.month() == 12 {
                (dt.year() + 1, 1)
            } else {
                (dt.year(), dt.month() + 1)
            };
            chrono::Local
                .with_ymd_and_hms(year, month as u32, 1, 0, 0, 0)
                .single()?
        }
        RESET_FIRST_DAY_YEAR => chrono::Local
            .with_ymd_and_hms(dt.year() + 1, 1, 1, 0, 0, 0)
            .single()?,
        RESET_YEARLY => shift_months(dt, 12),
        _ => {
            // Monthly anniversary. Keep the anchor day (usually the day the
            // subscription/reset happened), clamped to the target month.
            let anchor = anchor_day.unwrap_or_else(|| dt.day());
            shift_months_anchored(dt, 1, anchor)
        }
    };
    Some(next.timestamp())
}

fn shift_months(dt: chrono::DateTime<chrono::Local>, months: i32) -> chrono::DateTime<chrono::Local> {
    shift_months_anchored(dt, months, dt.day())
}

fn shift_months_anchored(
    dt: chrono::DateTime<chrono::Local>,
    months: i32,
    anchor_day: u32,
) -> chrono::DateTime<chrono::Local> {
    let total = dt.year() * 12 + (dt.month0() as i32) + months;
    let year = total.div_euclid(12);
    let month0 = total.rem_euclid(12) as u32;
    let last_day = days_in_month(year, month0 + 1);
    let day = anchor_day.clamp(1, last_day);
    chrono::Local
        .with_ymd_and_hms(year, month0 + 1, day, dt.hour(), dt.minute(), dt.second())
        .single()
        .unwrap_or(dt)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let (ny, nm) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    let first = chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap_or_default();
    let next = chrono::NaiveDate::from_ymd_opt(ny, nm, 1).unwrap_or_default();
    (next - first).num_days().max(1) as u32
}

/// Compute/refresh `next_reset_at` after a subscription change.
pub async fn refresh_next_reset(pool: &MySqlPool, user_id: i64) -> anyhow::Result<()> {
    let settings = Settings::load(pool).await;
    let user: Option<(Option<i64>, Option<i64>)> =
        sqlx::query_as("SELECT plan_id, expired_at FROM v2_user WHERE id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
    let Some((plan_id, expired_at)) = user else {
        return Ok(());
    };
    let plan: Option<PlanRow> = match plan_id {
        Some(id) => sqlx::query_as("SELECT * FROM v2_plan WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?,
        None => None,
    };
    let method = resolve_method(plan.as_ref().and_then(|p| p.reset_traffic_method), &settings);
    let next = next_reset_at(util::now(), method, None);
    let _ = expired_at;
    sqlx::query("UPDATE v2_user SET next_reset_at = ? WHERE id = ?")
        .bind(next)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// One scheduler pass. Returns the number of users reset.
pub async fn run_reset_pass(state: &SharedState) -> anyhow::Result<usize> {
    let settings = state.settings().await;
    let due: Vec<(i64, Option<i64>, i64, i64, i64, i64, Option<i64>)> = sqlx::query_as(
        "SELECT id, plan_id, u, d, transfer_enable, reset_count, next_reset_at FROM v2_user WHERE next_reset_at IS NOT NULL AND next_reset_at <= ? AND banned = 0",
    )
    .bind(util::now())
    .fetch_all(&state.db)
    .await?;

    let mut reset_count = 0usize;
    for (user_id, plan_id, old_u, old_d, _transfer, _count, _next) in due {
        let plan: Option<PlanRow> = match plan_id {
            Some(id) => sqlx::query_as("SELECT * FROM v2_plan WHERE id = ?")
                .bind(id)
                .fetch_optional(&state.db)
                .await?,
            None => None,
        };
        let method = resolve_method(plan.as_ref().and_then(|p| p.reset_traffic_method), &settings);
        let now = util::now();
        let next = next_reset_at(now, method, None);

        let mut tx = state.db.begin().await?;
        sqlx::query(
            "UPDATE v2_user SET u = 0, d = 0, last_reset_at = ?, next_reset_at = ?, reset_count = reset_count + 1, updated_at = ? WHERE id = ?",
        )
        .bind(now)
        .bind(next)
        .bind(now)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
        let _ = sqlx::query(
            "INSERT INTO v2_traffic_reset_logs (user_id, reset_type, reset_time, old_upload, old_download, old_total, new_upload, new_download, new_total, trigger_source, created_at, updated_at) VALUES (?, 'auto', NOW(), ?, ?, ?, 0, 0, 0, 'scheduler', NOW(), NOW())",
        )
        .bind(user_id)
        .bind(old_u)
        .bind(old_d)
        .bind(old_u + old_d)
        .execute(&mut *tx)
        .await;
        tx.commit().await?;
        reset_count += 1;
    }
    Ok(reset_count)
}

/// Background loop: run every minute, also purge the in-memory cache.
pub async fn scheduler(state: SharedState) {
    let mut ticker = tokio::time::interval(Duration::from_secs(60));
    ticker.tick().await;
    loop {
        ticker.tick().await;
        state.cache.purge_expired();
        match run_reset_pass(&state).await {
            Ok(0) => {}
            Ok(count) => tracing::info!(count, "traffic reset scheduler"),
            Err(err) => tracing::warn!(error = %err, "traffic reset scheduler failed"),
        }
    }
}

/// Convenience: parse `virtual_nodes` from protocol settings (kept for parity
/// with the legacy panel's virtual-node feature).
pub fn virtual_nodes(settings: &Value) -> Vec<Value> {
    settings
        .get("virtual_nodes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, TimeZone};

    fn ts(y: i32, m: u32, d: u32) -> i64 {
        chrono::Local
            .with_ymd_and_hms(y, m, d, 12, 0, 0)
            .single()
            .unwrap()
            .timestamp()
    }

    #[test]
    fn first_day_of_next_month() {
        let next = next_reset_at(ts(2026, 11, 16), RESET_FIRST_DAY_MONTH, None).unwrap();
        let dt = chrono::Local.timestamp_opt(next, 0).single().unwrap();
        assert_eq!((dt.year(), dt.month(), dt.day()), (2026, 12, 1));
    }

    #[test]
    fn monthly_clamps_anchor_day() {
        let next = next_reset_at(ts(2026, 1, 31), RESET_MONTHLY, Some(31)).unwrap();
        let dt = chrono::Local.timestamp_opt(next, 0).single().unwrap();
        assert_eq!((dt.year(), dt.month(), dt.day()), (2026, 2, 28));
    }

    #[test]
    fn never_returns_none() {
        assert!(next_reset_at(ts(2026, 11, 16), RESET_NEVER, None).is_none());
    }

    #[test]
    fn first_day_of_next_year() {
        let next = next_reset_at(ts(2026, 11, 16), RESET_FIRST_DAY_YEAR, None).unwrap();
        let dt = chrono::Local.timestamp_opt(next, 0).single().unwrap();
        assert_eq!((dt.year(), dt.month(), dt.day()), (2027, 1, 1));
    }
}
