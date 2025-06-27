use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    }
};
use futures_util::stream::StreamExt;
use serde_json::json;
use tokio::sync::broadcast::{self};
use tokio_stream::wrappers::BroadcastStream;

pub async fn subscribe(
    State(queue): State<broadcast::Sender<String>>,
) -> impl IntoResponse {
    let stream = BroadcastStream::new(queue.subscribe()).map(|msg| match msg {
        Ok(msg) => Ok(Event::default()
            .event("message")
            .data(json!(msg).to_string())),
        Err(e) => Err(e),
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn send(
    State(queue): State<broadcast::Sender<String>>,
    new_message: String,
) -> &'static str {
    queue.send(new_message).expect("Error sending message");

    "Message send"
}