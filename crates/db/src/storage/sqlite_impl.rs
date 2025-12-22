pub mod user_storage;

use crate::Result;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

pub struct StorageSqliteImpl {
    pool: SqlitePool,
}

impl StorageSqliteImpl {
    pub async fn new(target: String) -> Result<Self> {
        let target = target.replace("sqlite:", "");
        let options = SqliteConnectOptions::new().filename(target).create_if_missing(true);
        let pool = SqlitePoolOptions::new().max_connections(6).connect_with(options).await?;
        sqlx::migrate!("./migrations/sqlite").run(&pool).await?;
        Ok(StorageSqliteImpl { pool })
    }
}
