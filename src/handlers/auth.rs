use crate::dtos::auth::{LoginRequest, TokenResponse};
use crate::dtos::common::{ApiResponse, ApiResponseEmptyEnvelope, ApiResponseTokenEnvelope};
use crate::models::user::NewUser;
use crate::services::auth::{authenticate_user, register_user};
use crate::services::token::{generate_access_token, validate_refresh_token};
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json};
use serde_json::Value;
use sqlx::{Pool, Postgres};

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    request_body = NewUser,
    responses(
        (status = 200, description = "User registered", body = ApiResponseTokenEnvelope),
        (status = 400, description = "Bad request")
    )
)]
pub async fn register_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Value>, StatusCode> {
    match register_user(&pool, &new_user).await {
        Ok(token_pair) => Ok(Json(ApiResponse::success_ok(serde_json::json!(
            TokenResponse {
                access_token: token_pair.access_token,
                refresh_token: token_pair.refresh_token,
            }
        )))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

/// Login and receive access & refresh tokens
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login success", body = ApiResponseTokenEnvelope),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn login_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<Value>, StatusCode> {
    match authenticate_user(&pool, &payload.email, &payload.password).await {
        Ok(token_pair) => Ok(Json(ApiResponse::success_ok(serde_json::json!(
            TokenResponse {
                access_token: token_pair.access_token,
                refresh_token: token_pair.refresh_token,
            }
        )))),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

/// Logout user (invalidate refresh token)
#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "auth",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Logout success", body = ApiResponseEmptyEnvelope),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn logout_handler(headers: HeaderMap) -> Result<Json<Value>, StatusCode> {
    // Expect Bearer <refresh_token>
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    // Optionally validate token format; for stateless logout we just acknowledge
    let _ = validate_refresh_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok(Json(ApiResponse::success_ok(serde_json::json!({}))))
}

/// Refresh access token using refresh token from Authorization: Bearer <token>
#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "auth",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Token refreshed", body = ApiResponseTokenEnvelope),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn refresh_token_handler(headers: HeaderMap) -> Result<Json<Value>, StatusCode> {
    // Expect Bearer <refresh_token>
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    // Validate refresh token and mint a new access token
    let claims = validate_refresh_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let new_access = generate_access_token(claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(ApiResponse::success_ok(serde_json::json!(
        TokenResponse {
            access_token: new_access,
            refresh_token: token.to_string(),
        }
    ))))
}
