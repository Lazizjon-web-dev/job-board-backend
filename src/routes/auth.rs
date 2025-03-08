use crate::handlers::auth::{login, register};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("login", web::post().to(login))
            .route("register", web::post().to(register)),
    );
}