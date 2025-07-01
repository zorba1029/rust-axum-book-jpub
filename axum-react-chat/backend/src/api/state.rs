use crate::entities::chat::Model as ChatModel;

use sea_orm::DatabaseConnection;
// use axum::extract::FromRef;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub queue: broadcast::Sender<ChatModel>,
}
