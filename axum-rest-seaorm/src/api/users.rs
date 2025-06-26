use std::collections::HashMap;

use axum::{extract::{Query, State}, http::StatusCode, Json};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, 
    QueryFilter, QueryOrder, Order,
    ActiveModelTrait, ActiveValue, ModelTrait
};
use crate::entities::users::{ActiveModel, Column, Entity, Model};
use crate::utils::app_error::AppError;
use crate::utils::hash::hash_password;
use utoipa::ToSchema;

// Wrapper functions for OpenAPI documentation
#[utoipa::path(
    get,
    path = "/user",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Option<String>, Query, description = "User ID"),
        ("username" = Option<String>, Query, description = "Username to search")
    ),
    responses(
        (status = 200, description = "User found", body = Model),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 400, description = "Invalid parameters", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn get_user_handler(
    Query(params): Query<QueryParams>,
    State(conn): State<DatabaseConnection>,
) -> Result<Json<Model>, AppError> {
    get_user(Query(params), State(conn)).await
}

pub async fn get_user(
    Query(params): Query<QueryParams>,
    State(conn): State<DatabaseConnection>,
) -> Result<Json<Model>, AppError> {
    let mut condition = Condition::any();

    if let Some(id) = &params.id {
        match id.parse::<i32>() {
            Ok(parsed_id) => condition = condition.add(Column::Id.eq(parsed_id)),
            Err(_) => {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    "ID must be an integer",
                ));
            }
        }
    }
    if let Some(username) = &params.username {
        condition = condition.add(Column::Username.contains(username));
    }
    println!("condition: {:?}", condition);
    println!("id: {:?}", &params.id);
    println!("username: {:?}", &params.username);
    
    // let user = Entity::find()
    //     .filter(condition)
    //     .one(&conn)
    //     .await
    //     .unwrap()
    //     .unwrap();
    // Json(user)

    match Entity::find()
        .filter(condition)
        .one(&conn)
        .await
    {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(AppError::new(
            StatusCode::NOT_FOUND, 
            "User not found"
        )),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Database error"
        )),
    }
}

#[utoipa::path(
    get,
    path = "/users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Option<String>, Query, description = "User ID"),
        ("username" = Option<String>, Query, description = "Username to search")
    ),
    responses(
        (status = 200, description = "List of users", body = Vec<Model>),
        (status = 400, description = "Invalid parameters", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn get_users_handler(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Model>>, AppError> {
    get_users(State(conn), Query(params)).await
}

pub async fn get_users(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Model>>, AppError> {
    let mut condition = Condition::all();

    if let Some(id) = params.get("id") {
        match id.parse::<i32>() {
            Ok(parsed_id) => condition = condition.add(Column::Id.eq(parsed_id)),
            Err(_) => {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST, 
                    "ID must be an integer"
                ));
            }
        }
    }

    if let Some(username) = params.get("username") {
        condition = condition.add(Column::Username.contains(username));
    }
    // println!("condition: {:?}", condition);
    // println!("id: {:?}", params.get("id"));
    // println!("username: {:?}", params.get("username"));
    
    match Entity::find()
        .filter(condition)
        .order_by(Column::Username, Order::Asc)
        .all(&conn)
        .await
    {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR, 
            "Database error"
        )),
    }
}

#[derive(serde::Deserialize, ToSchema)]
pub struct QueryParams {
    #[schema(example = 1)]
    pub id: Option<String>,
    #[schema(example = "john")]
    pub username: Option<String>,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct DeleteParams {
    #[schema(example = "1")]
    pub id: String,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct UpsertModel {
    #[schema(example = 1)]
    pub id: Option<i32>,
    #[schema(example = "john_doe")]
    pub username: Option<String>,
    #[schema(example = "secure_password")]
    pub password: Option<String>,
}

#[utoipa::path(
    post,
    path = "/auth/signup",
    security(
        ("bearer_auth" = [])
    ),
    request_body = inline(UpsertModel),
    responses(
        (status = 200, description = "User created", body = Model),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn post_user_handler(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    create_user(State(conn), Json(user)).await
}

pub async fn create_user(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    if user.username.is_none() || user.password.is_none() {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Username or password not provided"
        ));
    }

    let hashed_password = hash_password(&user.password.unwrap())?;

    let new_user = ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(user.username.unwrap()),
        password: ActiveValue::Set(hashed_password),
    };

    match new_user.insert(&conn).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    }
}

#[utoipa::path(
    put,
    path = "/users",
    security(
        ("bearer_auth" = [])
    ),
    request_body = inline(UpsertModel),
    responses(
        (status = 200, description = "User updated", body = Model),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn put_user_handler(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    put_user(State(conn), Json(user)).await
}

pub async fn put_user(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    let id = match user.id {
        Some(id) => id,
        None => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "ID not provided"
            ));
        }
    };

    let found_user = match Entity::find_by_id(id).one(&conn).await {
        Ok(user) => user.ok_or(AppError::new(
            StatusCode::NOT_FOUND,
            "User not found"
        ))?,
        Err(_) => return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    };

    let mut active_user: ActiveModel = found_user.into();

    active_user.username = user.username.map(ActiveValue::Set).unwrap_or(active_user.username);
    active_user.password = user.password.map(ActiveValue::Set).unwrap_or(active_user.password);

    match active_user.update(&conn).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    }
}

#[utoipa::path(
    delete,
    path = "/users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = String, Query, description = "User ID to delete")
    ),
    responses(
        (status = 200, description = "User deleted", body = String),
        (status = 400, description = "Invalid parameters", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn delete_user_handler(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<DeleteParams>,
) -> Result<Json<&'static str>, AppError> {
    delete_user(State(conn), Query(params)).await
}

pub async fn delete_user(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<DeleteParams>,
) -> Result<Json<&'static str>, AppError> {
    //-->> TimeoutLayer testing
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    //<<--- TimeoutLayer testing

    let user_id = params.id.parse::<i32>()
        .map_err(|_| AppError::new(
            StatusCode::BAD_REQUEST,
            "User ID must be an integer"
        ))?;

    // user_id가 존재하는지 확인
    let user_to_delete = Entity::find_by_id(user_id)
        .one(&conn)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "User not found"))?;

    match user_to_delete.delete(&conn).await {
        Ok(_) => {
            println!("User deleted: {}", user_id);
            Ok(Json("User deleted"))
        },
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    }
}

// -- TimeoutLayer Testing --
// * Preparing request to http://localhost:8000/users?id=12
// * Current time is 2025-06-26T03:51:25.572Z
// * Enable automatic URL encoding
// * Using default HTTP version
// * Enable timeout of 30000ms
// * Enable SSL validation
// * Found bundle for host: 0x11405114280 [serially]
// * Can not multiplex, even if we wanted to
// * Re-using existing connection #1 with host localhost
// * Connected to localhost (127.0.0.1) port 8000 (#1)

// > DELETE /users?id=12 HTTP/1.1
// > Host: localhost:8000
// > User-Agent: insomnia/11.2.0
// > Accept: */*

// * Mark bundle as not supporting multiuse

// < HTTP/1.1 408 Request Timeout
// < content-length: 0
// < date: Thu, 26 Jun 2025 03:51:28 GMT

// * Connection #1 to host localhost left intact