use std::collections::HashMap;

use axum::{extract::{Query, State}, http::StatusCode, Json};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, 
    QueryFilter, QueryOrder, Order,
};
// use sea_orm::{
//     ActiveModelTrait, ActiveValue
// };
use crate::entities::users::{ActiveModel, Column, Entity, Model};
use crate::utils::AppError;


pub async fn get_user(
    Query(params): Query<HashMap<String, String>>,
    State(conn): State<DatabaseConnection>,
// ) -> Json<Model> {
) -> Result<Json<Model>, AppError> {
    // let conn = Database::connect(DATABASE_URL).await.unwrap();
    let mut condition = Condition::any();

    if let Some(id) = params.get("id") {
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
    if let Some(username) = params.get("username") {
        condition = condition.add(Column::Username.contains(username));
    }
    println!("condition: {:?}", condition);
    println!("id: {:?}", params.get("id"));
    println!("username: {:?}", params.get("username"));
    
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

pub async fn get_users(
    Query(params): Query<HashMap<String, String>>,
    State(conn): State<DatabaseConnection>,
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

