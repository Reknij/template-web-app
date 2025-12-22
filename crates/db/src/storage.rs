pub mod sqlite_impl;

use crate::Result;
use async_trait::async_trait;
use sys_core::models::{
    Pagination,
    user::{UserDetail, UserDetailToAddOrUpdate, UserType},
};
use uuid::Uuid;

pub trait FullStorage: UserStorage {}
impl<T> FullStorage for T where T: UserStorage {}

#[async_trait]
pub trait UserStorage: Send + Sync {
    async fn add_user(&self, user_type: UserType, detail: UserDetailToAddOrUpdate) -> Result<Uuid>;
    async fn exists_user_type(&self, user_type: UserType) -> Result<bool>;
    async fn remove_user(&self, id: Uuid) -> Result<bool>;
    async fn get_user(&self, id: Uuid) -> Result<Option<UserDetail>>;
    async fn get_user_list(&self, pagination: Pagination) -> Result<Vec<UserDetail>>;
    async fn get_user_by_validate(&self, username: &str, password: &str) -> Result<Option<UserDetail>>;
    async fn update_user(&self, id: Uuid, detail: UserDetailToAddOrUpdate) -> Result<bool>;
}
