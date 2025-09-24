use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, ToSchema)]
pub struct User {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "alice")]
    pub username: String,
    #[schema(example = "alice@example.com")]
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[schema(example = 1696166400)] // Example Unix timestamp
    pub created_at: i64,
    #[schema(example = 1696166400)] // Example Unix timestamp
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NewUser {
    #[schema(example = "alice")]
    pub username: String,
    #[schema(example = "alice@example.com")]
    pub email: String,
    #[schema(example = "Passw0rd!")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfile {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "alice")]
    pub username: String,
    #[schema(example = "alice@example.com")]
    pub email: String,
    #[schema(example = 1696166400)] // Example Unix timestamp
    pub created_at: i64,
    #[schema(example = 1696166400)] // Example Unix timestamp
    pub updated_at: i64,
}
