use crate::error::CheckError;
use crate::traits::{Check, Transport};
use crate::utils::Result;

use async_trait::async_trait;
use prometheus::Histogram;
use serde_json::Value;

pub struct PrometheusCheck<'a> {
    uri: &'a str,
    query: &'a str,
    transport: Box<dyn Transport + Sync + Send + 'static>,
    metric: Histogram, // TODO define buckets
}

impl<'a> PrometheusCheck<'a> {
    pub fn new<T: Transport + Sync + Send + 'static>(
        name: &'a str,
        uri: &'a str,
        query: &'a str,
        transport: T,
    ) -> PrometheusCheck<'a> {
        PrometheusCheck {
            uri: uri,
            query: query,
            transport: Box::new(transport),
            metric: register_histogram!(name, name).unwrap(),
        }
    }
}

#[async_trait]
impl<'a> Check for PrometheusCheck<'a> {
    async fn update(&mut self) -> Result<()> {
        let response = self
            .transport
            .get(&format!("{}?query={}", self.uri, self.query))
            .await?;
        let val: Value = serde_json::from_str(&response)?;
        // TODO check if error while parsing
        match val["data"]["result"][0]["value"][1].as_str() {
            Some(v) => {
                self.metric.observe(v.parse()?);
            }
            _ => {
                return Err(Box::new(CheckError::new(
                    "prometheus response malformed: sample value should be a string",
                )));
            }
        }

        return Ok(());
    }
}
