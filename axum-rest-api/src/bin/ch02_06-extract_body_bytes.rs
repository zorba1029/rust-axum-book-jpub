use axum::Router;
use axum::routing::post;
use axum::body::Bytes;

// ----------------------------------
// Request Body extract 사용 예
// Bytes body - POST http://localhost:8000/
// ----------------------------------
async fn hello_bytes_body(body: Bytes) -> String {
    format!("hello bytes body : {:?}", String::from_utf8_lossy(&body))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/body_bytes", post(hello_bytes_body));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}