mod entities;
mod db;
mod api;
mod utils;

use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;

use db::init_db;
use api::{get_user, get_users};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let conn: DatabaseConnection = init_db().await;

    let app = Router::new()
        .route("/user", get(get_user))
        .route("/users", get(get_users))
        .with_state(conn);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
