use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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

#[derive(Serialize, ToSchema)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}