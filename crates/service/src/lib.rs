pub mod error;
mod preprocess;
pub mod service_ext;
use db::db::FullDb;
pub use error::Error;
use shared::{config::Config, models::user::UserSummary};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

use std::{fmt::Debug, sync::Arc};

#[derive(Clone)]
pub struct CommonService {
    storage: Arc<dyn FullDb>,
    config: Arc<Config>,
}

#[derive(Clone)]
pub struct CoreService {
    storage: Arc<dyn FullDb>,

    /// Current logined user.
    user: Option<UserSummary>,
}

impl Debug for CommonService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoreService").field("storage", &"...").finish()
    }
}

impl Debug for CoreService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoreService").field("storage", &"...").field("user", &self.user).finish()
    }
}

impl CommonService {
    pub fn new(storage: Arc<dyn FullDb + 'static>, config: Config) -> Self {
        Self { storage, config: Arc::new(config) }
    }
    pub fn core(&self, user: Option<UserSummary>) -> CoreService {
        CoreService { storage: self.storage.clone(), user }
    }
    pub fn config(&self) -> Arc<Config> {
        self.config.clone()
    }
}

impl CoreService {
    pub fn try_get_current_user(&self) -> Result<&UserSummary> {
        if let Some(user) = self.user.as_ref() {
            Ok(user)
        } else {
            Err(crate::Error::AuthError("Please login to continue!"))
        }
    }
    pub fn only_admin(&self) -> Result<()> {
        if !self.user.as_ref().is_some_and(|user| user.is_admin()) {
            return Err(Error::AuthError("Only admin can do!"));
        }
        Ok(())
    }
    pub fn only_admin_or_user(&self, user_id: Uuid) -> Result<()> {
        if !self.user.as_ref().is_some_and(|user| user.is_admin() || user.id == user_id) {
            return Err(Error::AuthError("Only admin or user can do!"));
        }
        Ok(())
    }
}
