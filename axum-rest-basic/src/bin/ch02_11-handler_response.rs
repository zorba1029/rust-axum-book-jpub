use axum::Router;
use axum::routing::get;
use axum::extract::{Json, Path};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use axum::http::StatusCode;
use axum_extra::{TypedHeader, headers::{ContentType, ContentLength}};

// ----------------------------------
// Request Body extract 사용 예
// Multipart body - POST http://localhost:8000/
// ----------------------------------
#[derive(Debug, Serialize)]
struct Message {
    message: &'static str,
}

async fn hello_handler() -> Json<Message> {
    Json(Message {
        message: "Hello, World!",
    })
}

pub async fn hello_handler_nested() -> Json<Value> {
    Json(json!({
        "itmes" : [
            {
                "name": "apple",
                "details": {
                    "color": "red",
                    "origin": "South Korea"
                }
            },
            {
                "name": "banana",
                "details": {
                    "color": "yellow",
                    "origin": "South America"
                }
            }
        ]
    }))
}

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub details: ItemDetails,
}

#[derive(Serialize, Deserialize)]
pub struct ItemDetails {
    pub color: String,
    pub origin: String,
}

async fn hello_handler_inventory() -> Json<Inventory> {
    Json(Inventory {
        items: vec![
            Item {
                name: "apple".to_string(),
                details: ItemDetails {
                    color: "red".to_owned(),
                    origin: "South Korea".to_owned(),
                },
            },
            Item {
                name: "banana".to_string(),
                details: ItemDetails {
                    color: "yellow".to_string(),
                    origin: "South America".to_string(),
                },
            },
        ],
    })
}

async fn hello_handler_inventory_status_code() -> (StatusCode, Json<Inventory>) {
    (
        StatusCode::CREATED, 
        Json(Inventory {
            items: vec![
                Item {
                    name: "apple".to_string(),
                    details: ItemDetails {
                        color: "red".to_owned(),
                        origin: "South Korea".to_owned(),
                    },
                },
                Item {
                    name: "banana".to_string(),
                    details: ItemDetails {
                        color: "yellow".to_string(),
                        origin: "South America".to_string(),
                    },
                },
            ],
        })
    )
}

pub async fn hello_handler_nested_header_status_code() -> (TypedHeader<ContentType>, (StatusCode, Json<Value>)) {
    (
        TypedHeader(ContentType::json()),
        (
            StatusCode::CREATED, 
            Json(json!({
                "itmes" : [
                    {
                        "name": "apple",
                        "details": {
                            "color": "red",
                            "origin": "South Korea"
                        }
                    },
                    {
                        "name": "banana",
                        "details": {
                            "color": "yellow",
                            "origin": "South America"
                        }
                    }
                ]
            }))
        )
    )
}

pub async fn hello_handler_multiple_headers_status_code(Path(num): Path<i32>) 
-> (TypedHeader<ContentType>, TypedHeader<ContentLength>, (StatusCode, Json<Value>)) {
// -> (TypedHeader<ContentType>, (StatusCode, Json<Value>)) {
    match num {
        0 => (
            TypedHeader(ContentType::json()),
            TypedHeader(ContentLength(27)),
            (
                StatusCode::CREATED, 
                Json(json!({ "message" : "Hello, World!".to_string() })),
            ),
        ),
        _ => (
            TypedHeader(ContentType::json()),
            TypedHeader(ContentLength(36)),
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(json!({ "messages" : "Error during creation".to_string() }))
            ),
        )
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/handler", get(hello_handler))
        .route("/handler_nested", get(hello_handler_nested))
        .route("/handler_inventory", get(hello_handler_inventory))
        .route("/handler_inventory_status_code", get(hello_handler_inventory_status_code))
        .route("/handler_nested_header_status_code", get(hello_handler_nested_header_status_code))
        .route("/handler_multiple_headers_status_code/:num", get(hello_handler_multiple_headers_status_code));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// -- headers ---
// User-Agent: insomnia/11.2.0
// Content-Type: text/plain

// -- response --
// User-Agent: "insomnia/11.2.0", Content-Type: "text/plain"
