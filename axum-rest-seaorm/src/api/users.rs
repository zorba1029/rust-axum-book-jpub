use std::collections::HashMap;

use axum::{extract::{Query, State}, http::StatusCode, Json};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, 
    QueryFilter, QueryOrder, Order,
    ActiveModelTrait, ActiveValue, ModelTrait
};
use crate::entities::users::{ActiveModel, Column, Entity, Model};
use crate::utils::app_error::AppError;
use utoipa::ToSchema;

// Wrapper functions for OpenAPI documentation
#[utoipa::path(
    get,
    path = "/user",
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
    id: Option<i32>,
    #[schema(example = "john_doe")]
    username: Option<String>,
    #[schema(example = "secure_password")]
    password: Option<String>,
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = inline(UpsertModel),
    responses(
        (status = 200, description = "User created", body = Model),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn post_user_handler(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    post_user(State(conn), Json(user)).await
}

pub async fn post_user(
    State(conn): State<DatabaseConnection>,
    Json(user): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    if user.username.is_none() || user.password.is_none() {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Username or password not provided"
        ));
    }

    let new_user = ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(user.username.unwrap()),
        password: ActiveValue::Set(user.password.unwrap()),
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