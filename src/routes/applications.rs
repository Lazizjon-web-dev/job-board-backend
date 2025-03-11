use crate::handlers::applications::{
    CreateApplicationRequest, UpdateApplicationRequest, create_application, delete_application, get_application_by_id, get_applications, update_application,
};
use actix_web::{
    HttpRequest, HttpResponse,
    web::{Data, Json, Path, ServiceConfig, delete, get, post, put, scope},
};
use sqlx::PgPool;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/applications")
            .route("", get().to(get_applications))
            .route("/{id}", get().to(get_application_by_id))
            .route(
                "",
                post().to(
                    |req: HttpRequest,
                     form: Json<CreateApplicationRequest>,
                     pool: Data<PgPool>| async move {
                        let token = match req.headers().get("Authorization") {
                            Some(header) => header.to_str().unwrap_or("").to_string(),
                            None => return HttpResponse::Unauthorized().json("Missing token"),
                        };
                        create_application(pool, form, token).await
                    },
                ),
            )
            .route(
                "/{id}",
                put().to(
                    |req: HttpRequest,
                     form: Json<UpdateApplicationRequest>,
                     pool: Data<PgPool>,
                     application_id: Path<i32>| async move {
                        let token = match req.headers().get("Authorization") {
                            Some(header) => header.to_str().unwrap_or("").to_string(),
                            None => return HttpResponse::Unauthorized().json("Missing token"),
                        };
                        update_application(pool, application_id, form, token).await
                    },
                ),
            )
            .route(
                "/{id}",
                delete().to(|req: HttpRequest, pool: Data<PgPool>, application_id: Path<i32>| async move {
                    let token = match req.headers().get("Authorization") {
                        Some(header) => header.to_str().unwrap_or("").to_string(),
                        None => return HttpResponse::Unauthorized().json("Missing token"),
                    };
                    delete_application(pool, application_id, token).await
                })
            ),
    );
}
