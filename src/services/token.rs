use crate::models::token::AuthClaims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
// remove unused serde import (derive is on model types)
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::env;

// Uses `AuthClaims` from `crate::models::token`

pub fn generate_access_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();

    let access_token_expiry = env::var("JWT_ACCESS_TOKEN_EXPIRY").expect("JWT_ACCESS_TOKEN_EXPIRY must be set");

    // Parse expiry string to chrono::Duration
    let duration = parse_duration(&access_token_expiry)
        .unwrap_or_else(|_| panic!("Invalid JWT_ACCESS_TOKEN_EXPIRY format: {}", access_token_expiry));
    let expiration = now + duration;

    let claims = AuthClaims {
        sub: user_id,
        exp: expiration.timestamp(),
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn generate_refresh_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    
    let refresh_token_expiry = env::var("JWT_REFRESH_TOKEN_EXPIRY").expect("JWT_REFRESH_TOKEN_EXPIRY must be set");
    let duration = parse_duration(&refresh_token_expiry)
        .unwrap_or_else(|_| panic!("Invalid JWT_REFRESH_TOKEN_EXPIRY format: {}", refresh_token_expiry));

    let expiration = now + duration;

    let claims = AuthClaims {
        sub: user_id,
        exp: expiration.timestamp(),
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn validate_access_token(token: &str) -> Result<AuthClaims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn validate_refresh_token(token: &str) -> Result<AuthClaims, jsonwebtoken::errors::Error> {
    // Currently refresh tokens share the same signing/validation rules.
    // Split into different validation if you change headers/claims later.
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

fn parse_duration(s: &str) -> Result<Duration, &'static str> {
    let s = s.trim().to_lowercase();
    if let Some(stripped) = s.strip_suffix('m') {
        let minutes: i64 = stripped.parse().map_err(|_| "Invalid minutes")?;
        Ok(Duration::minutes(minutes))
    } else if let Some(stripped) = s.strip_suffix('h') {
        let hours: i64 = stripped.parse().map_err(|_| "Invalid hours")?;
        Ok(Duration::hours(hours))
    } else if let Some(stripped) = s.strip_suffix('d') {
        let days: i64 = stripped.parse().map_err(|_| "Invalid days")?;
        Ok(Duration::days(days))
    } else {
        Err("Unknown duration format")
    }
}

// Uses `TokenPair` from `crate::models::token`