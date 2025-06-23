use axum::{routing::{get, post}, Router};

//--------------------------------
// nested routes 사용 예
//
// http://localhost:8000/api/user/hello
// http://localhost:8000/api/team/
async fn user_hello() -> &'static str {
    "user hello"
}

#[tokio::main]
async fn main() {
    let usr_routes = Router::new()
        .route("/", get(|| async { "user" }))
        .route("/hello", get(user_hello))
        .route("/login", get(|| async { "login" }));
    let team_routes = Router::new()
        .route("/", post(|| async { "teams" }));
    let api_routes = Router::new()
        .nest("/user", usr_routes)
        .nest("/team", team_routes);
    let app = Router::new()
        .nest("/api", api_routes);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

