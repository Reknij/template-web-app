pub mod error;
pub mod filters;
pub mod db;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
