use tokio::fs;
use utoipa::path;

#[utoipa::path(
    get,
    path = "/text",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Get text content", body = String),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    ),
    tag = "Text"
)]
pub async fn get_text_handler() -> String {
    fs::read_to_string("alice_in_wonderland.txt")
        .await
        .unwrap()
}
