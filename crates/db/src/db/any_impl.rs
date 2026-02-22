pub mod user_storage;

use std::sync::Arc;

use crate::db::FullDb;

pub struct AnyDbImpl {
    inner: Arc<dyn FullDb>,
}

impl AnyDbImpl {
    pub fn new(inner: impl FullDb + 'static) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}
