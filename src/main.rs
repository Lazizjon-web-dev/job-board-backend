use actix_cors::Cors;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};
use dotenv::dotenv;
use routes::*;
use sqlx::PgPool;
use std::{env, vec};

mod handlers;
mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(auth::config)
            .configure(users::config)
            .configure(jobs::config)
            .configure(applications::config)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
