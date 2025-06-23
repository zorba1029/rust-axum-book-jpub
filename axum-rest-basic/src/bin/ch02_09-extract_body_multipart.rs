use axum::Router;
use axum::routing::post;
use axum::extract::Multipart;

// ----------------------------------
// Request Body extract 사용 예
// Multipart body - POST http://localhost:8000/body_multipart
// ----------------------------------
async fn upload_file(mut body: Multipart) -> String {
    if let Ok(Some(field)) = body.next_field().await {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        format!("upload file /multipart: {}, {} bytes", name, data.len())
    } else {
        "no fiield found in multipart data".to_string()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/body_multipart", post(upload_file));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//-- headers ---
// Content-Type: multipart/form-data
// User-Agent: insomnia/11.2.0

//-- response --
// upload file /multipart: file, 418983 bytes

//-- response --
// * Preparing request to http://localhost:8000/body_multipart
// * Current time is 2025-06-23T15:12:28.044Z
// * Enable automatic URL encoding
// * Using default HTTP version
// * Enable timeout of 30000ms
// * Enable SSL validation
// * Found bundle for host: 0x11c05c5c180 [serially]
// * Can not multiplex, even if we wanted to
// * Re-using existing connection #50 with host localhost
// * Connected to localhost (127.0.0.1) port 8000 (#50)

// > POST /body_multipart HTTP/1.1
// > Host: localhost:8000
// > Content-Type: multipart/form-data; boundary=X-INSOMNIA-BOUNDARY
// > User-Agent: insomnia/11.2.0
// > Accept: */*
// > Content-Length: 418983

// | (64 KB hidden)
// | (64 KB hidden)
// | (64 KB hidden)
// | (64 KB hidden)
// | (64 KB hidden)
// | (64 KB hidden)
// | (25.2 KB hidden)

// * We are completely uploaded and fine
// * Mark bundle as not supporting multiuse

// < HTTP/1.1 200 OK
// < content-type: text/plain; charset=utf-8
// < content-length: 43
// < date: Mon, 23 Jun 2025 15:12:28 GMT


// * Received 43 B chunk
// * Connection #50 to host localhost left intact