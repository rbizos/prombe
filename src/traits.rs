use crate::utils::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Check {
    async fn update(&mut self) -> Result<()>;
}

// Transport trait is mainly used to have a pattern for moking
#[async_trait]
pub trait Transport {
    async fn get(&self, uri: &str) -> Result<String>;
}
