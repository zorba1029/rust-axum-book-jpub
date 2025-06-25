mod entities;
mod db;
mod api;
mod utils;

use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;

use db::init_db;
use api::users;
use api::category;
use api::product;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let conn: DatabaseConnection = init_db().await;

    let app = Router::new()
        .route("/user", get(users::get_user))
        .route("/users", get(users::get_users)
            .post(users::post_user)
            .put(users::put_user)
            .delete(users::delete_user)
        )
        .route("/category", get(category::get_category)
            .post(category::post_category)
            .delete(category::delete_category)
        )
        .route("/product", get(product::get_product)
            .post(product::post_product)
            .put(product::put_product)
            .delete(product::delete_product)
        )
        .with_state(conn);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
