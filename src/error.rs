use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("resource not found")]
    NotFound,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error("invalid request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            Self::Database(_) => {
                tracing::error!(error = %self, "database operation failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "database error".into())
            }
            Self::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}
