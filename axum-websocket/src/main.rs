use std::sync::Arc;
use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        State,WebSocketUpgrade,
    },
    http::{HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
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
    println!("websocket_handler");
    ws.on_upgrade(|socket| handle_websocket(socket, app_state))
}

    // -- ws.split() 함수 원형 -----
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

    // 1. 브로드캐스트 메시지를 클라이언트에게 보내는 태스크를 별도로 생성 
    // 이 블록이 끝나면 MutexGuard가 drop되면서 lock이 자동으로 해제.
    // 이렇게 하면 아래의 receive_from_websocket에서 데드락 없이 broadcast_tx에 접근 가능.
    {
        // app_state.broadcast_tx.lock()의 결과물(MutexGuard)은 이 블록 안에서만 유효합니다.
        let broadcast_rx = app_state.broadcast_tx.lock().await.subscribe();
        tokio::spawn(async move {
            send_to_websocket(ws_tx, broadcast_rx).await;
        });
    } // <-- 이 지점에서 lock이 해제됨
    
    // 2. 클라이언트로부터 오는 메시지를 처리하는 루프를 'await' 함
    receive_from_websocket(ws_rx, app_state.broadcast_tx).await;

    // 3. (연결 종료 후) 후처리 코드 (예: 로그 남기기)
    println!("WebSocket connection closed");
}

//-- 위 함수를 다음과 같이 해도 됨 ---
// -- 명시적으로 lock()을 release 할 수 있음:
// async fn handle_websocket(ws: WebSocket, app_state: AppState) {
//     use std::mem;
//     // (ws_tx, ws_rx):(SplitSink<WebSocket, Message>, SplitStream<WebSocket>)
//     let (ws_tx, ws_rx) = ws.split();
//     let ws_tx: Arc<Mutex<SplitSink<WebSocket, Message>>> = Arc::new(Mutex::new(ws_tx));

//     // 1. MutexGuard를 얻기 위해 lock()을 호출하고, 그 결과를 변수에 저장한다.
//     let guard = app_state.broadcast_tx.lock().await;

//     // 2. MutexGuard를 사용하여 원하는 작업을 수행한다.
//     let broadcast_rx = guard.subscribe();

//     // 3. 더 이상 잠금이 필요 없으므로, MutexGuard를 명시적으로 drop하여 잠금을 해제한다.
//     mem::drop(guard);  // <<-- 이 부분이 중요함.

//     // 4. 이제 broadcast_tx는 잠겨있지 않다.
//     tokio::spawn(async move {
//         send_to_websocket(ws_tx, broadcast_rx).await;
//     });
    
//     // 5. 클라이언트로부터 오는 메시지를 처리하는 루프를 'await' 함
//     receive_from_websocket(ws_rx, app_state.broadcast_tx).await;

//     // 6.(연결 종료 후) 후처리 코드 (예: 로그 남기기)
//     println!("WebSocket connection closed");
// }

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

async fn authenticate(
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if headers.get("Authorization")
        .map(|value| value == "Bearer token")
        .unwrap_or(false) 
    {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(32);
    let app_state = AppState {
        broadcast_tx: Arc::new(Mutex::new(tx)),
    };

    let app = Router::new().route("/ws", get(websocket_handler))
        .route_layer(middleware::from_fn(authenticate))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


