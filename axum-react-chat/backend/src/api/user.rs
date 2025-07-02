use std::collections::HashMap;

#[cfg(feature = "shuttle")]
use shuttle_axum::axum::{
    extract::{Query, State},
    Json,
};

#[cfg(not(feature = "shuttle"))]
use axum::{
    extract::{Query, State},
    Json,
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection,
    EntityTrait, ModelTrait, QueryFilter,
};
use serde::Deserialize;

use crate::entities::users::{ActiveModel, Column, Entity as UserEntity, Model};
use crate::api::state::AppState;
use tracing::info;

pub async fn get_user(
    // State(conn): State<DatabaseConnection>,
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Model>> {
    let conn: DatabaseConnection = app_state.conn.clone();
    let mut condition = Condition::all();

    if let Some(id) = params.get("id") {
        condition = condition.add(Column::Id.eq(id.parse::<i32>().unwrap()));
    }

    if let Some(username) = params.get("username") {
        condition = condition.add(Column::Username.contains(username.clone()));
    }

    Json(UserEntity::find()
        .filter(condition)
        .all(&conn) 
        .await
        .unwrap(),
    )
}


#[derive(Deserialize)]
pub struct UpsertUser {
    id: Option<i32>,
    username: Option<String>,
    password: Option<String>,
}

pub async fn post_user(
    // State(conn): State<DatabaseConnection>,
    State(app_state): State<AppState>,
    Json(user): Json<UpsertUser>,
) -> Json<Model> {
    let conn: DatabaseConnection = app_state.conn.clone();
    let new_user = ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(user.username.unwrap()),
        password: ActiveValue::Set(user.password.unwrap_or("not-defined".to_string())),
    };
    info!("post_user() - new_user: {:?}", new_user.clone());

    let result = new_user.insert(&conn).await.unwrap();

    Json(result)
}

pub async fn put_user(
    // State(conn): State<DatabaseConnection>,
    State(app_state): State<AppState>,
    Json(user): Json<UpsertUser>,
) -> Json<Model> {
    let conn: DatabaseConnection = app_state.conn.clone();
    let result = UserEntity::find_by_id(user.id.unwrap())
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    let new_user = ActiveModel {
        id: ActiveValue::Set(result.id),
        username: ActiveValue::Set(user.username.unwrap()),
        password: ActiveValue::Set(user.password.unwrap()),
    };

    let result = new_user.update(&conn).await.unwrap();

    Json(result)
}

pub async fn delete_user(
    // State(conn): State<DatabaseConnection>,
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<&'static str> {
    let conn: DatabaseConnection = app_state.conn.clone();
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let mut condition = Condition::any();

    if let Some(id) = params.get("id") {
        condition = condition.add(Column::Id.eq(id.parse::<i32>().unwrap()));
    }

    if let Some(username) = params.get("username") {
        condition = condition.add(Column::Username.contains(username.clone()));
    }

    let user = UserEntity::find()
        .filter(condition)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    user.delete(&conn).await.unwrap();

    Json("Deleted")
}