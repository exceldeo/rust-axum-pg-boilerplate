use crate::middleware::auth::AuthenticatedUser;
use crate::repositories::user::find_user_by_id;
use crate::models::user::UserProfile;
use axum::{Extension, Json};
use sqlx::{Pool, Postgres};
use axum::http::StatusCode;

#[utoipa::path(
    get,
    path = "/api/user/profile",
    tag = "user",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "User profile", body = UserProfile),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_profile(
    AuthenticatedUser(user_id): AuthenticatedUser,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<UserProfile>, StatusCode> {
    let user = find_user_by_id(&pool, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let profile = UserProfile {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };
    Ok(Json(profile))
}