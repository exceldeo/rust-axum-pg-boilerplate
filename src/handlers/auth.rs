use crate::models::user::NewUser;
use crate::services::auth::{register_user, authenticate_user};
use crate::services::token::{generate_access_token, validate_refresh_token};
use axum::{Extension, Json};
use axum::http::{HeaderMap, StatusCode};
use sqlx::{Pool, Postgres};
use serde_json::Value;
use utoipa::ToSchema;

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    request_body = NewUser,
    responses(
        (status = 200, description = "User registered", body = serde_json::Value),
        (status = 400, description = "Bad request")
    )
)]
pub async fn register_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Value>, StatusCode> {
    match register_user(&pool, &new_user).await {
        Ok(user) => Ok(Json(serde_json::json!({
            "status": "success",
            "message": "User registered successfully",
            "user_id": user.id
        }))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

#[derive(serde::Deserialize, ToSchema)]
pub struct AuthPayload {
    #[schema(example = "alice@example.com")]
    pub email: String,
    #[schema(example = "Passw0rd!")]
    pub password: String,
}

/// Login and receive access & refresh tokens
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = AuthPayload,
    responses(
        (status = 200, description = "Login success", body = serde_json::Value),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn login_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Value>, StatusCode> {
    // i want to print for testing only
    println!("Login attempt for email: {}", payload.email);


    match authenticate_user(&pool, &payload.email, &payload.password).await {
        Ok(token_pair) => Ok(Json(serde_json::json!({
            "status": "success",
            "access_token": token_pair.access_token,
            "refresh_token": token_pair.refresh_token
        }))),
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
        (status = 200, description = "Logout success", body = serde_json::Value),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn logout_handler(headers: HeaderMap) -> Result<Json<Value>, StatusCode> {
    // Expect Bearer <refresh_token>
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    let token = auth_header.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    // Optionally validate token format; for stateless logout we just acknowledge
    let _ = validate_refresh_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok(Json(serde_json::json!({ "status": "success" })))
}

/// Refresh access token using refresh token from Authorization: Bearer <token>
#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "auth",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Token refreshed", body = serde_json::Value),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn refresh_token_handler(headers: HeaderMap) -> Result<Json<Value>, StatusCode> {
    // Expect Bearer <refresh_token>
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    let token = auth_header.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    // Validate refresh token and mint a new access token
    let claims = validate_refresh_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let new_access = generate_access_token(claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(serde_json::json!({ 
        "status":"success",
        "access_token": new_access 
    })))
}