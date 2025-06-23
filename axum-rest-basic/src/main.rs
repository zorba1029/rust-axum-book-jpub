
//--------------------------------
// Route 사용 예
//
// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/", get(|| async { "Welcome to Axum!" }))
//         .route("/", post(|| async { "Create a new data!" }))
//         .route("/", put(|| async { "Update an existing data!" }))
//         .route("/", delete(|| async { "Delete an existing data!" }));
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

//--------------------------------
// Route Handler 사용 예
//
// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/", 
//             get(|| async { "Welcome to Axum!" })
//                 .post(|| async { "Create a new data!" })
//                 .put(|| async { "Update an existing data!" })
//                 .delete(|| async { "Delete an existing data!" })
//         );
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

//--------------------------------
// nested routes 사용 예
//
// use axum::{routing::{delete, get, post, put}, Router};

// async fn user_hello() -> &'static str {
//     "user hello"
// }

// #[tokio::main]
// async fn main() {
//     let usr_routes = Router::new()
//         .route("/", get(|| async { "user" }))
//         .route("/hello", get(user_hello))
//         .route("/login", get(|| async { "login" }));
//     let team_routes = Router::new()
//         .route("/", post(|| async { "teams" }));
//     let api_routes = Router::new()
//         .nest("/user", usr_routes)
//         .nest("/team", team_routes);
//     let app = Router::new()
//         .nest("/api", api_routes);
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

//--------------------------------
// extract::Path 사용 예
// route parameter - http://localhost:8000/{id}/{name}
//--------------------------------
// use axum::{routing::{delete, get, post, put}, Router};
// use axum::extract::Path;

// async fn hello(Path((id, name)): Path<(i32, String)>) -> String {
//     format!("hello - {}: {}", id, name)
// }

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/:id/:name", get(hello));
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

//--------------------------------
// extract::Query 사용 예
// query parameter - http://localhost:8000/?id=1&category=toys
//--------------------------------
// use axum::{routing::get, Router};
// use axum::extract::{Path, Query};
// use std::collections::HashMap;
// use serde::Deserialize;

// // route parameter - http://localhost:8000/{id}/{name}
// async fn hello_path(Path((id, name)): Path<(i32, String)>) -> String {
//     format!("hello path - {}: {}", id, name)
// }


// // query parameter - http://localhost:8000/products?id=1&name=toys
// async fn hello_query(Query(user): Query<HashMap<String, String>>) -> String {
//     format!("hello query - {}: {}", user["id"].parse::<i32>().unwrap(), user["name"])
// }

// #[derive(Debug, Deserialize)]
// struct User {
//     id: i32,
//     name: Option<String>,
// }

// // query parameter - http://localhost:8000/user?id=1&name=toys
// async fn hello_query_user(Query(user): Query<User>) -> String {
//     format!("hello query user - {}: {}", user.id, user.name.as_ref().unwrap_or(&"no name".to_string()))
// }


// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/:id/:name", get(hello_path))
//         .route("/products", get(hello_query))
//         .route("/user", get(hello_query_user));
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

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