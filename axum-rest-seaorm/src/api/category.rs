use std::collections::HashMap;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter,
};

use crate::entities::category::{ActiveModel, Column, Entity, Model};
use crate::utils::app_error::AppError;
use utoipa::ToSchema;

// Wrapper functions for OpenAPI documentation
#[utoipa::path(
    get,
    path = "/categories",
    params(
        ("name" = Option<String>, Query, description = "Category name to search")
    ),
    responses(
        (status = 200, description = "List of categories", body = Vec<Model>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Categories"
)]
pub async fn get_category_handler(
    Query(params): Query<HashMap<String, String>>,
    State(conn): State<DatabaseConnection>,
) -> Result<Json<Vec<Model>>, AppError> {
    get_category(Query(params), State(conn)).await
}

// SELECT
pub async fn get_category(
    Query(params): Query<HashMap<String, String>>,
    State(conn): State<DatabaseConnection>,
) -> Result<Json<Vec<Model>>, AppError> {
    let mut condition = Condition::all();

    if let Some(name) = params.get("name") {
        condition = condition.add(Column::Name.contains(name));
    }

    match Entity::find().filter(condition).all(&conn).await {
        Ok(categories) => Ok(Json(categories)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    }
}

#[derive(serde::Deserialize, ToSchema)]
pub struct UpsertModel {
    name: Option<String>,
}

#[utoipa::path(
    post,
    path = "/categories",
    request_body = inline(UpsertModel),
    responses(
        (status = 200, description = "Category created", body = Model),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Categories"
)]
pub async fn post_category_handler(
    State(conn): State<DatabaseConnection>,
    Json(category): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    post_category(State(conn), Json(category)).await
}

// INSERT
// #[axum::debug_handler]
pub async fn post_category(
    State(conn): State<DatabaseConnection>,
    Json(category): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    let new_category = ActiveModel {
        name: ActiveValue::Set(category.name.unwrap()),
    };

    match new_category.insert(&conn).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    }
}

#[utoipa::path(
    delete,
    path = "/categories",
    params(
        ("name" = String, Query, description = "Category name to delete")
    ),
    responses(
        (status = 200, description = "Category deleted", body = String),
        (status = 400, description = "Name not provided", body = ErrorResponse),
        (status = 404, description = "Category not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Categories"
)]
pub async fn delete_category_handler(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<&'static str>, AppError> {
    delete_category(State(conn), Query(params)).await
}

// DELETE
pub async fn delete_category(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<&'static str>, AppError> {
    if params.get("name").is_none() {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Name not provided"
        ));
    }

    let category = match Entity::find()
        .filter(Condition::any().add(Column::Name.contains(params.get("name").unwrap())))
        .one(&conn)
        .await {
            Ok(Some(category)) => category,
            Ok(None) => return Err(AppError::new(
                StatusCode::NOT_FOUND,
                "Category not found"
            )),
            Err(_) => return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error"
            )),
        };

    match category.delete(&conn).await {
        Ok(_) => Ok(Json("Category deleted")),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    }
}
