// ----------------------------------
// Request Body extract 사용 예
// Plain String body - POST http://localhost:8000/
// ----------------------------------
use axum::{routing::post, Router};
use axum::extract::Json;
use serde::{Deserialize, Serialize};

async fn hello_string_body(name: String) -> String {
    format!("hello string body : {}", name)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(hello_string_body));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}