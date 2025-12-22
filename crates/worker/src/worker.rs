use std::time::Duration;

use crate::Result;

use async_trait::async_trait;

#[async_trait]
pub trait Worker: Send + Sync {
    async fn loop_process(&self) -> Result<Duration>;
}
