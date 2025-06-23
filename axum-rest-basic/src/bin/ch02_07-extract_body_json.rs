use axum::Router;
use axum::routing::post;
use axum::extract::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    name: String,
}

// ----------------------------------
// Request Body extract 사용 예
// JSON body - POST http://localhost:8000/
// ----------------------------------
async fn hello_json_body(user: Json<User>) -> String {
    format!("hello json body : {:?}", user.name)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/body_json", post(hello_json_body));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}