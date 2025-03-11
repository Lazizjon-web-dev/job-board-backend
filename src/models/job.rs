use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool, query, query_as, types::Decimal};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub salary: Decimal,
    pub location: String,
    pub category: String,
    pub employer_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Job {
    pub async fn create(
        pool: &PgPool,
        user_id: i32,
        title: &str,
        description: &str,
        location: &str,
        salary: Decimal,
        category: &str,
    ) -> Result<Self, Error> {
        let job = query_as!(
            Job,
            r#"
                INSERT INTO jobs (title, description, location, salary, category, employer_id)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, title, description, location, salary, category, employer_id, created_at, updated_at
            "#,
            title,
            description,
            location,
            salary,
            category,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(job)
    }

    pub async fn find_by_id(pool: &PgPool, job_id: i32) -> Result<Self, Error> {
        let job = query_as!(
            Job,
            r#"
                SELECT id, title, description, location, salary, category, employer_id, created_at, updated_at
                FROM jobs
                WHERE id = $1
            "#,
            job_id
        )
        .fetch_one(pool)
        .await?;

        Ok(job)
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<Self>, Error> {
        let jobs = query_as!(
            Job,
            r#"
                SELECT id, title, description, location, salary, category, employer_id, created_at, updated_at
                FROM jobs
                WHERE employer_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(jobs)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, Error> {
        let jobs = query_as!(
            Job,
            r#"
                SELECT id, title, description, location, salary, category, employer_id, created_at, updated_at
                FROM jobs
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(jobs)
    }

    pub async fn update(
        pool: &PgPool,
        job_id: i32,
        title: &str,
        description: &str,
        location: &str,
        salary: Decimal,
        category: &str,
    ) -> Result<Self, Error> {
        let job = query_as!(
            Job,
            r#"
                UPDATE jobs
                SET title = $1,
                    description = $2,
                    location = $3,
                    salary = $4,
                    category = $5,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = $6
                RETURNING id, title, description, location, salary, category, employer_id, created_at, updated_at
            "#,
            title,
            description,
            location,
            salary,
            category,
            job_id
        )
        .fetch_one(pool)
        .await?;

        Ok(job)
    }

    pub async fn delete(pool: &PgPool, job_id: i32) -> Result<(), Error> {
        query!(
            r#"
                DELETE FROM jobs
                WHERE id = $1
            "#,
            job_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
