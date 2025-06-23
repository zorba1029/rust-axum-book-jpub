use axum::{routing::{get}, Router};

//--------------------------------
// Route Handler 사용 예
//
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", 
            get(|| async { "Welcome to Axum!" })
                .post(|| async { "Create a new data!" })
                .put(|| async { "Update an existing data!" })
                .delete(|| async { "Delete an existing data!" })
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}