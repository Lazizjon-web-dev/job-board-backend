use crate::models::user::User;
use crate::utils::jwt::validate_token;
use actix_web::HttpResponse;
use sqlx::PgPool;
pub async fn get_user_from_token(pool: &PgPool, token: &str) -> Result<User, HttpResponse> {
    let email = match validate_token(token) {
        Ok(claims) => claims.sub,
        Err(_) => return Err(HttpResponse::Unauthorized().json("Invalid token")),
    };
    match User::find_by_email(pool, &email).await {
        Ok(user) => Ok(user),
        Err(_) => Err(HttpResponse::NotFound().json("User not found")),
    }
}
