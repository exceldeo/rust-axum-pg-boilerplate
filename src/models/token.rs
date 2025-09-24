use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// Struct untuk menyimpan data klaim yang akan dimasukkan ke dalam JWT.
// 'sub' (subject) biasanya adalah ID pengguna.
// 'exp' (expiration) adalah waktu kedaluwarsa token.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthClaims {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub sub: Uuid,
    #[schema(example = 1700000000)]
    pub exp: i64,
}

// Struct untuk mengelola pasangan token (access token dan refresh token)
// yang akan dikirimkan ke klien setelah login berhasil.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct TokenPair {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    #[schema(example = "dGhpcy1pcz1hLXJlZnJlc2gtdG9rZW4tZXhhbXBsZQ...")]
    pub refresh_token: String,
}
