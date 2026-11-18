use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::extract::State;
use sqlx::MySqlPool;
use tokio::sync::RwLock;

use crate::cache::MemoryCache;
use crate::config::Config;
use crate::error::ApiError;
use crate::models::UserRow;
use crate::plugin::PluginRegistry;
use crate::settings::Settings;
use crate::util;

pub struct AppState {
    pub db: MySqlPool,
    pub cfg: Config,
    pub settings: RwLock<Settings>,
    pub cache: MemoryCache,
    pub plugin: Arc<PluginRegistry>,
    pub ws: Arc<crate::ws::WsRegistry>,
    pub started_at: i64,
}

impl AppState {
    pub async fn settings(&self) -> Settings {
        self.settings.read().await.clone()
    }

    pub async fn refresh_settings(&self) {
        let loaded = Settings::load(&self.db).await;
        *self.settings.write().await = loaded;
    }

    pub fn secure_path(&self) -> String {
        // Avoid awaiting in handlers that only need the path; cached value is
        // refreshed on settings writes.
        let settings = self.settings.try_read();
        match settings {
            Ok(s) => s.secure_path(&self.cfg.app_key),
            Err(_) => "admin".to_string(),
        }
    }

    pub fn subscribe_path(&self) -> String {
        let settings = self.settings.try_read();
        match settings {
            Ok(s) => s.subscribe_path(),
            Err(_) => "s".to_string(),
        }
    }
}

pub type SharedState = Arc<AppState>;

pub async fn load_user_by_token(db: &MySqlPool, raw_token: &str) -> Option<UserRow> {
    let token = util::bearer_token(Some(raw_token)).unwrap_or_else(|| raw_token.to_string());
    let hash = util::sha256_hex(&token);
    let user = sqlx::query_as::<_, UserRow>(
        r#"SELECT u.* FROM personal_access_tokens t
           JOIN v2_user u ON u.id = t.tokenable_id
           WHERE t.token = ?
             AND (t.tokenable_type = 'App\\Models\\User' OR t.tokenable_type LIKE '%User')
             AND (t.expires_at IS NULL OR t.expires_at > NOW())
           LIMIT 1"#,
    )
    .bind(hash)
    .fetch_optional(db)
    .await
    .ok()
    .flatten()?;

    // last_used_at is informational; failure must not reject a valid session.
    let _ = sqlx::query("UPDATE personal_access_tokens SET last_used_at = NOW() WHERE token = ?")
        .bind(util::sha256_hex(&token))
        .execute(db)
        .await;
    Some(user)
}

pub struct AuthUser(pub UserRow);

impl FromRequestParts<SharedState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok());
        let token = util::bearer_token(header)
            .or_else(|| query_param(parts.uri.query(), "auth_data"))
            .ok_or_else(|| ApiError::new(StatusCode::FORBIDDEN, "未登录或登陆已过期"))?;

        let user = load_user_by_token(&state.db, &token)
            .await
            .ok_or_else(|| ApiError::new(StatusCode::FORBIDDEN, "未登录或登陆已过期"))?;

        if user.banned != 0 {
            return Err(ApiError::forbidden("Your account has been suspended"));
        }
        Ok(AuthUser(user))
    }
}

pub struct AdminUser(pub UserRow);

impl AdminUser {
    pub fn empty() -> Self {
        AdminUser(UserRow::default())
    }
}

impl FromRequestParts<SharedState> for AdminUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let AuthUser(user) = AuthUser::from_request_parts(parts, state).await?;
        if user.is_admin == 0 {
            return Err(ApiError::new(StatusCode::FORBIDDEN, "Unauthorized"));
        }
        Ok(AdminUser(user))
    }
}

pub struct MaybeUser(pub Option<UserRow>);

impl FromRequestParts<SharedState> for MaybeUser {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok());
        let token = util::bearer_token(header).or_else(|| query_param(parts.uri.query(), "token"));
        let user = match token {
            Some(token) => load_user_by_token(&state.db, &token).await,
            None => None,
        };
        Ok(MaybeUser(user))
    }
}

pub fn query_param(query: Option<&str>, key: &str) -> Option<String> {
    let query = query?;
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if k == key {
            return Some(
                percent_encoding::percent_decode_str(v)
                    .decode_utf8_lossy()
                    .to_string(),
            );
        }
    }
    None
}

pub async fn require_client_user(
    state: &SharedState,
    token: Option<&str>,
) -> Result<UserRow, ApiError> {
    let token = util::validate_subscription_token(token)
        .ok_or_else(|| ApiError::forbidden("token is error"))?;
    let user = sqlx::query_as::<_, UserRow>("SELECT * FROM v2_user WHERE token = ? LIMIT 1")
        .bind(&token)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| ApiError::forbidden("token is error"))?;
    if user.banned != 0 {
        return Err(ApiError::forbidden("Your account has been suspended"));
    }
    Ok(user)
}
