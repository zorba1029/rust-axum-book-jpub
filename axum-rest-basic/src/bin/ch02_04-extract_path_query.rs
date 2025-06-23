use axum::Router;
use axum::routing::get;
use axum::extract::{Path, Query};
use std::collections::HashMap;
use serde::Deserialize;

//--------------------------------
//--------------------------------
// extract::Path,
// extract::Query 사용 예
// query parameter - http://localhost:8000/?id=1&category=toys
//--------------------------------


// route parameter - http://localhost:8000/{id}/{name}
async fn hello_path(Path((id, name)): Path<(i32, String)>) -> String {
    format!("hello path - {}: {}", id, name)
}


// query parameter - http://localhost:8000/products?id=1&name=toys
async fn hello_query(Query(user): Query<HashMap<String, String>>) -> String {
    format!("hello query - {}: {}", user["id"].parse::<i32>().unwrap(), user["name"])
}

#[derive(Debug, Deserialize)]
struct User {
    id: i32,
    name: Option<String>,
}

// query parameter - http://localhost:8000/user?id=1&name=toys
async fn hello_query_user(Query(user): Query<User>) -> String {
    format!("hello query user - {}: {}", user.id, user.name.as_ref().unwrap_or(&"no name".to_string()))
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:id/:name", get(hello_path))
        .route("/products", get(hello_query))
        .route("/user", get(hello_query_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
