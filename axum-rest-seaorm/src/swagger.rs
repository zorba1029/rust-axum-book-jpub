use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema,
};
use serde::{Deserialize, Serialize};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

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
        crate::api::auth::login_handler,
        crate::api::text::get_text_handler,
    ),
    components(
        schemas(
            // Entities (DB 모델)
            crate::entities::users::Model,
            crate::entities::product::Model,
            crate::entities::category::Model,
            
            // API 요청/응답 스키마 (핸들러에 정의)
            crate::api::users::QueryParams,
            crate::api::users::DeleteParams,
            crate::api::auth::LoginRequest,
            
            // 공통 에러 응답
            ErrorResponse
        )
    ),
    modifiers(&SecurityAddon),
    info(
        title = "Axum REST API with SeaORM",
        version = "1.0.0",
        description = "REST API built with Axum and SeaORM",
        contact(
            name = "API Support",
            email = "zorbahouse@yahoo.com"
        )
    ),
    servers(
        (url = "http://localhost:8000", description = "Local server")
    )
)]
pub struct ApiDoc; 