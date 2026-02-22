use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use jsonwebtoken::{DecodingKey, Validation, decode};
use shared::models::user::UserSummary;

use crate::app_state::AppState;

#[derive(Debug, Clone)]
pub struct CurrentUser(pub Option<UserSummary>);

impl FromRequestParts<AppState> for CurrentUser {
    // Keeping Rejection type standard
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth_header_value = parts.headers.get("Authorization").and_then(|h| h.to_str().ok());

        // Check if the Authorization header exists
        if let Some(auth_header) = auth_header_value {
            // 1. Strip the "Bearer " prefix
            let token = auth_header
                .strip_prefix("Bearer ")
                // If the token scheme is wrong, reject the request (UNAUTHORIZED)
                .ok_or((StatusCode::UNAUTHORIZED, "Invalid token scheme"))?;

            let validation = Validation::default();

            // 2. Decode and validate the token
            let decoding_key = DecodingKey::from_secret(state.com.config().security.auth_key.as_bytes());
            let token_data_result = decode::<UserSummary>(token, &decoding_key, &validation);

            match token_data_result {
                Ok(token_data) => {
                    // 3. Token is valid, map claims to UserSummary
                    let claims = token_data.claims;
                    // Return CurrentUser with Some(UserSummary)
                    Ok(CurrentUser(Some(claims)))
                }
                Err(_) => {
                    return Err((StatusCode::UNAUTHORIZED, "Invalid or Expired Token"));
                }
            }
        } else {
            // 5. No Authorization header present - this is the "optional" part
            Ok(CurrentUser(None))
        }
    }
}
