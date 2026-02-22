#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Common(#[from] shared::error::CommonError),
    #[error("Storage Error: {0}")]
    StorageError(#[from] db::Error),
    #[error("Auth Error: {0}")]
    AuthError(&'static str),
    #[error("Format Error: {0}")]
    FormatError(&'static str),
}
