use crate::models::user::{NewUser, User};
use crate::models::token::TokenPair;
use crate::services::token::{generate_access_token, generate_refresh_token};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::{Pool, Postgres};
use crate::repositories::user::{insert_user, find_user_by_email};

pub async fn register_user(
    pool: &Pool<Postgres>,
    new_user: &NewUser,
) -> Result<User, sqlx::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(new_user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = insert_user(pool, new_user, &password_hash).await?;

    Ok(user)
}

pub async fn authenticate_user(
    pool: &Pool<Postgres>,
    email: &str,
    password: &str,
) -> Result<TokenPair, String> {
    let user = find_user_by_email(pool, email)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "User not found".to_string())?;

    let parsed_hash =
        argon2::password_hash::PasswordHash::new(&user.password_hash).unwrap();
    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err("Invalid credentials".to_string());
    }

    let access_token = generate_access_token(user.id).map_err(|e| e.to_string())?;
    let refresh_token = generate_refresh_token(user.id).map_err(|e| e.to_string())?;

    println!("Generated tokens for user {}: access_token={}, refresh_token={}", user.id, access_token, refresh_token); // Debug print

    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}