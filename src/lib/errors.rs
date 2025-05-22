use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    InternalServerError(Option<String>),
    BadRequest(String),
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
    Forbidden(String),
    Validation(ValidationErrors)
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ValidationErrorResponse {
    pub message: String,
    pub errors: ValidationErrors,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ErrorBody {
    Message { message: String },
    ValidationWithMessage(ValidationErrorResponse),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_body) = match self {
            AppError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorBody::Message {
                    message: msg.unwrap_or_else(|| "Something went wrong".to_string()),
                },
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorBody::Message { message: msg },
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorBody::Message { message: msg },
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                ErrorBody::Message { message: msg },
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                ErrorBody::Message { message: msg },
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                ErrorBody::Message { message: msg },
            ),
            AppError::Validation(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorBody::ValidationWithMessage(ValidationErrorResponse {
                    message: "Validation failed".to_string(),
                    errors,
                }),
            ),
        };

        (status_code, Json(error_body)).into_response()
    }
}
