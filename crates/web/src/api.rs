mod api_result;
mod login_auth;

use crate::{
    api::{
        api_result::ApiResult,
        login_auth::{LoginAuthRequest, LoginAuthResponse, LoginFailure, ReasonField},
    },
    app_state::AppState,
    jwt,
    models::current_user::CurrentUser,
};
use api_result::Result;
use axum::{
    Json, Router, debug_handler,
    extract::{Path, Query, State},
    routing,
};

use service::service_ext::user_ext::UserExt;
use sys_core::models::{
    Pagination,
    user::{UserDetail, UserDetailToAddOrUpdate},
};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", routing::post(add_user).get(get_user_list))
        .route("/users/{id}", routing::delete(remove_user).get(get_user).put(update_user))
        .route("/login", routing::post(login))
}

#[debug_handler]
async fn add_user(State(app): State<AppState>, CurrentUser(user): CurrentUser, Json(detail): Json<UserDetailToAddOrUpdate>) -> Result<Uuid> {
    ApiResult::ok(app.core(user).add_user(detail).await?)
}

#[debug_handler]
async fn remove_user(State(app): State<AppState>, CurrentUser(user): CurrentUser, Path(id): Path<Uuid>) -> Result<bool> {
    ApiResult::ok(app.core(user).remove_user(id).await?)
}

#[debug_handler]
async fn get_user(State(app): State<AppState>, CurrentUser(user): CurrentUser, Path(id): Path<Uuid>) -> Result<Option<UserDetail>> {
    ApiResult::ok(app.core(user).get_user(id).await?)
}

#[debug_handler]
async fn get_user_list(State(app): State<AppState>, CurrentUser(user): CurrentUser, Query(pagination): Query<Pagination>) -> Result<Vec<UserDetail>> {
    ApiResult::ok(app.core(user).get_user_list(pagination).await?)
}

#[debug_handler]
async fn update_user(State(app): State<AppState>, CurrentUser(user): CurrentUser, Path(id): Path<Uuid>, Json(detail): Json<UserDetailToAddOrUpdate>) -> Result<bool> {
    ApiResult::ok(app.core(user).update_user(id, detail).await?)
}

#[debug_handler]
async fn login(State(app): State<AppState>, Json(credentials): Json<LoginAuthRequest>) -> Result {
    let user = app.core(None).get_user_by_validate(&credentials.username, &credentials.password).await?;
    if let Some(user) = user {
        let token = jwt::generate_token(&user, app.com.config().security.auth_key.as_str())?;
        let response = LoginAuthResponse { user_id: user.id, token };
        if let Ok(response) = serde_json::to_value(&response) {
            ApiResult::ok(response)
        } else {
            ApiResult::error("Failed to serialize login response.")
        }
    } else {
        let response = LoginFailure {
            reasons: vec![ReasonField {
                field: "username/password".to_string(),
                message: "Invalid username or password.".to_string(),
            }],
        };

        if let Ok(response) = serde_json::to_value(&response) {
            ApiResult::fail(response)
        } else {
            ApiResult::error("Failed to serialize login response.")
        }
    }
}
