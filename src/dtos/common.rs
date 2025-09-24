use crate::dtos::auth::TokenResponse;
use crate::models::user::UserProfile;
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse {
    /// Status response
    #[schema(example = "error")]
    pub status: String,
    /// HTTP status code
    #[schema(example = 400)]
    pub code: u16,
    /// Message detail
    #[schema(example = "Bad Request")]
    pub message: String,

    /// Optional additional data
    pub data: Option<serde_json::Value>,
}

impl ApiResponse {
    pub fn success_with(
        code: u16,
        message: impl Into<String>,
        data: serde_json::Value,
    ) -> serde_json::Value {
        json!({
            "status": "success",
            "code": code,
            "message": message.into(),
            "data": data,
        })
    }

    pub fn success_ok(data: serde_json::Value) -> serde_json::Value {
        Self::success_with(200, "OK", data)
    }

    pub fn error_with(code: u16, message: impl Into<String>) -> serde_json::Value {
        json!({
            "status": "error",
            "code": code,
            "message": message.into(),
        })
    }
}

// Swagger-visible envelopes for specific payloads
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponseTokenEnvelope {
    /// Status response
    #[schema(example = "success")]
    pub status: String,
    /// HTTP status code
    #[schema(example = 200)]
    pub code: u16,
    /// Message detail
    #[schema(example = "OK")]
    pub message: String,
    /// Token response payload
    pub data: TokenResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponseEmptyEnvelope {
    /// Status response
    #[schema(example = "success")]
    pub status: String,
    /// HTTP status code
    #[schema(example = 200)]
    pub code: u16,
    /// Message detail
    #[schema(example = "OK")]
    pub message: String,
    /// Empty object
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponseUserProfileEnvelope {
    /// Status response
    #[schema(example = "success")]
    pub status: String,
    /// HTTP status code
    #[schema(example = 200)]
    pub code: u16,
    /// Message detail
    #[schema(example = "OK")]
    pub message: String,
    /// User profile payload
    pub data: UserProfile,
}
