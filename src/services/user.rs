use crate::models::user::UserProfile;
use crate::repositories::user::find_user_by_id;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn service_get_profile(
    pool: &Pool<Postgres>,
    user_id: Uuid,
) -> Result<UserProfile, String> {
    let user = find_user_by_id(pool, user_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;

    Ok(UserProfile {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
    })
}
