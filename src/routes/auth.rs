// File: src/routes/auth.rs

use axum::{routing::{post}, Router};

use crate::handlers::auth::{login_handler, register_handler, logout_handler, refresh_token_handler };

pub fn auth_routes() -> Router { // return non-generic Router state
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_token_handler)) // Placeholder for refresh handler
        .route("/logout", post(logout_handler))  // Placeholder for logout handler
}