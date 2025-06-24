use axum::extract::State;
use axum::Router;
use axum::routing::get;
use std::sync::{Arc, Mutex};

// ----------------------------------
// App State 사용 예
// - GET http://localhost:8000/handler  
// ----------------------------------
// async fn hello_app_state(State(mut data): State<Vec<u8>>) -> String {
//     data[0] += 1;
//     format!("Hello, World! {:?}", data)
// }

// make app state with Arc<Mutex<Vec<u8>>> so that it can be shared/modified between threads
async fn hello_app_state_arc(State(data): State<Arc<Mutex<Vec<u8>>>>) -> String {
    let mut data = data.lock().unwrap();
    data[0] += 1;
    format!("Hello, World! {:?}", data)
}


#[tokio::main]
async fn main() {
    // let data = vec![0; 3];
    let data_arc = Arc::new(Mutex::new(vec![0; 3]));

    let app = Router::new()
        // .route("/app_data", get(hello_app_state))
        .route("/app_data_arc", get(hello_app_state_arc))
        // .with_state(data)
        .with_state(data_arc);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

