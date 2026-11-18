use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{json, Value};
use sqlx::Row;

use crate::plugin::{NativePlugin, PluginContext, PluginKind, PluginMeta};
use crate::util;

static TICKET_ID: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:工单提醒.*?#|工单\s*ID\s*[:：]?\s*)(\d+)").expect("ticket id regex")
});

pub struct TelegramPlugin;

fn config_fields() -> Value {
    json!({
        "enable_ticket_notify": {
            "type": "boolean", "label": "开启工单通知", "default": true,
            "description": "是否开启工单创建和回复的 Telegram 通知功能"
        },
        "enable_payment_notify": {
            "type": "boolean", "label": "开启支付通知", "default": true,
            "description": "是否开启支付成功的 Telegram 通知功能"
        },
        "start_welcome_title": {
            "type": "string", "label": "欢迎标题", "default": "🎉 欢迎使用 Fboard Telegram Bot！"
        },
        "start_bot_description": {
            "type": "text", "label": "机器人描述",
            "default": "🤖 我是您的专属助手，可以帮助您：\\n• 绑定您的 Fboard 账号\\n• 查看流量使用情况\\n• 获取最新订阅链接\\n• 管理账号绑定状态"
        },
        "start_bind_guide": {
            "type": "text", "label": "绑定指导",
            "default": "🔗 请先绑定您的 Fboard 账号：\\n1. 登录您的 Fboard 账户\\n2. 复制您的订阅链接\\n3. 发送 /bind + 订阅链接"
        },
        "start_unbind_guide": {
            "type": "text", "label": "已绑定用户命令列表",
            "default": "📋 可用命令：\\n/traffic - 查看流量使用情况\\n/getlatesturl - 获取订阅链接\\n/unbind - 解绑账号"
        },
        "start_bind_commands": {
            "type": "text", "label": "未绑定用户命令列表",
            "default": "📋 可用命令：\\n/bind [订阅链接] - 绑定账号"
        },
        "start_footer": {
            "type": "text", "label": "底部提示", "default": "💡 提示：所有命令都需要在私聊中使用"
        },
        "help_text": {
            "type": "text", "label": "帮助文本",
            "default": "请使用以下命令：\\n/bind - 绑定账号\\n/traffic - 查看流量\\n/getlatesturl - 获取最新链接"
        }
    })
}

fn cfg_string(ctx: &PluginContext<'_>, key: &str, default: &str) -> String {
    ctx.config
        .get(key)
        .and_then(|v| v.as_str())
        .map(|v| v.replace("\\n", "\n"))
        .unwrap_or_else(|| default.replace("\\n", "\n"))
}

fn cfg_bool(ctx: &PluginContext<'_>, key: &str, default: bool) -> bool {
    match ctx.config.get(key) {
        Some(Value::Bool(v)) => *v,
        Some(Value::Number(v)) => v.as_i64().unwrap_or(0) != 0,
        Some(Value::String(v)) => matches!(v.as_str(), "1" | "true" | "yes" | "on"),
        _ => default,
    }
}

async fn send_message(ctx: &PluginContext<'_>, chat_id: i64, text: &str) -> bool {
    let token = ctx.settings.str("telegram_bot_token", "");
    if token.is_empty() {
        return false;
    }
    let base = ctx
        .settings
        .str("telegram_api_base", "https://api.telegram.org");
    let url = format!("{}/bot{}/sendMessage", base.trim_end_matches('/'), token);
    let client = reqwest::Client::new();
    match client
        .post(url)
        .json(&json!({"chat_id": chat_id, "text": text}))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(response) => {
            if !response.status().is_success() {
                let body = response.text().await.unwrap_or_default();
                tracing::warn!(body, "telegram sendMessage failed");
                false
            } else {
                true
            }
        }
        Err(err) => {
            tracing::warn!(error = %err, "telegram sendMessage request failed");
            false
        }
    }
}

fn format_gb(bytes: i64) -> String {
    format!("{:.2}", bytes.max(0) as f64 / 1024.0 / 1024.0 / 1024.0)
}

fn extract_token(url: &str) -> Option<String> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(parsed) = reqwest::Url::parse(trimmed) {
        if let Some(token) = parsed
            .query_pairs()
            .find(|(key, _)| key == "token")
            .map(|(_, value)| value.to_string())
        {
            if !token.is_empty() {
                return Some(token);
            }
        }
        let last = parsed.path_segments()?.filter(|s| !s.is_empty()).last()?;
        return Some(last.to_string());
    }
    let no_query = trimmed.split('?').next().unwrap_or(trimmed);
    no_query
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

