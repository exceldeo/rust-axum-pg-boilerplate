use axum::{routing::get, Router};

pub fn user_routes() -> Router {
    Router::new()
        .route("/profile", get(crate::handlers::user::get_profile))
}

