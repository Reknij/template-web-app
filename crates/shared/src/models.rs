use std::cmp;

pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    page: i64,
    size: i64,
}

impl Pagination {
    pub fn new(page: i64, size: i64) -> Self {
        Self { page, size }
    }

    pub fn unlimited() -> Self {
        Self { page: -1, size: -1 }
    }

    pub fn is_unlimited(&self) -> bool {
        self.page == -1 && self.size == -1
    }

    /// Get safety page and size. (page, size)
    pub fn get_safety(&self) -> (i64, i64) {
        (cmp::max(self.page, 1), cmp::max(self.size, 1))
    }
}
