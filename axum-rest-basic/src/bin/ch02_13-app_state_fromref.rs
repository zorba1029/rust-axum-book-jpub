use axum::extract::{FromRef, State};
use axum::Router;
use axum::routing::get;
use std::sync::{Arc, Mutex};


// ----------------------------------
// App State 사용 예
// -- AppState에서 특정 field를 참조하여 사용하는 방법
// ----------------------------------

#[derive(FromRef, Clone)]
struct AppState {
    auth_token: String,
    current_users: i32,
    data: Arc<Mutex<Vec<u8>>>,
}


async fn hello_token(State(auth_token): State<String>) -> String {
    format!("Hello, Token - {}", auth_token)
}

async fn hello_users(State(users): State<i32>) -> String {
    format!("Hello, Users - {}", users)
}

async fn hello_app_state_arc(State(data): State<Arc<Mutex<Vec<u8>>>>) -> String {
    let mut data = data.lock().unwrap();
    data[0] += 1;
    format!("Hello, World! {:?}", data)
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        auth_token: "1234567890".to_string(),
        current_users: 100,
        data: Arc::new(Mutex::new(vec![0; 3])),
    };

    let app = Router::new()
        .route("/token", get(hello_token))
        .route("/users", get(hello_users))
        .route("/app_data", get(hello_app_state_arc))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

