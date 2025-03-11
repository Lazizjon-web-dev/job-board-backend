use crate::models::application::Application;
use crate::utils::auth::get_user_from_token;
use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateApplicationRequest {
    pub job_id: i32,
    pub message: String,
}

#[derive(Deserialize)]
pub struct UpdateApplicationRequest {
    pub message: String,
}

pub async fn create_application(
    pool: Data<PgPool>,
    form: Json<CreateApplicationRequest>,
    token: String,
) -> HttpResponse {
    let user = match get_user_from_token(&pool, &token).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    match Application::create(&pool, user.id, form.job_id, &form.message).await {
        Ok(application) => HttpResponse::Ok().json(application),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create application"),
    }
}

pub async fn get_application_by_id(pool: Data<PgPool>, application_id: Path<i32>) -> HttpResponse {
    match Application::find_by_id(&pool, application_id.into_inner()).await {
        Ok(application) => HttpResponse::Ok().json(application),
        Err(_) => HttpResponse::NotFound().json("Application not found"),
    }
}

pub async fn get_applications(pool: Data<PgPool>) -> HttpResponse {
    match Application::find_all(&pool).await {
        Ok(applications) => HttpResponse::Ok().json(applications),
        Err(_) => HttpResponse::InternalServerError().json("Failed to get applications"),
    }
}

pub async fn update_application(
    pool: Data<PgPool>,
    application_id: Path<i32>,
    form: Json<UpdateApplicationRequest>,
    token: String,
) -> HttpResponse {
    let user = match get_user_from_token(&pool, &token).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    //TODO: Add check for whether user has permission to update

    match Application::update(&pool, application_id.into_inner(), &form.message).await {
        Ok(application) => HttpResponse::Ok().json(application),
        Err(_) => HttpResponse::InternalServerError().json("Failed to update application"),
    }
}

pub async fn delete_application(
    pool: Data<PgPool>,
    application_id: Path<i32>,
    token: String,
) -> HttpResponse {
    let user = match get_user_from_token(&pool, &token).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    //TODO: Add check for whether user has permission to delete

    match Application::delete(&pool, application_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Application deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete application"),
    }
}
