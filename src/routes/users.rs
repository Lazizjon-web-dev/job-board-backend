use crate::handlers::users::{get_applications_of_user, get_jobs_of_user, get_user_by_id};
use actix_web::web::{ServiceConfig, get, scope};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/users")
            .route("/{id}", get().to(get_user_by_id))
            .route("/{id}/jobs", get().to(get_jobs_of_user))
            .route("/{id}/applications", get().to(get_applications_of_user)),
    );
}
