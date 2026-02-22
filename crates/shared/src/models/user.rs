use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Type)]
#[repr(u16)]
pub enum UserType {
    Admin,
    Regular,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSummary {
    pub id: Uuid,
    pub user_type: UserType,
    pub alias: String,
    pub username: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserMinInfo {
    pub id: Uuid,
    pub user_type: UserType,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserDetail {
    pub id: Uuid,
    pub user_type: UserType,
    pub alias: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserDetailToAddOrUpdate {
    pub alias: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl UserSummary {
    pub fn is_admin(&self) -> bool {
        matches!(self.user_type, UserType::Admin)
    }

    pub fn is_not_admin(&self) -> bool {
        !self.is_admin()
    }

    pub fn is_regular(&self) -> bool {
        matches!(self.user_type, UserType::Regular)
    }
}

impl From<UserMinInfo> for Uuid {
    fn from(value: UserMinInfo) -> Self {
        value.id
    }
}

impl From<UserSummary> for UserMinInfo {
    fn from(value: UserSummary) -> Self {
        Self {
            id: value.id,
            user_type: value.user_type,
        }
    }
}

impl From<&UserSummary> for UserMinInfo {
    fn from(value: &UserSummary) -> Self {
        Self {
            id: value.id,
            user_type: value.user_type,
        }
    }
}

impl From<UserDetail> for UserMinInfo {
    fn from(value: UserDetail) -> Self {
        Self {
            id: value.id,
            user_type: value.user_type,
        }
    }
}

impl From<&UserDetail> for UserMinInfo {
    fn from(value: &UserDetail) -> Self {
        Self {
            id: value.id,
            user_type: value.user_type,
        }
    }
}

impl UserMinInfo {
    pub fn is_admin(&self) -> bool {
        matches!(self.user_type, UserType::Admin)
    }

    pub fn is_not_admin(&self) -> bool {
        !self.is_admin()
    }

    pub fn is_regular(&self) -> bool {
        matches!(self.user_type, UserType::Regular)
    }
}
