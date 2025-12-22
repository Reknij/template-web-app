use service::{CommonService, CoreService};
use sys_core::models::user::UserSummary;

#[derive(Debug, Clone)]
pub struct AppState {
    pub com: CommonService,
}

impl AppState {
    /// Shortcut for `self.com.core(user)``
    pub fn core(&self, user: Option<UserSummary>) -> CoreService {
        self.com.core(user)
    }
}
