use crate::dtos::common::{ApiResponse, ApiResponseUserProfileEnvelope};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::UserProfile;
use crate::repositories::user::find_user_by_id;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde_json::Value;
use sqlx::{Pool, Postgres};

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
) -> Result<Json<Value>, StatusCode> {
    let user = find_user_by_id(&pool, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse::success_ok(serde_json::json!(
        UserProfile {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    ))))
}
