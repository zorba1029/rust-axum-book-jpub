use axum::Router;
use axum::routing::post;
use axum::extract::Form;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    alias: Option<String>,
}

// ----------------------------------
// Request Body extract 사용 예
// Form body - POST http://localhost:8000/
// ----------------------------------
async fn hello_form_body(Form(user): Form<User>) -> String {
    format!("hello form body : {} - {}", user.name, user.alias.as_ref().unwrap_or(&"no alias".to_string()))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/body_form", post(hello_form_body));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}