use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool, query, query_as};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        password_hash: &str,
        role: &str,
    ) -> Result<Self, Error> {
        let user = query_as!(
            User,
            r#"
                INSERT INTO users (username, email, password_hash, role)
                VALUES ($1, $2, $3, $4)
                RETURNING id, username, email, password_hash, role, created_at, updated_at
            "#,
            username,
            email,
            password_hash,
            role
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Self, Error> {
        let user = query_as!(
            User,
            r#"
                SELECT id, username, email, password_hash, role,  created_at, updated_at
                FROM users
                WHERE email = $1
            "#,
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
    pub async fn find_by_id(pool: &PgPool, user_id: &i32) -> Result<Self, Error> {
        let user = query_as!(
            User,
            r#"
                SELECT id, username, email, password_hash, role, created_at, updated_at
                FROM users
                WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
    pub async fn delete(pool: &PgPool, user_id: i32) -> Result<(), Error> {
        query!(
            r#"
                DELETE FROM users
                WHERE id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
