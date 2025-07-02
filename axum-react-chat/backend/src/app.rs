use crate::{
    api::{
        chat::{get_chat, send, subscribe},
        chat_room::{delete_room, get_room, post_room, put_room},
        state::AppState,
        user::{delete_user, get_user, post_user, put_user},
    },
};

#[cfg(feature = "shuttle")]
use shuttle_axum::axum::{
    self,
    routing::{get, post},
    Router,
};

#[cfg(not(feature = "shuttle"))]
use axum::{
    routing::{get, post},
    Router,
};

use migration::{Migrator, MigratorTrait};
use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use sqlx::PgPool;
use tokio::sync::broadcast;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

pub async fn create_app(pool: PgPool) -> Router {
    let conn = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    Migrator::up(&conn, None).await.unwrap();

    create_router(conn)
}

pub fn create_router(conn: DatabaseConnection) -> Router {
    let state = AppState {
        conn,
        queue: broadcast::channel(10).0,
    };

    Router::new()
        .nest(
            "/chat",
            Router::new()
                .route("/", get(get_chat)) // 채팅 메시지 조회
                .route("/subscribe", get(subscribe)) // 채팅 메시지 구독
                .route("/send", post(send)), // 채팅 메시지 전송
        )
        .route(
            "/room",
            get(get_room)
                .post(post_room)
                .put(put_room)
                .delete(delete_room),
        )
        .route(
            "/user",
            get(get_user)
                .post(post_user)
                .put(put_user)
                .delete(delete_user),
        )
        .layer(CorsLayer::new()
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_origin(Any))
        .fallback_service(
            ServeDir::new("static")
                // .append_index_html_on_directories(true)
                .not_found_service(ServeFile::new("static/index.html")),
        )
        .with_state(state)
} 