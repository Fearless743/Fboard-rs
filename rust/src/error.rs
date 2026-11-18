use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};

/// Business/API error. Mirrors the Laravel `ApiException` envelope:
/// `{"status":"fail","message":...,"data":null,"error":null}`.
#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
    pub data: Option<Value>,
}

pub type ApiResult<T = Value> = Result<T, ApiError>;

impl ApiError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            data: None,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    pub fn too_many(message: impl Into<String>) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, message)
    }

    pub fn server(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status.as_u16(), self.message)
    }
}

impl std::error::Error for ApiError {}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!(error = %err, "database error");
        ApiError::server("数据库操作失败")
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!(error = %err, "internal error");
        ApiError::server("服务器内部错误")
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({
            "status": "fail",
            "message": self.message,
            "data": self.data,
            "error": null,
        });
        (self.status, Json(body)).into_response()
    }
}

/// Helper conversions used in handlers.
pub trait ApiContext<T> {
    fn api(self) -> ApiResult<T>;
    fn api_msg(self, message: &str) -> ApiResult<T>;
}

impl<T> ApiContext<T> for Result<T, sqlx::Error> {
    fn api(self) -> ApiResult<T> {
        self.map_err(ApiError::from)
    }
    fn api_msg(self, message: &str) -> ApiResult<T> {
        self.map_err(|err| {
            tracing::error!(error = %err, "database error");
            ApiError::server(message)
        })
    }
}
