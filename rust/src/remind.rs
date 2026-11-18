use std::collections::HashMap;
use std::time::Duration;

use crate::state::SharedState;
use crate::util;

/// Periodically send subscription expiry / traffic reminders. Mirrors the
/// `send:remindMail` command and `MailService` thresholds from the Laravel
/// panel: expire within 24 hours, traffic between 80% and 100%.
pub async fn scheduler(state: SharedState) {
    // Give the HTTP server a moment to start before doing heavy queries.
    tokio::time::sleep(Duration::from_secs(30)).await;
    loop {
        if let Err(err) = run_once(&state).await {
            tracing::warn!(error = %err, "reminder mail pass failed");
        }
        tokio::time::sleep(Duration::from_secs(1800)).await;
    }
}

struct ReminderUser {
    id: i64,
    email: String,
    expired_at: Option<i64>,
    transfer_enable: i64,
    u: i64,
    d: i64,
    remind_expire: i64,
    remind_traffic: i64,
}

pub async fn run_once(state: &SharedState) -> anyhow::Result<(usize, usize)> {
    let settings = state.settings().await;
    if !settings.bool("remind_mail_enable", false) {
        return Ok((0, 0));
    }
    if settings.str("email_host", "").is_empty() && settings.str("mail_host", "").is_empty() {
        return Ok((0, 0));
    }

    let rows: Vec<(i64, String, Option<i64>, i64, i64, i64, i64, i64)> = sqlx::query_as(
        "SELECT id, email, expired_at, transfer_enable, u, d, remind_expire, remind_traffic \
         FROM v2_user \
         WHERE banned = 0 AND email IS NOT NULL AND email <> '' \
           AND (remind_expire = 1 OR remind_traffic = 1) \
         ORDER BY id ASC",
    )
    .fetch_all(&state.db)
    .await?;

    let users: Vec<ReminderUser> = rows
        .into_iter()
        .map(
            |(id, email, expired_at, transfer_enable, u, d, remind_expire, remind_traffic)| {
                ReminderUser {
                    id,
                    email,
                    expired_at,
                    transfer_enable,
                    u,
                    d,
                    remind_expire,
                    remind_traffic,
                }
            },
        )
        .collect();

    let now = util::now();
    let app_name = settings.str("app_name", "Fboard");
    let app_url = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let mut vars = HashMap::new();
    vars.insert("name".into(), app_name.clone());
    vars.insert("url".into(), app_url);

    let mut expire_sent = 0usize;
    let mut traffic_sent = 0usize;

    for user in users {
        if user.remind_expire == 1 {
            if let Some(expired_at) = user.expired_at {
                let within_day = expired_at > now && expired_at - 86400 < now;
                let flag = format!("LAST_SEND_EMAIL_REMIND_EXPIRE_{}", user.id);
                if within_day && state.cache.get(&flag).is_none() {
                    let subject = format!("{app_name} 服务即将到期");
                    match crate::mail::send_template(
                        &state.db,
                        &settings,
                        &user.email,
                        "remindExpire",
                        &subject,
                        &vars,
                    )
                    .await
                    {
                        Ok(()) => {
                            state.cache.put(flag, "1", 24 * 3600);
                            expire_sent += 1;
                        }
                        Err(err) => {
                            tracing::warn!(error = %err, email = %user.email, "expire reminder failed");
                        }
                    }
                }
            }
        }

        if user.remind_traffic == 1 && user.transfer_enable > 0 {
            let used = user.u + user.d;
            if used > 0 {
                let ratio = used as f64 / user.transfer_enable as f64;
                let flag = format!("LAST_SEND_EMAIL_REMIND_TRAFFIC_{}", user.id);
                if (0.8..1.0).contains(&ratio) && state.cache.get(&flag).is_none() {
                    let subject = format!("{app_name} 流量使用已达到 80%");
                    match crate::mail::send_template(
                        &state.db,
                        &settings,
                        &user.email,
                        "remindTraffic",
                        &subject,
                        &vars,
                    )
                    .await
                    {
                        Ok(()) => {
                            state.cache.put(flag, "1", 24 * 3600);
                            traffic_sent += 1;
                        }
                        Err(err) => {
                            tracing::warn!(error = %err, email = %user.email, "traffic reminder failed");
                        }
                    }
                }
            }
        }
    }

    if expire_sent > 0 || traffic_sent > 0 {
        tracing::info!(expire_sent, traffic_sent, "reminder mails sent");
    }
    Ok((expire_sent, traffic_sent))
}

#[cfg(test)]
mod tests {
    #[test]
    fn traffic_window_is_eighty_to_hundred() {
        let ratio_ok = 0.8f64;
        let ratio_low = 0.79f64;
        let ratio_full = 1.0f64;
        assert!((0.8..1.0).contains(&ratio_ok));
        assert!(!(0.8..1.0).contains(&ratio_low));
        assert!(!(0.8..1.0).contains(&ratio_full));
    }
}
