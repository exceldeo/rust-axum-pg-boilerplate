use crate::models::user::{NewUser, User};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn insert_user(
    pool: &Pool<Postgres>,
    new_user: &NewUser,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password_hash, created_at, updated_at
        "#,
        new_user.username,
        new_user.email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email(
    pool: &Pool<Postgres>,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_id(
    pool: &Pool<Postgres>,
    user_id: Uuid,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

