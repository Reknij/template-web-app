use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginAuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginAuthResponse {
    pub user_id: Uuid,
    pub token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginFailure {
    pub reasons: Vec<ReasonField>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReasonField {
    pub field: String,
    pub message: String,
}
