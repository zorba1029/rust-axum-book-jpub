use axum::{routing::{delete, get, post, put}, Router};
//--------------------------------
// Route 사용 예
//
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Welcome to Axum!" }))
        .route("/", post(|| async { "Create a new data!" }))
        .route("/", put(|| async { "Update an existing data!" }))
        .route("/", delete(|| async { "Delete an existing data!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

