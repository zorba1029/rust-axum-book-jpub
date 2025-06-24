use axum::Router;
use axum::routing::get;
use axum::debug_handler;

// ----------------------------------
// App State 사용 예
// -- AppState에서 특정 field를 참조하여 사용하는 방법
// ----------------------------------

#[debug_handler]
async fn handler() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

