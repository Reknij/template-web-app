use crate::{CoreService, Result, preprocess::Preprocess};
use async_trait::async_trait;
use shared::models::{
    Pagination,
    user::{UserDetail, UserDetailToAddOrUpdate, UserType},
};
use uuid::Uuid;

#[async_trait]
pub trait UserExt {
    async fn add_user(&self, detail: UserDetailToAddOrUpdate) -> Result<Uuid>;
    async fn remove_user(&self, id: Uuid) -> Result<bool>;
    async fn update_user(&self, id: Uuid, detail: UserDetailToAddOrUpdate) -> Result<bool>;
    async fn get_user(&self, id: Uuid) -> Result<Option<UserDetail>>;
    async fn get_user_by_validate(&self, username: &str, password: &str) -> Result<Option<UserDetail>>;
    async fn get_user_list(&self, pagination: Pagination) -> Result<Vec<UserDetail>>;
}

#[async_trait]
impl UserExt for CoreService {
    async fn add_user(&self, mut detail: UserDetailToAddOrUpdate) -> Result<Uuid> {
        detail.process().await?;
        let user_type = if self.storage.exists_user_type(UserType::Admin).await? {
            UserType::Regular
        } else {
            UserType::Admin
        };
        Ok(self.storage.add_user(user_type, detail).await?)
    }
    async fn remove_user(&self, id: Uuid) -> Result<bool> {
        self.only_admin()?;
        Ok(self.storage.remove_user(id).await?)
    }
    async fn update_user(&self, id: Uuid, mut detail: UserDetailToAddOrUpdate) -> Result<bool> {
        self.only_admin_or_user(id)?;
        detail.process().await?;
        Ok(self.storage.update_user(id, detail).await?)
    }
    async fn get_user(&self, id: Uuid) -> Result<Option<UserDetail>> {
        self.only_admin_or_user(id)?;
        Ok(self.storage.get_user(id).await?)
    }
    async fn get_user_by_validate(&self, username: &str, password: &str) -> Result<Option<UserDetail>> {
        Ok(self.storage.get_user_by_validate(username, password).await?)
    }
    async fn get_user_list(&self, pagination: Pagination) -> Result<Vec<UserDetail>> {
        self.only_admin()?;
        Ok(self.storage.get_user_list(pagination).await?)
    }
}
