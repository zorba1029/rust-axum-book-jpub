mod api;
mod entities;

use api::{
    chat::{get_chat, send, subscribe},
    chat_room::{delete_room, get_room, post_room, put_room},
    state::AppState,
    user::{delete_user, get_user, post_user, put_user},
};
use shuttle_axum::axum::{
    routing::{get, post},
    Router,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::SqlxPostgresConnector;
use sqlx::PgPool;
use tokio::sync::broadcast;
use  tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};


#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer().without_time())
        .with(EnvFilter::from_default_env())
        .init();

    let state = AppState { 
        conn: SqlxPostgresConnector::from_sqlx_postgres_pool(pool),
        queue: broadcast::channel(10).0,
    };

    Migrator::up(&state.conn, None).await.unwrap();
    
    let app = Router::new()
        .nest("/chat", 
            Router::new()
                .route("/", get(get_chat)) // 채팅 메시지 조회
                .route("/subscribe", get(subscribe)) // 채팅 메시지 구독
                .route("/send", post(send)) // 채팅 메시지 전송
        )
        .route("/room",
            get(get_room)
                .post(post_room)
                .put(put_room)
                .delete(delete_room),
        )
        .route("/user",
            get(get_user)
                .post(post_user)
                .put(put_user)
                .delete(delete_user),
        )
        .layer(CorsLayer::new()
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_origin(Any)
        )
        .fallback_service(
            ServeDir::new("static")
                // .append_index_html_on_directories(true)
                .not_found_service(ServeFile::new("static/index.html"))
        )
        .with_state(state);
        
    Ok(app.into())
}


// # shuttle feature를 활성화하여 빌드/배포
// shuttle run --port 3000
// cargo build --features shuttle
// shuttle deploy --features shuttle