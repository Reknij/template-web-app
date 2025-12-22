use crate::{
    Result,
    storage::{UserStorage, sqlite_impl::StorageSqliteImpl},
};
use async_trait::async_trait;
use sys_core::models::{
    Pagination,
    user::{UserDetail, UserDetailToAddOrUpdate, UserType},
};
use uuid::Uuid;

#[async_trait]
impl UserStorage for StorageSqliteImpl {
    async fn exists_user_type(&self, user_type: UserType) -> Result<bool> {
        let row = sqlx::query(r#"SELECT 1 FROM users WHERE is_deleted = FALSE AND user_type = ? LIMIT 1"#)
            .bind(user_type)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.is_some())
    }

    /// Inserts a new user record into the 'users' table and returns the new ID.
    async fn add_user(&self, user_type: UserType, detail: UserDetailToAddOrUpdate) -> Result<Uuid> {
        let id = Uuid::now_v7();
        let query = sqlx::query(
            r#"
            INSERT INTO users (id, alias, username, password, email, user_type)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(detail.alias)
        .bind(detail.username)
        .bind(detail.password)
        .bind(detail.email)
        .bind(user_type);

        query.execute(&self.pool).await?;
        Ok(id)
    }

    /// Deletes a user record by ID from the 'users' table.
    async fn remove_user(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE users SET is_deleted = TRUE
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Fetches a single user record by ID from the 'users' table.
    async fn get_user(&self, id: Uuid) -> Result<Option<UserDetail>> {
        // NOTE: SELECT fields MUST match the UserDetail struct fields exactly
        let detail = sqlx::query_as::<_, UserDetail>(
            r#"
            SELECT id, user_type, alias, username, password, email, created_at, updated_at
            FROM users
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(detail)
    }

    async fn get_user_by_validate(&self, username: &str, password: &str) -> Result<Option<UserDetail>> {
        // NOTE: SELECT fields MUST match the UserDetail struct fields exactly
        let detail = sqlx::query_as::<_, UserDetail>(
            r#"
            SELECT id, user_type, alias, username, password, email, created_at, updated_at
            FROM users
            WHERE username = ? AND password = ? AND is_deleted = 0
            "#,
        )
        .bind(username)
        .bind(password)
        .fetch_optional(&self.pool)
        .await?;

        Ok(detail)
    }

    /// Fetches a paginated list of user records from the 'users' table.
    async fn get_user_list(&self, pagination: Pagination) -> Result<Vec<UserDetail>> {
        let (page, size) = if pagination.is_unlimited() { (0, i64::MAX) } else { pagination.get_safety() };

        // NOTE: SELECT fields MUST match the UserDetail struct fields exactly
        let list = sqlx::query_as::<_, UserDetail>(
            r#"
            SELECT id, user_type, alias, username, password, email, created_at, updated_at
            FROM users
            WHERE is_deleted = 0
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(size)
        .bind(page - 1)
        .fetch_all(&self.pool)
        .await?;

        Ok(list)
    }

    /// Updates an existing user record in the 'users' table by ID.
    async fn update_user(&self, id: Uuid, detail: UserDetailToAddOrUpdate) -> Result<bool> {
        // Update all fields from UserDetailToAddOrUpdate
        let result = sqlx::query(
            r#"
            UPDATE users
            SET alias = ?, username = ?, password = ?, email = ?
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(detail.alias)
        .bind(detail.username)
        .bind(detail.password)
        .bind(detail.email)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
