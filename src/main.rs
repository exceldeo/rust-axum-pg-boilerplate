use axum::{routing::get, Extension, Router};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener; // ADD THIS
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Impor modul yang kita buat
mod db;
mod dtos;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes; // Menambahkan ini untuk modularisasi rute
mod services;
mod utils;
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::register_handler,
        crate::handlers::auth::login_handler,
        crate::handlers::auth::logout_handler,
        crate::handlers::auth::refresh_token_handler,
        crate::handlers::user::get_profile,
    ),
    components(
        schemas(
            crate::models::user::NewUser,
            crate::models::user::User,
            crate::models::user::UserProfile,
            crate::models::token::TokenPair,
            crate::models::token::AuthClaims,
            crate::dtos::auth::LoginRequest,
            crate::dtos::auth::TokenResponse,
            crate::dtos::common::ApiResponse,
            crate::dtos::common::ApiResponseTokenEnvelope,
            crate::dtos::common::ApiResponseEmptyEnvelope,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "user", description = "User endpoints")
    )
,
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
        use utoipa::openapi::ComponentsBuilder;

        let bearer = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );

        let mut components = openapi
            .components
            .clone()
            .unwrap_or_else(|| ComponentsBuilder::new().build());
        components = ComponentsBuilder::from(components)
            .security_scheme("bearerAuth", bearer)
            .build();
        openapi.components = Some(components);
    }
}

#[tokio::main]
async fn main() {
    // Memuat variabel lingkungan dari file .env
    dotenv().ok();

    // 1. Menyiapkan Pool Koneksi Database
    let db_pool = db::setup_db_pool()
        .await
        .expect("Failed to connect to the database");

    // 2. Mengatur CORS (Cross-Origin Resource Sharing)
    // Ini penting untuk mengizinkan permintaan dari domain lain (misalnya, frontend)
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    // 3. Membangun Router Utama Aplikasi
    // Di sini kita menggabungkan semua rute dari modul `routes`
    let app = Router::new()
        // Basic health and root routes to avoid 404 on base URL
        .route("/", get(|| async { "OK" }))
        .route("/healthz", get(|| async { "ok" }))
        // Menggabungkan rute-rute otentikasi dari `routes/auth.rs`
        .nest("/api/auth", routes::auth::auth_routes())
        .nest("/api/user", routes::user::user_routes())
        // Swagger UI & OpenAPI JSON
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        // // Menambahkan rute lain yang memerlukan autentikasi
        // .route("/api/protected", get(handlers::user::protected_handler))
        // Menyisipkan pool database sebagai 'Extension' agar bisa diakses oleh handlers
        .layer(Extension(db_pool))
        // Menerapkan middleware CORS ke seluruh aplikasi
        .layer(cors);

    // 4. Mengatur Alamat Server
    let addr: SocketAddr = env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string())
        .parse()
        .expect("Invalid BIND_ADDRESS format");

    println!("🚀 Server listening on {}", addr);

    // 5. Menjalankan Server Axum
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
