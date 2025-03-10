use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool, query_as};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<Self, Error> {
        let user = query_as!(
            User,
            r#"
                INSERT INTO users (username, email, password_hash)
                VALUES ($1, $2, $3)
                RETURNING id, username, email, password_hash, created_at
            "#,
            username,
            email,
            password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Self, Error> {
        let user = query_as!(
            User,
            r#"
                SELECT id, username, email, password_hash, created_at
                FROM users
                WHERE email = $1
            "#,
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
