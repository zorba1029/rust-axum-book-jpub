use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::body::Bytes;
use axum::http::StatusCode;
use axum::Json;
use axum::{Router, routing::post};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use axum::extract::State;


type Cache = Arc<Mutex<HashMap<String, Bytes>>>;

#[derive(Deserialize, Serialize, Debug)]
struct DogData {
    // 품종
    breed: String,
    // 사진 갯수
    num_pics: Option<u32>,
}

async fn proxy_handler(Json(dog_data): Json<DogData>) -> (StatusCode, Bytes) {
    // println!("dog_data: {:?}", dog_data);
    let mut url = format!("https://dog.ceo/api/breed/{}/images/random", &dog_data.breed);

    if let Some(num_pics) = dog_data.num_pics {
        url.push_str(&format!("/{}", num_pics));
    }
    let client = Client::new();
    let response = client.get(url).send().await.unwrap();

    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();

    (StatusCode::from_u16(status).unwrap(), body)
}

async fn proxy_handler_cached(
    State(state): State<Cache>,
    Json(dog_data): Json<DogData>) 
-> (StatusCode, Bytes) {
    // println!("dog_data: {:?}", dog_data);
    // cache check
    if let Some(body) = state.lock().unwrap().get(&dog_data.breed).cloned() {
        println!("cache hit");
        return (StatusCode::OK, body);
    }
    println!("cache miss");
    let mut url = format!("https://dog.ceo/api/breed/{}/images/random", &dog_data.breed);

    if let Some(num_pics) = dog_data.num_pics {
        url.push_str(&format!("/{}", num_pics));
    }
    let client = Client::new();
    let response = client.get(url).send().await.unwrap();

    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();

    let mut cache = state.lock().unwrap();
    cache.insert(dog_data.breed, body.clone());
    (StatusCode::from_u16(status).unwrap(), body)
}


#[tokio::main]
async fn main() {
    let state: Cache = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", post(proxy_handler))
        .route("/cached", post(proxy_handler_cached))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//-- request --
// curl -X POST http://localhost:8000/ -H "Content-Type: application/json" -d '{"breed": "chihuahua", "num_pics": 3}'
//-- response --
// {
// 	"message": [
// 		"https://images.dog.ceo/breeds/chihuahua/n02085620_14252.jpg",
// 		"https://images.dog.ceo/breeds/chihuahua/n02085620_2887.jpg",
// 		"https://images.dog.ceo/breeds/chihuahua/n02085620_6295.jpg"
// 	],
// 	"status": "success"
// }
