use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "alice@example.com")]
    pub email: String,
    #[schema(example = "Passw0rd!")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    /// JWT access token
    #[schema(example = "<access_token>")]
    pub access_token: String,
    /// Refresh token
    #[schema(example = "<refresh_token>")]
    pub refresh_token: String,
}