async fn handle_bind(ctx: &PluginContext<'_>, chat_id: i64, text: &str) -> bool {
    let Some(url) = text.split_whitespace().nth(1) else {
        send_message(ctx, chat_id, "参数有误，请携带订阅地址发送").await;
        return true;
    };
    let Some(token) = extract_token(url) else {
        send_message(ctx, chat_id, "订阅地址无效").await;
        return true;
    };
    let user: Option<(i64, Option<i64>)> =
        sqlx::query_as("SELECT id, telegram_id FROM v2_user WHERE token = ? LIMIT 1")
            .bind(&token)
            .fetch_optional(ctx.db)
            .await
            .unwrap_or(None);
    let Some((user_id, bound)) = user else {
        send_message(ctx, chat_id, "用户不存在").await;
        return true;
    };
    if bound.is_some() {
        send_message(ctx, chat_id, "该账号已经绑定了Telegram账号").await;
        return true;
    }
    let updated = sqlx::query("UPDATE v2_user SET telegram_id = ?, updated_at = ? WHERE id = ?")
        .bind(chat_id)
        .bind(util::now())
        .bind(user_id)
        .execute(ctx.db)
        .await
        .map(|result| result.rows_affected() > 0)
        .unwrap_or(false);
    if updated {
        send_message(ctx, chat_id, "绑定成功").await;
    } else {
        send_message(ctx, chat_id, "设置失败").await;
    }
    true
}

async fn bound_user(ctx: &PluginContext<'_>, chat_id: i64) -> Option<(i64, String, i64, i64, i64, Option<i64>)> {
    sqlx::query_as(
        "SELECT id, email, u, d, transfer_enable, expired_at FROM v2_user WHERE telegram_id = ? LIMIT 1",
    )
    .bind(chat_id)
    .fetch_optional(ctx.db)
    .await
    .unwrap_or(None)
}

