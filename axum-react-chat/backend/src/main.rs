#![cfg(feature = "shuttle")]

mod api;
mod entities;
mod app;

use sqlx::PgPool;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer().without_time())
        .with(EnvFilter::from_default_env())
        .try_init()
        .ok();

    let app = app::create_app(pool).await;

    Ok(app.into())
}


// # shuttle feature를 활성화하여 빌드/배포
// shuttle run --port 3000
// cargo build --features shuttle
// shuttle deploy --features shuttle

// # 로컬 빌드
// cargo build --features shuttle

// # 로컬 실행
// cargo shuttle run --port 3000