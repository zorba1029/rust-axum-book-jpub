mod api;

use api::chat::{send, subscribe};
use shuttle_axum::axum::{
    routing::{get, post},
    Router,
};
use tokio::sync::broadcast;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

// async fn hello_world() -> &'static str {
//     "Hello, world!"
// }

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let message_queue: broadcast::Sender<String> = broadcast::channel(10).0;

    let app = Router::new()
        .nest("/chat",  
            Router::new()
                .route("/subscribe", get(subscribe))
                .route("/send", post(send))
        )
        .with_state(message_queue);

    Ok(app.into())
}

//-- run
// > shuttle run --port 3000

// > shuttle deploy --allow-dirty
