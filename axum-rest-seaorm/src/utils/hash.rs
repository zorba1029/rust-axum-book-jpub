use super::app_error;
use axum::http::StatusCode;
use bcrypt::{hash, verify};
use tracing::error;

const COST: u32 = 12;

pub fn hash_password(password: &str) -> Result<String, app_error::AppError> {
    hash(password, COST).map_err(|err| {
        error!("Error hashing password: {:?}", err);
        app_error::AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password")
    })
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, app_error::AppError> {
    verify(password, hash).map_err(|err| {
        error!("Error verifying password: {:?}", err);
        app_error::AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error verifying password")
    })
}
