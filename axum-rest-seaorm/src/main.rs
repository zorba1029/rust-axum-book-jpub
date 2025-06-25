mod entities;
mod db;
mod api;
mod utils;
mod swagger;

use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use db::init_db;
use api::users;
use api::category;
use api::product;
use swagger::ApiDoc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let conn: DatabaseConnection = init_db().await;

    // OpenAPI 문서 생성
    let openapi = ApiDoc::openapi();

    let app = Router::new()
        .route("/user", get(users::get_user_handler))
        .route("/users", get(users::get_users_handler)
            .post(users::post_user_handler)
            .put(users::put_user_handler)
            .delete(users::delete_user_handler)
        )
        .route("/categories", get(category::get_category_handler)
            .post(category::post_category_handler)
            .delete(category::delete_category_handler)
        )
        .route("/product", get(product::get_product_handler)
            .post(product::post_product_handler)
            .put(product::put_product_handler)
            .delete(product::delete_product_handler)
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .with_state(conn);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    println!("Swagger UI available at http://localhost:8000/swagger-ui/");

    axum::serve(listener, app).await.unwrap();
}

//----------------------------------
// > tree ./axum-rest-seaorm -L 3 -a -I "target"
// ./axum-rest-seaorm
// ├── .env
// ├── .env-example
// ├── Cargo.lock
// ├── Cargo.toml
// ├── migration
// │   ├── Cargo.lock
// │   ├── Cargo.toml
// │   ├── README.md
// │   └── src
// │       ├── lib.rs
// │       ├── m20250624_091523_create_table.rs
// │       └── main.rs
// └── src
//     ├── api
//     │   ├── category.rs
//     │   ├── mod.rs
//     │   ├── product.rs
//     │   └── users.rs
//     ├── db
//     │   ├── init.rs
//     │   └── mod.rs
//     ├── entities
//     │   ├── category.rs
//     │   ├── mod.rs
//     │   ├── prelude.rs
//     │   ├── product.rs
//     │   └── users.rs
//     ├── main.rs
//     ├── swagger.rs
//     └── utils
//         ├── app_error.rs
//         └── mod.rs
