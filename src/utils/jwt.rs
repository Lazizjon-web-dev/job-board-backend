use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (e.g., user email)
    pub exp: usize,  // Expiration time
}

pub fn create_token(email: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Invalid timestamp")
        .timestamp();
    let claims = Claims {
        sub: email.to_string(),
        exp: expiration as usize,
    };
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(
        &Header::default,
        &claims,
        &EncodingKey::from_secret(&secret_key),
    )
    .unwrap()
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&secret_key),
        &Validation::default,
    )
    .map(|data| data.claims)
}
