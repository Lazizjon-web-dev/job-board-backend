use crate::models::job::Job;
use crate::utils::auth::get_user_from_token;
use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use serde::Deserialize;
use sqlx::{PgPool, types::Decimal};

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub title: String,
    pub description: String,
    pub location: String,
    pub salary: Decimal,
    pub category: String,
}

#[derive(Deserialize)]
pub struct UpdateJobRequest {
    pub title: String,
    pub description: String,
    pub location: String,
    pub salary: Decimal,
    pub category: String,
}

pub async fn create_job(
    pool: Data<PgPool>,
    form: Json<CreateJobRequest>,
    token: String,
) -> HttpResponse {
    let user = match get_user_from_token(&pool, &token).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    match Job::create(
        &pool,
        user.id,
        &form.title,
        &form.description,
        &form.location,
        form.salary,
        &form.category,
    )
    .await
    {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create job"),
    }
}

pub async fn get_job_by_id(pool: Data<PgPool>, job_id: Path<i32>) -> HttpResponse {
    match Job::find_by_id(&pool, job_id.into_inner()).await {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(_) => HttpResponse::NotFound().json("Job not found"),
    }
}

pub async fn get_jobs(pool: Data<PgPool>) -> HttpResponse {
    match Job::find_all(&pool).await {
        Ok(jobs) => HttpResponse::Ok().json(jobs),
        Err(_) => HttpResponse::InternalServerError().json("Failed to find jobs"),
    }
}

pub async fn update_job(
    pool: Data<PgPool>,
    job_id: Path<i32>,
    form: Json<UpdateJobRequest>,
    token: String,
) -> HttpResponse {
    let user = match get_user_from_token(&pool, &token).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    //TODO: Add check for whether user has permission to update

    match Job::update(
        &pool,
        job_id.into_inner(),
        &form.title,
        &form.description,
        &form.location,
        form.salary,
        &form.category,
    )
    .await
    {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(_) => HttpResponse::InternalServerError().json("Failed to update job"),
    }
}

pub async fn delete_job(pool: Data<PgPool>, job_id: Path<i32>, token: String) -> HttpResponse {
    let user = match get_user_from_token(&pool, &token).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    //TODO: Add check for whether user has permission to delete the job

    match Job::delete(&pool, job_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Job deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete job"),
    }
}
