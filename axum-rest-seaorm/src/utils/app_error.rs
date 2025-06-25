use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};

pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, Json(self.message.clone())).into_response()
    }
}