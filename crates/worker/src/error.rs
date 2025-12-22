use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CommonError(#[from] sys_core::error::CommonError),
    #[error(transparent)]
    StorageError(#[from] db::error::Error),
    #[error("Notify error: {0}")]
    NotifyError(Cow<'static, str>),
}
