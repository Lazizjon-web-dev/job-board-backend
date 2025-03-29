use crate::models::user::User;
use crate::models::job::Job;
use crate::models::application::Application;
use actix_web::{
    HttpResponse,
    web::{Data, Path},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
}

pub async fn get_user_by_id(pool: Data<PgPool>, user_id: Path<i32>) -> HttpResponse {
    match User::find_by_id(&pool, &user_id).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
        }),
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }
}

pub async fn get_jobs_of_user(pool: Data<PgPool>, user_id: Path<i32>) -> HttpResponse {
    match Job::find_by_user_id(&pool, &user_id).await {
        Ok(jobs) => HttpResponse::Ok().json(jobs),
        Err(_) => HttpResponse::NotFound().json("Jobs not found"),
    }
}

pub async fn get_applications_of_user(pool: Data<PgPool>, user_id: Path<i32>) -> HttpResponse {
    match Application::find_by_user_id(&pool, &user_id).await {
        Ok(applications) => HttpResponse::Ok().json(applications),
        Err(_) => HttpResponse::NotFound().json("Applications not found"),
    }
}   