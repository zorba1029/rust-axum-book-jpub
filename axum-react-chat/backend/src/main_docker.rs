mod api;
mod db;
mod entities;
mod app;

#[cfg(feature = "shuttle")]
use shuttle_axum::axum;

#[cfg(not(feature = "shuttle"))]
use axum;

use db::init_db;
use migration::{Migrator, MigratorTrait};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer().without_time())
        .with(EnvFilter::from_default_env())
        .try_init()
        .ok();

    let db = init_db().await;
    Migrator::up(&db, None).await.unwrap();
    let app = app::create_router(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// > cargo build --bin docker
// -- run
// > docker-compose up --build