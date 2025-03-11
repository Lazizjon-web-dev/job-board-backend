use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool, query, query_as};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Application {
    pub id: i32,
    pub job_id: i32,
    pub user_id: i32,
    pub message: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Application {
    pub async fn create(
        pool: &PgPool,
        user_id: i32,
        job_id: i32,
        message: &str,
    ) -> Result<Self, Error> {
        let application = query_as!(
            Application,
            r#"
                INSERT INTO applications (job_id, user_id, message)
                VALUES ($1, $2, $3)
                RETURNING id, job_id, user_id, message, status, created_at, updated_at
            "#,
            job_id,
            user_id,
            message,
        )
        .fetch_one(pool)
        .await?;

        Ok(application)
    }

    pub async fn find_by_id(pool: &PgPool, application_id: i32) -> Result<Self, Error> {
        let application = query_as!(
            Application,
            r#"
                SELECT id, job_id, user_id, message, status, created_at, updated_at
                FROM applications
                WHERE id = $1
            "#,
            application_id
        )
        .fetch_one(pool)
        .await?;

        Ok(application)
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<Self>, Error> {
        let applications = query_as!(
            Application,
            r#"
                SELECT id, job_id, user_id, message, status, created_at, updated_at
                FROM applications
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(applications)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, Error> {
        let applications = query_as!(
            Application,
            r#"
                SELECT id, job_id, user_id, message, status, created_at, updated_at
                FROM applications
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(applications)
    }

    pub async fn update_status(
        pool: &PgPool,
        application_id: i32,
        status: &str,
    ) -> Result<Self, Error> {
        let application = query_as!(
            Application,
            r#"
                UPDATE applications
                SET status = $1
                WHERE id = $2
                RETURNING id, job_id, user_id, message, status, created_at, updated_at
            "#,
            status,
            application_id
        )
        .fetch_one(pool)
        .await?;

        Ok(application)
    }

    pub async fn update(
        pool: &PgPool,
        application_id: i32,
        message: &str,
    ) -> Result<Self, Error> {
        let application = query_as!(
            Application,
            r#"
                UPDATE applications
                SET message = $1
                WHERE id = $2
                RETURNING id, job_id, user_id, message, status, created_at, updated_at
            "#,
            message,
            application_id,
        )
        .fetch_one(pool)
        .await?;

        Ok(application)
    }

    pub async fn delete(pool: &PgPool, application_id: i32) -> Result<(), Error> {
        query!(
            r#"
                DELETE FROM applications
                WHERE id = $1
            "#,
            application_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
