use std::collections::HashMap;

use axum::extract::{ConnectInfo, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Json;
use serde_json::{json, Value};

use crate::cache::keys;
use crate::error::{ApiError, ApiResult};
use crate::mail;
use crate::models::UserRow;
use crate::response::{ok, str_field};
use crate::state::{SharedState, load_user_by_token};
use crate::util;

fn client_ip(headers: &HeaderMap, connect: Option<&std::net::SocketAddr>) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|v| v.trim().to_string())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.trim().to_string())
        })
        .or_else(|| connect.map(|c| c.ip().to_string()))
        .unwrap_or_else(|| "127.0.0.1".into())
}

fn user_agent(headers: &HeaderMap) -> String {
    headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string()
}

fn valid_email(email: &str) -> bool {
    static RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
        regex::Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap()
    });
    RE.is_match(email)
}

pub async fn register(
    State(state): State<SharedState>,
    ConnectInfo(connect): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let email = str_field(&body, "email")
        .map(|v| v.trim().to_lowercase())
        .unwrap_or_default();
    let password = str_field(&body, "password").unwrap_or_default();
    if !valid_email(&email) {
        return Err(ApiError::bad_request("邮箱格式不正确"));
    }
    if password.len() < 8 {
        return Err(ApiError::bad_request("密码必须大于 8 位"));
    }

    let settings = state.settings().await;
    let ip = client_ip(&headers, Some(&connect));

    if settings.bool("stop_register", false) {
        return Err(ApiError::bad_request("注册已关闭"));
    }

    if settings.bool("register_limit_by_ip_enable", false) {
        let key = keys::register_ip_rate(&ip);
        let count: i64 = state
            .cache
            .get(&key)
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        if count >= settings.i64("register_limit_count", 3) {
            return Err(ApiError::too_many("注册过于频繁，请稍后再试"));
        }
    }

    if settings.bool("email_whitelist_enable", false) {
        let suffix = email.rsplit('@').next().unwrap_or("");
        let allowed = settings.arr("email_whitelist_suffix");
        let allowed: Vec<String> = allowed
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        if !allowed.is_empty() && !allowed.iter().any(|s| s == suffix) {
            return Err(ApiError::bad_request("邮箱后缀不在白名单内"));
        }
    }

    if settings.bool("email_gmail_limit_enable", false) && email.ends_with("@gmail.com") {
        let prefix = email.split('@').next().unwrap_or("");
        if prefix.contains('.') || prefix.contains('+') {
            return Err(ApiError::bad_request("不支持 Gmail 别名邮箱"));
        }
    }

    let invite_code = str_field(&body, "invite_code").unwrap_or_default();
    if settings.bool("invite_force", false) && invite_code.is_empty() {
        return Err(ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, "必须使用邀请码注册"));
    }

    let email_code = str_field(&body, "email_code").unwrap_or_default();
    if settings.bool("email_verify", false) {
        if email_code.len() != 6 || !email_code.chars().all(|c| c.is_ascii_digit()) {
            return Err(ApiError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "邮箱验证码不能为空",
            ));
        }
        let cached = state.cache.get(&keys::email_verify_code(&email));
        if cached.as_deref() != Some(email_code.as_str()) {
            return Err(ApiError::bad_request("邮箱验证码错误"));
        }
    }

    let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM v2_user WHERE email = ? LIMIT 1")
        .bind(&email)
        .fetch_optional(&state.db)
        .await?;
    if existing.is_some() {
        return Err(ApiError::bad_request("邮箱已存在"));
    }

    // Invite code (optional unless forced).
    let invite_user_id = if !invite_code.is_empty() {
        let row: Option<(i64, i64, i64)> = sqlx::query_as(
            "SELECT id, user_id, status FROM v2_invite_code WHERE code = ? LIMIT 1",
        )
        .bind(&invite_code)
        .fetch_optional(&state.db)
        .await?;
        match row {
            Some((id, user_id, status)) if status == 0 => {
                if !settings.bool("invite_never_expire", false) {
                    sqlx::query("UPDATE v2_invite_code SET status = 1, updated_at = ? WHERE id = ?")
                        .bind(util::now())
                        .bind(id)
                        .execute(&state.db)
                        .await?;
                }
                Some(user_id)
            }
            _ => {
                if settings.bool("invite_force", false) {
                    return Err(ApiError::bad_request("邀请码无效"));
                }
                None
            }
        }
    } else {
        None
    };

    let now = util::now();
    let password_hash = util::hash_password(&password);
    let uuid = util::uuid_v4();
    let token = util::guid();
    let remind_expire = settings.i64("default_remind_expire", 1);
    let remind_traffic = settings.i64("default_remind_traffic", 1);

    let result = sqlx::query(
        r#"INSERT INTO v2_user
           (invite_user_id, email, password, uuid, token, remind_expire, remind_traffic,
            balance, commission_balance, t, u, d, transfer_enable, banned, is_admin,
            created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, 0, 0, 0, 0, 0, 0, 0, 0, ?, ?)"#,
    )
    .bind(invite_user_id)
    .bind(&email)
    .bind(&password_hash)
    .bind(&uuid)
    .bind(&token)
    .bind(remind_expire)
    .bind(remind_traffic)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;
    let user_id = result.last_insert_id() as i64;

    apply_tryout_plan(&state, user_id, &settings).await?;
    crate::traffic::refresh_next_reset(&state.db, user_id).await.ok();

    record_login(&state, user_id, &ip, &user_agent(&headers), "register").await;

    if settings.bool("email_verify", false) {
        state.cache.forget(&keys::email_verify_code(&email));
    }
    if settings.bool("register_limit_by_ip_enable", false) {
        let ttl = settings.i64("register_limit_expire", 60) * 60;
        state.cache.increment(&keys::register_ip_rate(&ip), ttl.max(60));
    }

    let user = fetch_user(&state, user_id).await?;
    let auth = issue_token(&state, &user).await?;
    Ok(ok(auth))
}

