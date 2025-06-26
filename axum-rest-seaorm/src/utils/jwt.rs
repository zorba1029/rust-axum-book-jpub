use super::app_error;
use axum::{
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{debug, error};


#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    username: String,
}

use lazy_static::lazy_static;

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}

pub fn create_token(username: String) -> Result<String, app_error::AppError> {
    let now = chrono::Utc::now();
    let expires_at = now + Duration::hours(1);
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(JWT_SECRET.as_bytes());

    encode(&token_header, &claims, &key).map_err(|err| {
        error!("Error encoding JWT: {:?}", err);
        app_error::AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error encoding JWT")
    })
}

pub fn validate_token(token: &str) -> Result<Claims, app_error::AppError> {
    let binding = token.replace("Bearer ", "");
    let key = DecodingKey::from_secret(JWT_SECRET.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    decode::<Claims>(&binding, &key, &validation).map_err(|err| match err.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidToken
        | jsonwebtoken::errors::ErrorKind::InvalidSignature
        | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            error!("Error validating JWT: {:?}", err);
            app_error::AppError::new(StatusCode::UNAUTHORIZED, "Not authorized")
        }
        _ => {
            error!("Error validating JWT: {:?}", err);
            app_error::AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating JWT")
        }
    })
    .and_then(|decoded| {
        if chrono::Utc::now().timestamp() > decoded.claims.exp as i64 {
            Err(app_error::AppError::new(StatusCode::UNAUTHORIZED, "Token expired"))
        } else {
            Ok(decoded.claims)
        }
    })
}

pub async fn authenticate(
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Result<Response, app_error::AppError> {
    if let Some(value) = headers.get("Authorization") {
        let token = value.to_str().map_err(|err| {
            error!("Error getting Authorization header: {:?}", err);
            app_error::AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
        })?;

        let claims = validate_token(token)?;

        debug!("Authenticated user: {}", claims.username);

        if claims.exp < (chrono::Utc::now().timestamp() as usize) {
            return Err(app_error::AppError::new(StatusCode::UNAUTHORIZED, "Token expired"));
        }

        Ok(next.run(request).await)
    } else {
        Err(app_error::AppError::new(StatusCode::UNAUTHORIZED, "Not authenticated"))
    }
}