use std::collections::HashMap;

use shuttle_axum::axum::{
    extract::{Query, State},
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    Json,
};
use futures_util::stream::StreamExt;
use serde_json::json;
use serde::Deserialize;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::entities::{
    chat::{ActiveModel as ActiveChat, Column, Entity as ChatEntity, Model as ChatModel},
    room::{ActiveModel as ActiveRoom, Entity as RoomEntity},
};
use crate::api::state::AppState;
use tracing::info;


pub async fn subscribe(
    // State(queue): State<broadcast::Sender<ChatModel>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let queue = app_state.queue.clone();
    let stream = BroadcastStream::new(queue.subscribe())
        .map(|message| match message {
            Ok(message) => Ok(Event::default()
                .event("message")
                .data(json!(message).to_string())),
            Err(e) => Err(e.to_string()),
        });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

// pub async fn send(
//     State(state): State<AppState>,
//     message: String,
// ) -> impl IntoResponse {
//     state.queue.send(message.clone()).unwrap();

//     Json(json!(format!("Received message: {}", message.clone())))
// }
#[derive(Deserialize)]
pub struct NewMessage {
    pub sender: String,
    pub message: String,
    pub room_id: i32,
}

pub async fn send(
    // State(conn): State<DatabaseConnection>,
    // State(queue): State<broadcast::Sender<ChatModel>>,
    State(app_state): State<AppState>,
    Json(new_message): Json<NewMessage>,
) -> Json<ChatModel> {
    let conn: DatabaseConnection = app_state.conn.clone();
    let queue = app_state.queue.clone();
    
    let room = RoomEntity::find_by_id(new_message.room_id)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();
    
    info!("send() - room: {:?}", room.clone());

    let participants: Vec<String> = serde_json::from_str(&room.participants).unwrap();
    let mut participants = participants;
    
    if !participants.contains(&new_message.sender) {
        participants.push(new_message.sender.clone());
    }

    let participants = serde_json::to_string(&participants).unwrap();

    let rooom = ActiveRoom {
        id: ActiveValue::Set(room.id),
        participants: ActiveValue::Set(participants),
    };

    rooom.update(&conn)
        .await
        .expect("Error updating room participants");

    let new_message = ActiveChat {
        id: ActiveValue::NotSet,
        sender: ActiveValue::Set(new_message.sender),
        message: ActiveValue::Set(new_message.message),
        room_id: ActiveValue::Set(new_message.room_id),
        timestamp: ActiveValue::Set(chrono::Utc::now().naive_utc()),
    };

    let new_message = new_message
        .insert(&conn) 
        .await
        .expect("Error inserting new message");

    queue.send(new_message.clone())
        .expect("Error sending message to queue");

    Json(new_message)
}

pub async fn get_chat(
    // State(conn): State<DatabaseConnection>,
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<ChatModel>> {
    let conn: DatabaseConnection = app_state.conn.clone();
    let room_id = params.get("room_id").unwrap();

    let result = ChatEntity::find()
        .filter(Column::RoomId.eq(room_id.parse::<i32>().unwrap()))
        .all(&conn)
        .await
        .unwrap();

    Json(result)
}
