use axum::Router;
use axum::routing::post;

// ----------------------------------
// Request Body extract 사용 예
// Plain String body - POST http://localhost:8000/
// ----------------------------------
async fn hello_string_body(name: String) -> String {
    format!("hello string body : {}", name)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/body_text", post(hello_string_body));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}