use axum::{
    extract::{FromRequestParts}, // Removed State
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
// REMOVED: use axum_extra::extract::PrivateCookie;
use crate::services::token::validate_access_token;
use uuid::Uuid;
pub struct AuthenticatedUser(pub Uuid);

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let token = if let Some(header) = auth_header {
            if let Some(token_str) = header.strip_prefix("Bearer ") {
                token_str
            } else {
                return Err(StatusCode::UNAUTHORIZED.into_response());
            }
        } else {
            return Err(StatusCode::UNAUTHORIZED.into_response());
        };

        match validate_access_token(token) {
            Ok(claims) => Ok(AuthenticatedUser(claims.sub)),
            Err(_) => Err(StatusCode::UNAUTHORIZED.into_response()),
        }
    }
}