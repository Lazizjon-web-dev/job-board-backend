use crate::models::user::User;
use crate::utils::jwt::create_token;
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
}

pub async fn register(pool: Data<PgPool>, form: Json<RegisterRequest>) -> HttpResponse {
    let password_hash = hash(&form.password, DEFAULT_COST).unwrap();
    match User::create(
        &pool,
        &form.username,
        &form.email,
        &password_hash,
        &form.role,
    )
    .await
    {
        Ok(user) => {
            let token = create_token(&user.email);
            HttpResponse::Ok().json(AuthResponse { token })
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to register user"),
    }
}

pub async fn login(pool: Data<PgPool>, form: Json<LoginRequest>) -> HttpResponse {
    match User::find_by_email(&pool, &form.email).await {
        Ok(user) => {
            if verify(&form.password, &user.password_hash).unwrap() {
                let token = create_token(&user.email);

                HttpResponse::Ok().json(AuthResponse { token })
            } else {
                HttpResponse::Unauthorized().json("The password is incorrect")
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("The email is incorrect"),
    }
}
