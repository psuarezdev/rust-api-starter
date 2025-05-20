use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    InternalServerError(Option<String>),
    BadRequest(String),
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
    Forbidden(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            AppError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg.unwrap_or_else(|| "Something went wrong".to_string()),
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg,
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                msg,
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                msg,
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                msg,
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                msg,
            ),
        };

        let error_body = Json(ErrorResponse { message });
        (status_code, error_body).into_response()
    }
}