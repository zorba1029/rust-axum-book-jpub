use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter,
};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    entities::product::{ActiveModel, Column, Entity, Model},
    utils::app_error::AppError,
};

#[derive(Deserialize, ToSchema)]
pub struct UpsertModel {
    #[schema(example = 1)]
    id: Option<i32>,
    #[schema(example = "Laptop")]
    title: Option<String>,
    #[schema(example = 1200)]
    price: Option<i32>,
    #[schema(example = "Electronics")]
    category: Option<String>,
}

// Wrapper functions for OpenAPI documentation
#[utoipa::path(
    get,
    path = "/product",
    params(
        ("id" = Option<i32>, Query, description = "Product ID"),
        ("title" = Option<String>, Query, description = "Product title to search"),
        ("price" = Option<i32>, Query, description = "Product price"),
        ("category" = Option<String>, Query, description = "Product category")
    ),
    responses(
        (status = 200, description = "List of products", body = Vec<Model>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn get_product_handler(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<UpsertModel>,
) -> Result<Json<Vec<Model>>, AppError> {
    get_product(State(conn), Query(params)).await
}

pub async fn get_product(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<UpsertModel>,
) -> Result<Json<Vec<Model>>, AppError> {
    let mut condition = Condition::all();

    if let Some(id) = params.id {
        condition = condition.add(Column::Id.eq(id));
    }

    if let Some(title) = params.title {
        condition = condition.add(Column::Title.contains(title));
    }

    if let Some(price) = params.price {
        condition = condition.add(Column::Price.eq(price));
    }

    if let Some(category) = params.category {
        condition = condition.add(Column::Category.contains(category));
    }
    
    match Entity::find().filter(condition).all(&conn).await {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    }
}

#[utoipa::path(
    post,
    path = "/product",
    request_body = UpsertModel,
    responses(
        (status = 200, description = "Product created", body = Model),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn post_product_handler(
    State(conn): State<DatabaseConnection>,
    Json(product): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    post_product(State(conn), Json(product)).await
}

// INSERT
pub async fn post_product(
    State(conn): State<DatabaseConnection>,
    Json(product): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    let new_product = ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(product.title.unwrap()),
        price: ActiveValue::Set(product.price.unwrap()),
        category: ActiveValue::Set(product.category.unwrap()),
    };

    match new_product.insert(&conn).await {
        Ok(inserted_product) => Ok(Json(inserted_product)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    }
}

#[utoipa::path(
    put,
    path = "/product",
    request_body = UpsertModel,
    responses(
        (status = 200, description = "Product updated", body = Model),
        (status = 404, description = "Product not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn put_product_handler(
    State(conn): State<DatabaseConnection>,
    Json(product): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    put_product(State(conn), Json(product)).await
}

// UPDATE
// PUT /product
// -- body 예:
//{
//     "id": 1,
//     "title": "test",
//     "price": 100,
//     "category": "test"
// }
pub async fn put_product(
    State(conn): State<DatabaseConnection>,
    Json(product): Json<UpsertModel>,
) -> Result<Json<Model>, AppError> {
    let result = match Entity::find_by_id(product.id.unwrap())
        .one(&conn).await {
            Ok(result) => result.ok_or(AppError::new(
                StatusCode::NOT_FOUND,
                "Product not found"
            ))?,
            Err(_) => return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error"
            )),
        };

    let new_product = ActiveModel {
        id: ActiveValue::Set(result.id),
        title: ActiveValue::Set(product.title.unwrap_or(result.title)),
        price: ActiveValue::Set(product.price.unwrap_or(result.price)),
        category: ActiveValue::Set(product.category.unwrap_or(result.category)),
    };

    match new_product.update(&conn).await {
        Ok(updated_product) => Ok(Json(updated_product)),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    }
}

#[utoipa::path(
    delete,
    path = "/product",
    params(
        ("id" = Option<i32>, Query, description = "Product ID"),
        ("title" = Option<String>, Query, description = "Product title"),
        ("price" = Option<i32>, Query, description = "Product price"),
        ("category" = Option<String>, Query, description = "Product category")
    ),
    responses(
        (status = 200, description = "Product deleted", body = String),
        (status = 404, description = "Product not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn delete_product_handler(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<UpsertModel>,
) -> Result<Json<&'static str>, AppError> {
    delete_product(State(conn), Query(params)).await
}

// DELETE /product?id=1&title=test&price=100&category=test
pub async fn delete_product(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<UpsertModel>,
) -> Result<Json<&'static str>, AppError> {
    let mut condition = Condition::any();

    if let Some(id) = params.id {
        condition = condition.add(Column::Id.eq(id));
    }

    if let Some(title) = params.title {
        condition = condition.add(Column::Title.contains(title));
    }

    if let Some(price) = params.price {
        condition = condition.add(Column::Price.eq(price));
    }

    if let Some(category) = params.category {
        condition = condition.add(Column::Category.contains(category));
    }

    let product = match Entity::find().filter(condition).one(&conn).await {
        Ok(product) => product.ok_or(AppError::new(
            StatusCode::NOT_FOUND,
            "Product not found"
        ))?,
        Err(_) => return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,  
            "Database error"
        )),
    };

    match product.delete(&conn).await {
        Ok(_) => Ok(Json("Product deleted")),
        Err(_) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error"
        )),
    }
}