use crate::entities::users::{Column, Entity as UsersEntity};
use crate::utils::app_error::AppError;
use crate::utils::hash::verify_password;
use crate::utils::jwt::create_token;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = String),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn login_handler(
    State(db): State<DatabaseConnection>,
    Json(user_request): Json<LoginRequest>,
) -> Result<String, AppError> {
    let user = UsersEntity::find()
        .filter(Column::Username.eq(user_request.username))
        .one(&db)
        .await
        .map_err(|err| {
            error!("Error finding user: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error finding user")
        })?
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "User not found"))?;

    if !verify_password(&user_request.password, &user.password)? {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "Invalid password"));
    }

    Ok(create_token(user.username.clone())?)
}