async fn apply_tryout_plan(
    state: &SharedState,
    user_id: i64,
    settings: &crate::settings::Settings,
) -> ApiResult<()> {
    let plan_id = settings.i64("try_out_plan_id", 0);
    if plan_id <= 0 {
        return Ok(());
    }
    let plan: Option<crate::models::PlanRow> =
        sqlx::query_as("SELECT * FROM v2_plan WHERE id = ?")
            .bind(plan_id)
            .fetch_optional(&state.db)
            .await?;
    let Some(plan) = plan else { return Ok(()) };
    let days = settings.i64("try_out_hour", 1).max(1);
    let expired = util::now() + days * 3600;
    sqlx::query(
        "UPDATE v2_user SET plan_id = ?, group_id = ?, transfer_enable = ?, speed_limit = ?, device_limit = ?, expired_at = ?, updated_at = ? WHERE id = ?",
    )
    .bind(plan.id)
    .bind(plan.group_id)
    .bind((plan.transfer_enable.unwrap_or(0) as i64).saturating_mul(1_073_741_824))
    .bind(plan.speed_limit)
    .bind(plan.device_limit)
    .bind(expired)
    .bind(util::now())
    .bind(user_id)
    .execute(&state.db)
    .await?;
    Ok(())
}

async fn fetch_user(state: &SharedState, id: i64) -> ApiResult<UserRow> {
    sqlx::query_as::<_, UserRow>("SELECT * FROM v2_user WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| ApiError::not_found("用户不存在"))
}

pub async fn issue_token(state: &SharedState, user: &UserRow) -> ApiResult<Value> {
    let plain = util::random_chars(40);
    let hash = util::sha256_hex(&plain);
    sqlx::query(
        r#"INSERT INTO personal_access_tokens
           (tokenable_type, tokenable_id, name, token, abilities, expires_at, created_at, updated_at)
           VALUES ('App\\Models\\User', ?, ?, ?, '["*"]', DATE_ADD(NOW(), INTERVAL 1 YEAR), NOW(), NOW())"#,
    )
    .bind(user.id)
    .bind(util::random_chars(20))
    .bind(hash)
    .execute(&state.db)
    .await?;

    Ok(json!({
        "token": user.token,
        "auth_data": format!("Bearer {plain}"),
        "is_admin": user.is_admin,
    }))
}

pub async fn login(
    State(state): State<SharedState>,
    ConnectInfo(connect): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let email = str_field(&body, "email")
        .map(|v| v.trim().to_string())
        .unwrap_or_default();
    let password = str_field(&body, "password").unwrap_or_default();
    if !valid_email(&email) {
        return Err(ApiError::bad_request("邮箱格式不正确"));
    }
    if password.len() < 8 {
        return Err(ApiError::bad_request("密码必须大于 8 位"));
    }

    let settings = state.settings().await;
    let ip = client_ip(&headers, Some(&connect));
    let limit_enabled = settings.bool("password_limit_enable", true);
    let limit_count = settings.i64("password_limit_count", 5);
    let limit_expire = settings.i64("password_limit_expire", 60);

    let email_key = keys::password_error(&email);
    let ip_key = keys::password_error_ip(&ip);
    if limit_enabled {
        let email_failures: i64 = state
            .cache
            .get(&email_key)
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        let ip_failures: i64 = state
            .cache
            .get(&ip_key)
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        if email_failures >= limit_count || ip_failures >= limit_count {
            return Err(ApiError::too_many(format!(
                "密码错误次数过多，请在 {limit_expire} 分钟后重试"
            )));
        }
    }

    let user: Option<UserRow> =
        sqlx::query_as("SELECT * FROM v2_user WHERE email = ? LIMIT 1")
            .bind(&email)
            .fetch_optional(&state.db)
            .await?;

    let Some(user) = user else {
        if limit_enabled {
            record_password_failure(&state, &email_key, &ip_key, limit_expire);
        }
        return Err(ApiError::bad_request("邮箱或密码错误"));
    };

    if !util::verify_password(
        user.password_algo.as_deref(),
        user.password_salt.as_deref(),
        &password,
        &user.password,
    ) {
        if limit_enabled {
            record_password_failure(&state, &email_key, &ip_key, limit_expire);
        }
        return Err(ApiError::bad_request("邮箱或密码错误"));
    }

    if user.banned != 0 {
        return Err(ApiError::bad_request("您的账号已被封禁"));
    }

    state.cache.forget(&ip_key);

    sqlx::query("UPDATE v2_user SET last_login_at = ?, last_login_ip = ?, updated_at = ? WHERE id = ?")
        .bind(util::now())
        .bind(&ip)
        .bind(util::now())
        .bind(user.id)
        .execute(&state.db)
        .await?;
    record_login(
        &state,
        user.id,
        &ip,
        &user_agent(&headers),
        "password",
    )
    .await;

    let user = fetch_user(&state, user.id).await?;
    let auth = issue_token(&state, &user).await?;
    Ok(ok(auth))
}

fn record_password_failure(
    state: &SharedState,
    email_key: &str,
    ip_key: &str,
    limit_expire: i64,
) {
    let ttl = 60 * limit_expire.max(1);
    state.cache.increment(email_key, ttl);
    state.cache.increment(ip_key, ttl);
}

pub async fn forget(
    State(state): State<SharedState>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let email = str_field(&body, "email")
        .map(|v| v.trim().to_string())
        .unwrap_or_default();
    let code = str_field(&body, "email_code").unwrap_or_default();
    let password = str_field(&body, "password").unwrap_or_default();
    if password.len() < 8 {
        return Err(ApiError::bad_request("密码必须大于 8 位"));
    }

    let limit_key = keys::forget_request_limit(&email);
    let failures: i64 = state
        .cache
        .get(&limit_key)
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    if failures >= 3 {
        return Err(ApiError::too_many("重置失败，请稍后再试"));
    }

    let user: Option<UserRow> =
        sqlx::query_as("SELECT * FROM v2_user WHERE email = ? LIMIT 1")
            .bind(&email)
            .fetch_optional(&state.db)
            .await?;
    let Some(user) = user else {
        return Err(ApiError::bad_request("该邮箱尚未注册"));
    };
    // Strict byte comparison guards against IDN/collation variants.
    if user.email != email {
        return Err(ApiError::bad_request("邮箱验证码错误"));
    }

    let cached = state.cache.get(&keys::email_verify_code(&user.email));
    if cached.as_deref() != Some(code.as_str()) {
        state.cache.increment(&limit_key, 300);
        return Err(ApiError::bad_request("邮箱验证码错误"));
    }

    let hash = util::hash_password(&password);
    sqlx::query("UPDATE v2_user SET password = ?, password_algo = NULL, password_salt = NULL, updated_at = ? WHERE id = ?")
        .bind(hash)
        .bind(util::now())
        .bind(user.id)
        .execute(&state.db)
        .await?;
    sqlx::query("DELETE FROM personal_access_tokens WHERE tokenable_id = ? AND tokenable_type LIKE '%User'")
        .bind(user.id)
        .execute(&state.db)
        .await?;
    state.cache.forget(&keys::email_verify_code(&email));
    Ok(ok(json!(true)))
}

pub async fn send_email_verify(
    State(state): State<SharedState>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let email = str_field(&body, "email")
        .map(|v| v.trim().to_string())
        .unwrap_or_default();
    if !valid_email(&email) {
        return Err(ApiError::bad_request("邮箱格式不正确"));
    }
    let settings = state.settings().await;

    let existing: Option<(String,)> =
        sqlx::query_as("SELECT email FROM v2_user WHERE email = ? LIMIT 1")
            .bind(&email)
            .fetch_optional(&state.db)
            .await?;
    let recipient = existing.map(|(e,)| e).unwrap_or_else(|| email.clone());

    let cooldown = settings.i64("email_verify_cooldown", 60).max(10);
    let cooldown_key = format!("LAST_SEND_EMAIL_VERIFY_TIMESTAMP_{email}");
    if state.cache.get(&cooldown_key).is_some() {
        return Err(ApiError::bad_request("验证码已发送，请稍后再试"));
    }

    let code = format!("{:06}", rand::random::<u32>() % 1_000_000);
    let app_name = settings.str("app_name", "Fboard");
    let base = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let subject = format!("{app_name} 邮箱验证码");
    let mut vars = HashMap::new();
    vars.insert("name".into(), app_name);
    vars.insert("code".into(), code.clone());
    vars.insert("url".into(), base);

    let mut delivered = true;
    if let Err(err) =
        mail::send_template(&state.db, &settings, &recipient, "verify", &subject, &vars).await
    {
        tracing::warn!(error = %err, email = %recipient, "failed to send verification email");
        delivered = false;
    }

    state.cache.put(keys::email_verify_code(&recipient), code, 300);
    state.cache.put(cooldown_key, util::now().to_string(), cooldown);

    if !delivered
        && settings.str("email_host", "").is_empty()
        && settings.str("mail_host", "").is_empty()
    {
        return Err(ApiError::server("邮件服务未配置，请联系管理员"));
    }
    Ok(ok(json!(true)))
}

pub async fn pv(
    State(state): State<SharedState>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    if let Some(code) = str_field(&body, "invite_code") {
        if !code.is_empty() {
            sqlx::query("UPDATE v2_invite_code SET pv = pv + 1, updated_at = ? WHERE code = ?")
                .bind(util::now())
                .bind(code)
                .execute(&state.db)
                .await?;
        }
    }
    Ok(ok(json!(true)))
}

pub async fn quick_login_url(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let authorization = str_field(&body, "auth_data")
        .or_else(|| {
            headers
                .get(axum::http::header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        })
        .ok_or_else(|| ApiError::unauthorized("授权失败，请先登录"))?;
    let user = load_user_by_token(&state.db, &authorization)
        .await
        .ok_or_else(|| ApiError::unauthorized("账号信息已过期，请重新登录"))?;
    let code = util::guid();
    state
        .cache
        .put(format!("TEMP_TOKEN_{code}"), user.id.to_string(), 60);
    let redirect = util::sanitize_redirect(str_field(&body, "redirect").as_deref());
    let settings = state.settings().await;
    let base = if settings.str("app_url", "").is_empty() {
        state.cfg.app_url.clone()
    } else {
        settings.str("app_url", "")
    };
    let url = format!(
        "{}/#/login?verify={}&redirect={}",
        base.trim_end_matches('/'),
        code,
        urlencoding::encode(&redirect)
    );
    Ok(ok(json!(url)))
}

pub async fn mail_link(
    State(state): State<SharedState>,
    Json(body): Json<Value>,
) -> ApiResult<Response> {
    let email = str_field(&body, "email").unwrap_or_default();
    let settings = state.settings().await;
    if !settings.bool("login_with_mail_link_enable", false) {
        return Err(ApiError::not_found("not found"));
    }
    let cooldown = settings.i64("email_verify_cooldown", 60).max(10);
    let cooldown_key = format!("LAST_SEND_LOGIN_WITH_MAIL_LINK_TIMESTAMP_{email}");
    if state.cache.get(&cooldown_key).is_some() {
        return Err(ApiError::too_many("发送过于频繁，请稍后再试"));
    }

    let user: Option<(i64, String)> =
        sqlx::query_as("SELECT id, email FROM v2_user WHERE email = ? LIMIT 1")
            .bind(&email)
            .fetch_optional(&state.db)
            .await?;
    if let Some((user_id, recipient)) = user {
        let code = util::guid();
        state
            .cache
            .put(format!("TEMP_TOKEN_{code}"), user_id.to_string(), 300);
        state
            .cache
            .put(cooldown_key, util::now().to_string(), cooldown);
        let redirect = util::sanitize_redirect(str_field(&body, "redirect").as_deref());
        let base = if settings.str("app_url", "").is_empty() {
            state.cfg.app_url.clone()
        } else {
            settings.str("app_url", "")
        };
        let link = format!(
            "{}/#/login?verify={}&redirect={}",
            base.trim_end_matches('/'),
            code,
            urlencoding::encode(&redirect)
        );
        let app_name = settings.str("app_name", "Fboard");
        let subject = format!("登录 {app_name}");
        let mut vars = HashMap::new();
        vars.insert("name".into(), app_name);
        vars.insert("link".into(), link);
        vars.insert("url".into(), base);
        if let Err(err) = mail::send_template(
            &state.db,
            &settings,
            &recipient,
            "mailLogin",
            &subject,
            &vars,
        )
        .await
        {
            tracing::warn!(error = %err, "failed to send mail login link");
        }
    }
    // Always report success to avoid account enumeration.
    Ok(ok(json!(true)))
}

pub async fn token2login(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    if let Some(token) = params.get("token") {
        let redirect = util::sanitize_redirect(params.get("redirect").map(|s| s.as_str()));
        let settings = state.settings().await;
        let base = if settings.str("app_url", "").is_empty() {
            state.cfg.app_url.clone()
        } else {
            settings.str("app_url", "")
        };
        return Ok(Redirect::to(&format!(
            "{}/#/login?verify={}&redirect={}",
            base.trim_end_matches('/'),
            token,
            urlencoding::encode(&redirect)
        ))
        .into_response());
    }

    let verify = params
        .get("verify")
        .ok_or_else(|| ApiError::bad_request("无效请求"))?;
    let user_id: Option<i64> = state
        .cache
        .get(&format!("TEMP_TOKEN_{verify}"))
        .and_then(|v| v.parse().ok());
    let Some(user_id) = user_id else {
        return Err(ApiError::bad_request("Token 错误"));
    };
    let user = fetch_user(&state, user_id).await?;
    state.cache.forget(&format!("TEMP_TOKEN_{verify}"));
    let ip = client_ip(&headers, None);
    record_login(&state, user.id, &ip, &user_agent(&headers), "mail_link").await;
    let auth = issue_token(&state, &user).await?;
    Ok(ok(auth))
}

pub async fn record_login(
    state: &SharedState,
    user_id: i64,
    ip: &str,
    ua: &str,
    method: &str,
) {
    let now = util::now();
    let _ = sqlx::query(
        "INSERT INTO v2_user_login_log (user_id, ip, user_agent, method, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(ip)
    .bind(truncate(ua, 512))
    .bind(method)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await;

    // Keep at most 20 rows per user (UserLoginLog::MAX_PER_USER).
    let _ = sqlx::query(
        "DELETE FROM v2_user_login_log WHERE user_id = ? AND id NOT IN (SELECT id FROM (SELECT id FROM v2_user_login_log WHERE user_id = ? ORDER BY id DESC LIMIT 20) t)",
    )
    .bind(user_id)
    .bind(user_id)
    .execute(&state.db)
    .await;
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        s.chars().take(max).collect()
    }
}
