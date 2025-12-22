use std::{borrow::Cow, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum CommonError {
    /// An error occurred during input/output operations.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// An error occurred during (de)serialization of JSON data.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// A required configuration value was not found.
    #[error("Configuration error: missing key '{key}'")]
    MissingConfig { key: Cow<'static, str> },

    /// An invalid input value was provided.
    #[error("Invalid input: {message}")]
    InvalidInput { message: Cow<'static, str> },

    /// A file was expected but not found at the specified path.
    #[error("File not found: {path}")]
    NotFound { path: PathBuf },

    // You can add transparent error wrapping for other libraries you use
    // #[error("Network request failed: {0}")]
    // Reqwest(#[from] reqwest::Error),
    /// A generic, unexpected internal error occurred.
    #[error("An unexpected internal error occurred: {0}")]
    InternalError(Cow<'static, str>),
}
