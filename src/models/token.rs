use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// Struct untuk menyimpan data klaim yang akan dimasukkan ke dalam JWT.
// 'sub' (subject) biasanya adalah ID pengguna.
// 'exp' (expiration) adalah waktu kedaluwarsa token.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthClaims {
    pub sub: Uuid,
    pub exp: i64,
}

// Struct untuk mengelola pasangan token (access token dan refresh token)
// yang akan dikirimkan ke klien setelah login berhasil.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}