#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Common(#[from] shared::error::CommonError),
    #[error("Sqlx Error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Sqlx Migration Error: {0}")]
    SqlxMigrationError(#[from] sqlx::migrate::MigrateError),
}
