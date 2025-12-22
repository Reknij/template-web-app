use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sys_core::models::user::{UserDetail, UserSummary};

pub fn generate_token(user: &UserDetail, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // Set expiration to 1 hour from now
    let expiration = Utc::now() + Duration::hours(1);

    let claims = UserSummary {
        id: user.id,
        user_type: user.user_type.clone(),
        alias: user.alias.clone(),
        username: user.username.clone(),
        exp: expiration.timestamp() as usize, // Convert DateTime to Unix timestamp
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))?;

    Ok(token)
}
