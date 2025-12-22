use axum::response::IntoResponse;
use serde::Serialize;
use tracing::error;

pub type Result<T = serde_json::Value> = std::result::Result<ApiResult<T>, ApiResult<T>>;

#[derive(Debug, Serialize)]
pub struct ApiResult<T>
where
    T: Serialize,
{
    status: ApiStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ApiResultError>,
}

#[derive(Debug, Serialize)]
pub enum ApiStatus {
    Ok,
    Fail,
    Err,
}

#[derive(Debug, Serialize)]
pub struct ApiResultError {
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,
    code: ErrorCode,
}

#[derive(Debug, Serialize)]
#[repr(u16)]
pub enum ErrorCode {
    CommonError,
    StorageError,
    AuthError,
    FlexiError,
    JWTError,
    InternalError,
    FormatError,
}

impl<T> IntoResponse for ApiResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl<T> From<service::Error> for ApiResult<T>
where
    T: Serialize,
{
    fn from(value: service::Error) -> Self {
        let (msg, code) = match value {
            service::Error::Common(common_error) => (common_error.to_string(), ErrorCode::CommonError),
            service::Error::StorageError(error) => (error.to_string(), ErrorCode::StorageError),
            service::Error::AuthError(error) => (error.to_string(), ErrorCode::AuthError),
            service::Error::FormatError(error) => (error.to_string(), ErrorCode::FormatError),
        };
        error!("Service error ({code:?}): {msg}");
        Self {
            status: ApiStatus::Err,
            data: None,
            error: Some(ApiResultError { msg: Some(msg), code }),
        }
    }
}

impl<T> From<jsonwebtoken::errors::Error> for ApiResult<T>
where
    T: Serialize,
{
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        let (msg, code) = (value.to_string(), ErrorCode::JWTError);
        Self {
            status: ApiStatus::Err,
            data: None,
            error: Some(ApiResultError { msg: Some(msg), code }),
        }
    }
}

impl<T> ApiResult<T>
where
    T: Serialize,
{
    pub fn ok(data: T) -> std::result::Result<Self, Self> {
        Ok(Self {
            status: ApiStatus::Ok,
            data: Some(data),
            error: None,
        })
    }
    pub fn fail(data: T) -> std::result::Result<Self, Self> {
        Ok(Self {
            status: ApiStatus::Fail,
            data: Some(data),
            error: None,
        })
    }
    pub fn error(content: &str) -> std::result::Result<Self, Self> {
        Ok(Self {
            status: ApiStatus::Err,
            data: None,
            error: Some(ApiResultError {
                msg: Some(content.to_string()),
                code: ErrorCode::InternalError,
            }),
        })
    }
}
