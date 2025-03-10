use crate::handlers::jobs::{
    CreateJobRequest, UpdateJobRequest, create_job, delete_job, get_job_by_id, get_jobs, update_job,
};
use actix_web::{
    HttpRequest, HttpResponse,
    web::{Data, Json, Path, ServiceConfig, delete, get, post, put, scope},
};
use sqlx::PgPool;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/jobs")
            .route("", get().to(get_jobs))
            .route("/{id}", get().to(get_job_by_id))
            .route(
                "",
                post().to(
                    |req: HttpRequest,
                     form: Json<CreateJobRequest>,
                     pool: Data<PgPool>| async move {
                        let token = match req.headers().get("Authorization") {
                            Some(header) => header.to_str().unwrap_or("").to_string(),
                            None => return HttpResponse::Unauthorized().json("Missing token"),
                        };
                        create_job(pool, form, token).await
                    },
                ),
            )
            .route(
                "/{id}",
                put().to(
                    |req: HttpRequest,
                     form: Json<UpdateJobRequest>,
                     pool: Data<PgPool>,
                     job_id: Path<i32>| async move {
                        let token = match req.headers().get("Authorization") {
                            Some(header) => header.to_str().unwrap_or("").to_string(),
                            None => return HttpResponse::Unauthorized().json("Missing token"),
                        };
                        update_job(pool, job_id, form, token).await
                    },
                ),
            )
            .route(
                "/{id}",
                delete().to(|req: HttpRequest, pool: Data<PgPool>, job_id: Path<i32>| async move {
                    let token = match req.headers().get("Authorization") {
                        Some(header) => header.to_str().unwrap_or("").to_string(),
                        None => return HttpResponse::Unauthorized().json("Missing token"),
                    };
                    delete_job(pool, job_id, token).await
                })
            ),
    );
}
