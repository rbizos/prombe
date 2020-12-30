use crate::traits::{Check, Transport};
use crate::utils::Result;

use async_trait::async_trait;
use serde_json::Value;

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
        let response = self.transport.get(self.uri).await?;
        let v: Value = serde_json::from_str(&response)?;
        match v.get("data") {
            Some(da) => println!("{:?}", da),
            None => println!("mutmut"),
        }

        return Ok(());
    }
}