async fn handle_message(ctx: &PluginContext<'_>, message: &Value) -> bool {
    let chat_id = message
        .pointer("/chat/id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    if chat_id == 0 {
        return false;
    }
    let chat_type = message
        .pointer("/chat/type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let text = message.get("text").and_then(|v| v.as_str()).unwrap_or("");
    if text.is_empty() {
        return false;
    }

    // Reply to a ticket notification.
    if let Some(reply) = message.get("reply_to_message") {
        let reply_text = reply.get("text").and_then(|v| v.as_str()).unwrap_or("");
        if let Some(caps) = TICKET_ID.captures(reply_text) {
            if let Some(ticket_id) = caps.get(1).and_then(|m| m.as_str().parse::<i64>().ok()) {
                let user = bound_user(ctx, chat_id).await;
                let Some((user_id, _, _, _, _, _)) = user else {
                    send_message(ctx, chat_id, "请先绑定账号").await;
                    return true;
                };
                let ticket: Option<(i64, i64)> =
                    sqlx::query_as("SELECT id, user_id FROM v2_ticket WHERE id = ? LIMIT 1")
                        .bind(ticket_id)
                        .fetch_optional(ctx.db)
                        .await
                        .unwrap_or(None);
                let Some((ticket_id, ticket_user)) = ticket else {
                    send_message(ctx, chat_id, "工单不存在").await;
                    return true;
                };
                let now = util::now();
                let inserted = sqlx::query(
                    "INSERT INTO v2_ticket_message (user_id, ticket_id, message, is_bot, created_at, updated_at) VALUES (?, ?, ?, 0, ?, ?)",
                )
                .bind(user_id)
                .bind(ticket_id)
                .bind(text)
                .bind(now)
                .bind(now)
                .execute(ctx.db)
                .await
                .is_ok();
                let reply_status = if user_id != ticket_user { 1 } else { 0 };
                let _ = sqlx::query(
                    "UPDATE v2_ticket SET reply_status = ?, last_reply_user_id = ?, updated_at = ? WHERE id = ?",
                )
                .bind(reply_status)
                .bind(user_id)
                .bind(now)
                .bind(ticket_id)
                .execute(ctx.db)
                .await;
                let reply = if inserted {
                    format!("工单 #{ticket_id} 回复成功")
                } else {
                    "工单回复失败".to_string()
                };
                send_message(ctx, chat_id, &reply).await;
                return true;
            }
        }
    }

    let mut iter = text.split_whitespace();
    let raw_command = iter.next().unwrap_or("");
    if !raw_command.starts_with('/') {
        return false;
    }
    let command = raw_command
        .split('@')
        .next()
        .unwrap_or(raw_command)
        .to_ascii_lowercase();

    if chat_type != "private" {
        if matches!(
            command.as_str(),
            "/start" | "/bind" | "/traffic" | "/getlatesturl" | "/unbind"
        ) {
            send_message(ctx, chat_id, "请在私聊中使用此命令").await;
        }
        return true;
    }

    match command.as_str() {
        "/start" => {
            let title = cfg_string(ctx, "start_welcome_title", "🎉 欢迎使用 Fboard Telegram Bot！");
            let description = cfg_string(
                ctx,
                "start_bot_description",
                "🤖 我是您的专属助手，可以帮助您：\n• 绑定您的 Fboard 账号\n• 查看流量使用情况\n• 获取最新订阅链接\n• 管理账号绑定状态",
            );
            let footer = cfg_string(
                ctx,
                "start_footer",
                "💡 提示：所有命令都需要在私聊中使用",
            );
            let bound = bound_user(ctx, chat_id).await;
            let mut body = format!("{title}\n\n{description}\n\n");
            if let Some((_, email, _, _, _, _)) = bound {
                body.push_str(&format!("✅ 您已绑定账号：{email}\n\n"));
                body.push_str(&cfg_string(
                    ctx,
                    "start_unbind_guide",
                    "📋 可用命令：\n/traffic - 查看流量使用情况\n/getlatesturl - 获取订阅链接\n/unbind - 解绑账号",
                ));
            } else {
                body.push_str(&cfg_string(
                    ctx,
                    "start_bind_guide",
                    "🔗 请先绑定您的 Fboard 账号：\n1. 登录您的 Fboard 账户\n2. 复制您的订阅链接\n3. 发送 /bind + 订阅链接",
                ));
                body.push_str("\n\n");
                body.push_str(&cfg_string(
                    ctx,
                    "start_bind_commands",
                    "📋 可用命令：\n/bind [订阅链接] - 绑定账号",
                ));
            }
            body.push_str("\n\n");
            body.push_str(&footer);
            send_message(ctx, chat_id, &body).await;
            true
        }
        "/bind" => handle_bind(ctx, chat_id, text).await,
        "/traffic" => {
            let Some((_, _, u, d, total, _)) = bound_user(ctx, chat_id).await else {
                send_message(ctx, chat_id, "请先绑定账号").await;
                return true;
            };
            let used = u + d;
            let remaining = (total - used).max(0);
            let percent = if total > 0 {
                used as f64 / total as f64 * 100.0
            } else {
                0.0
            };
            let body = format!(
                "📊 流量使用情况\n\n已用流量：{}G\n总流量：{}G\n剩余流量：{}G\n使用率：{:.2}%",
                format_gb(used),
                format_gb(total),
                format_gb(remaining),
                percent
            );
            send_message(ctx, chat_id, &body).await;
            true
        }
        "/getlatesturl" => {
            let Some((_, _, _, _, _, _)) = bound_user(ctx, chat_id).await else {
                send_message(ctx, chat_id, "请先绑定账号").await;
                return true;
            };
            let token: Option<(String,)> =
                sqlx::query_as("SELECT token FROM v2_user WHERE telegram_id = ? LIMIT 1")
                    .bind(chat_id)
                    .fetch_optional(ctx.db)
                    .await
                    .ok()
                    .flatten();
            let Some((token,)) = token else {
                send_message(ctx, chat_id, "请先绑定账号").await;
                return true;
            };
            let base = if ctx.settings.str("app_url", "").is_empty() {
                ctx.app_url.to_string()
            } else {
                ctx.settings.str("app_url", "")
            };
            let url = format!(
                "{}/{}/{}",
                base.trim_end_matches('/'),
                ctx.settings.subscribe_path(),
                token
            );
            send_message(ctx, chat_id, &format!("🔗 您的订阅链接：\n\n{url}")).await;
            true
        }
        "/unbind" => {
            let Some((user_id, _, _, _, _, _)) = bound_user(ctx, chat_id).await else {
                send_message(ctx, chat_id, "请先绑定账号").await;
                return true;
            };
            let updated = sqlx::query(
                "UPDATE v2_user SET telegram_id = NULL, updated_at = ? WHERE id = ?",
            )
            .bind(util::now())
            .bind(user_id)
            .execute(ctx.db)
            .await
            .map(|r| r.rows_affected() > 0)
            .unwrap_or(false);
            send_message(
                ctx,
                chat_id,
                if updated { "解绑成功" } else { "解绑失败" },
            )
            .await;
            true
        }
        _ => {
            let help = cfg_string(
                ctx,
                "help_text",
                "请使用以下命令：\n/bind - 绑定账号\n/traffic - 查看流量\n/getlatesturl - 获取最新链接",
            );
            send_message(ctx, chat_id, &help).await;
            true
        }
    }
}

#[async_trait]
impl NativePlugin for TelegramPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            code: "telegram".into(),
            name: "Telegram Bot 集成".into(),
            kind: PluginKind::Feature,
            version: "1.0.0".into(),
            description: "Telegram Bot 消息处理和命令系统".into(),
            author: "Fboard".into(),
            builtin: true,
            path: None,
            config_fields: config_fields(),
            readme: "# Telegram\n\n配置 `telegram_bot_token` 后，将 webhook 指向 `/api/v1/guest/telegram/webhook?access_token=md5(bot_token)`。"
                .into(),
        }
    }

    async fn call(
        &self,
        method: &str,
        params: &Value,
        ctx: &PluginContext<'_>,
    ) -> anyhow::Result<Value> {
        match method {
            "telegram.webhook" => {
                let update = params
                    .get("update")
                    .cloned()
                    .unwrap_or_else(|| json!({}));
                if let Some(message) = update.get("message") {
                    let handled = handle_message(ctx, message).await;
                    Ok(json!({"handled": handled}))
                } else {
                    Ok(json!({"handled": false}))
                }
            }
            "telegram.ticket_notify" => {
                if !cfg_bool(ctx, "enable_ticket_notify", true) {
                    return Ok(json!({"sent": false}));
                }
                let ticket_id = params
                    .get("ticket_id")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let row = sqlx::query(
                    "SELECT t.subject, m.message, u.telegram_id FROM v2_ticket t \
                     JOIN v2_ticket_message m ON m.ticket_id = t.id \
                     JOIN v2_user u ON u.id = t.user_id \
                     WHERE t.id = ? ORDER BY m.id DESC LIMIT 1",
                )
                .bind(ticket_id)
                .fetch_optional(ctx.db)
                .await?;
                let Some(row) = row else {
                    return Ok(json!({"sent": false}));
                };
                let chat_id: Option<i64> = row.try_get("telegram_id").ok();
                let subject: String = row.try_get("subject").unwrap_or_default();
                let message: String = row.try_get("message").unwrap_or_default();
                let Some(chat_id) = chat_id else {
                    return Ok(json!({"sent": false}));
                };
                let text = format!("📮 工单提醒 #{ticket_id}\n\n📝 主题: {subject}\n💬 内容: {message}");
                let sent = send_message(ctx, chat_id, &text).await;
                Ok(json!({"sent": sent}))
            }
            "telegram.payment_notify" => {
                if !cfg_bool(ctx, "enable_payment_notify", true) {
                    return Ok(json!({"sent": false}));
                }
                let order_id = params
                    .get("order_id")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let row = sqlx::query(
                    "SELECT o.trade_no, o.total_amount, u.telegram_id FROM v2_order o                      JOIN v2_user u ON u.id = o.user_id WHERE o.id = ? LIMIT 1",
                )
                .bind(order_id)
                .fetch_optional(ctx.db)
                .await?;
                let Some(row) = row else {
                    return Ok(json!({"sent": false}));
                };
                let chat_id: Option<i64> = row.try_get("telegram_id").ok();
                let trade_no: String = row.try_get("trade_no").unwrap_or_default();
                let total: i64 = row.try_get("total_amount").unwrap_or(0);
                let Some(chat_id) = chat_id else {
                    return Ok(json!({"sent": false}));
                };
                let text = format!(
                    "💰 支付成功

订单：{trade_no}
金额：{:.2} 元",
                    total as f64 / 100.0
                );
                let sent = send_message(ctx, chat_id, &text).await;
                Ok(json!({"sent": sent}))
            }
            other => anyhow::bail!("unsupported method: {other}"),
        }
    }
}
