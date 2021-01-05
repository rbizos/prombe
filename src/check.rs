use crate::error::CheckError;
use crate::http;
use crate::traits::{Check, Transport};
use crate::utils::Result;

use async_trait::async_trait;
use prometheus::{exponential_buckets, Histogram};
use serde_json::Value;

pub struct PrometheusCheck<'a> {
    uri: &'a str,
    query: &'a str,
    transport: Box<dyn Transport + Sync + Send + 'static>,
    metric: Histogram, // TODO define buckets
                       // TODO failure history
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
            metric: register_histogram!(name, name, exponential_buckets(1.0, 2.0, 12).unwrap())
                .unwrap(),
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
        match &val["data"]["result"][0]["value"][1] {
            Value::String(value) => {
                self.metric.observe(value.parse()?);
            }
            _ => {
                return Err(Box::new(CheckError::new(
                    "error while parsing prometheus response",
                )));
            }
        };
        return Ok(());
    }
}

pub fn get_prombe_checks() -> Vec<PrometheusCheck<'static>> {
    let mut checks = Vec::new();

    let ingestion_delay = PrometheusCheck::new(
        &"ingestion_delay",
        &"http://localhost:9090/api/v1/query",
        &"time() - max_over_time(prombe_scrape_time_timestamp[1h])",
        http::HttpTransport {},
    );

    let point_loss = PrometheusCheck::new(
        &"point_loss",
        &"http://localhost:9090/api/v1/query",
        &"prombe_point_total - ignoring(instance, job) group_left sum(prombe_point_constant)",
        http::HttpTransport {},
    );
    checks.push(ingestion_delay);
    checks.push(point_loss);
    checks
}
