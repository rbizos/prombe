use crate::traits::{Check, Transport};
use crate::utils::Result;

use async_trait::async_trait;

pub struct PrometheusCheck<'a> {
    uri: &'a str,
    query: &'a str,
    transport: Box<dyn Transport + Sync + Send + 'static>,
}

impl<'a> PrometheusCheck<'a> {
    pub fn new<T: Transport + Sync + Send + 'static>(
        uri: &'a str,
        query: &'a str,
        transport: T,
    ) -> PrometheusCheck<'a> {
        PrometheusCheck {
            uri: uri,
            query: query,
            transport: Box::new(transport),
        }
    }
}

#[async_trait]
impl<'a> Check for PrometheusCheck<'a> {
    async fn update(&mut self) -> Result<()> {
        self.transport.get(self.uri).await?;
        println!("{:?}", self.query);
        return Ok(());
    }
}
