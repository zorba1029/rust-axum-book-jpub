use utoipa::{OpenApi, ToSchema};
use serde::{Deserialize, Serialize};

// 에러 응답 스키마 (이것만 swagger.rs에 정의)
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "Resource not found")]
    pub error: String,
}

// API 문서 구조체
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::users::get_user_handler,
        crate::api::users::get_users_handler,
        crate::api::users::post_user_handler,
        crate::api::users::put_user_handler,
        crate::api::users::delete_user_handler,
        crate::api::category::get_category_handler,
        crate::api::category::post_category_handler,
        crate::api::category::delete_category_handler,
        crate::api::product::get_product_handler,
        crate::api::product::post_product_handler,
        crate::api::product::put_product_handler,
        crate::api::product::delete_product_handler,
    ),
    components(
        schemas(
            // Entities (DB 모델)
            crate::entities::users::Model,
            crate::entities::product::Model,
            crate::entities::category::Model,
            
            // API 요청/응답 스키마 (핸들러에 정의)
            crate::api::users::UpsertModel,
            crate::api::product::UpsertModel,
            crate::api::category::UpsertModel,
            
            // 공통 에러 응답
            ErrorResponse
        )
    ),
    info(
        title = "Axum REST API with SeaORM",
        version = "1.0.0",
        description = "REST API built with Axum and SeaORM",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    ),
    servers(
        (url = "http://localhost:8000", description = "Local server")
    )
)]
pub struct ApiDoc; 