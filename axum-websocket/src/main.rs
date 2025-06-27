use std::sync::Arc;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State,WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{
    sink::SinkExt, 
    stream::{StreamExt, SplitSink, SplitStream},
};
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Mutex,
};


#[derive(Debug, Clone)]
struct AppState {
    broadcast_tx: Arc<Mutex<Sender<Message>>>,
}

async fn websocket_handler(ws: WebSocketUpgrade, State(app_state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(socket, app_state))
}

// fn split<Item>(self) -> (SplitSink<Self, Item>, SplitStream<Self>)
// where
//      Self: Sink<Item> + Sized,
// {
//      let (sink, stream) = split::split(self);
//      (
//          crate::sink::assert_sink::<Item, Self::Error, _>(sink),
//          assert_stream::<Self::Item, _>(stream),
//      )
// }
// 

async fn handle_websocket(ws: WebSocket, app_state: AppState) {
    // (ws_tx, ws_rx):(SplitSink<WebSocket, Message>, SplitStream<WebSocket>)
    let (ws_tx, ws_rx) = ws.split();
    let ws_tx: Arc<Mutex<SplitSink<WebSocket, Message>>> = Arc::new(Mutex::new(ws_tx));

    {
        let broadcast_rx = app_state.broadcast_tx.lock().await.subscribe();
        tokio::spawn(async move {
            send_to_websocket(ws_tx, broadcast_rx).await;
        });
    }
    
    receive_from_websocket(ws_rx, app_state.broadcast_tx).await;
    // while let Some(Ok(msg)) = ws_rx.next().await {
    //     ws_tx
    //         .send(Message::Text(format!("Message received: {}", msg.to_text().unwrap())))
    //         .await
    //         .unwrap();
    // }
}

async fn send_to_websocket(
    client_tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    mut broadcast_rx: Receiver<Message>,
) {
    while let Ok(msg) = broadcast_rx.recv().await {
        if client_tx.lock().await.send(msg).await.is_err() {
            return;
        }

    }
}

async fn receive_from_websocket(
    mut client_rx: SplitStream<WebSocket>,
    broadcast_tx: Arc<Mutex<Sender<Message>>>,
) {
    while let Some(Ok(msg)) = client_rx.next().await {
        if matches!(msg, Message::Close(_)) {
            return;
        }
        if broadcast_tx.lock().await.send(msg).is_err() {
            println!("Failed to broadcast a message.");
        }
    }
}


#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(32);
    let app_state = AppState {
        broadcast_tx: Arc::new(Mutex::new(tx)),
    };

    let app = Router::new().route("/ws", get(websocket_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


