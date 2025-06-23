use axum::Router;
use axum::routing::get;
use axum::http::header::{HeaderMap, CONTENT_TYPE, USER_AGENT};

// ----------------------------------
// Request Body extract 사용 예
// Multipart body - POST http://localhost:8000/
// ----------------------------------
async fn hello_header(headers: HeaderMap) -> String {
    let user_agent = headers
        .get(USER_AGENT)
        .map(|v| v.to_str().unwrap().to_string());
    let content_type = headers
        .get(CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().to_string());
    format!("User-Agent: {:?}, Content-Type: {:?}", 
        user_agent.unwrap_or_default(), content_type.unwrap_or_default())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/header", get(hello_header));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// -- headers ---
// User-Agent: insomnia/11.2.0
// Content-Type: text/plain

// -- response --
// User-Agent: "insomnia/11.2.0", Content-Type: "text/plain"
