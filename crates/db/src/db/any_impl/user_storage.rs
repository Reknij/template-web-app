use crate::{
    Result,
    db::{UserDb, any_impl::AnyDbImpl},
};
use async_trait::async_trait;
use shared::models::{
    Pagination,
    user::{UserDetail, UserDetailToAddOrUpdate, UserType},
};
use uuid::Uuid;

#[async_trait]
impl UserDb for AnyDbImpl {
    async fn exists_user_type(&self, user_type: UserType) -> Result<bool> {
        self.inner.exists_user_type(user_type).await
    }

    /// Inserts a new user record into the 'users' table and returns the new ID.
    async fn add_user(&self, user_type: UserType, detail: UserDetailToAddOrUpdate) -> Result<Uuid> {
        self.inner.add_user(user_type, detail).await
    }

    /// Deletes a user record by ID from the 'users' table.
    async fn remove_user(&self, id: Uuid) -> Result<bool> {
        self.inner.remove_user(id).await
    }

    /// Fetches a single user record by ID from the 'users' table.
    async fn get_user(&self, id: Uuid) -> Result<Option<UserDetail>> {
        self.inner.get_user(id).await
    }

    async fn get_user_by_validate(&self, username: &str, password: &str) -> Result<Option<UserDetail>> {
        self.inner.get_user_by_validate(username, password).await
    }

    /// Fetches a paginated list of user records from the 'users' table.
    async fn get_user_list(&self, pagination: Pagination) -> Result<Vec<UserDetail>> {
        self.inner.get_user_list(pagination).await
    }

    /// Updates an existing user record in the 'users' table by ID.
    async fn update_user(&self, id: Uuid, detail: UserDetailToAddOrUpdate) -> Result<bool> {
        self.inner.update_user(id, detail).await
    }
}
