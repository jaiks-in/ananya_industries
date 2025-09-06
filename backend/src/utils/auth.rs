use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // user email
    pub role: String,  // "admin" or "user"
    pub exp: usize,    // expiration
}

// Get secret from .env
fn get_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "change_me_in_production".to_string())
}

// Generate JWT token
pub fn generate_token(email: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap()
        + Duration::from_secs(60 * 60); // 1 hour

    let claims = Claims {
        sub: email.to_string(),
        role: role.to_string(),
        exp: expiration.as_secs() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(get_secret().as_bytes()))
}

// Verify JWT token
pub fn verify_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(get_secret().as_bytes()), &Validation::default())
}
