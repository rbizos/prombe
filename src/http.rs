use crate::traits::Transport;
use crate::utils::Result;

use async_trait::async_trait;

use reqwest::get;

pub struct HttpTransport {}

#[async_trait]
impl Transport for HttpTransport {
    async fn get(&self, uri: &str) -> Result<String> {
        match get(uri).await?.text().await {
            Ok(text) => Ok(text),
            Err(e) => Err(Box::new(e)),
        }
    }
}

/*
timeout can be implemented with select and tokio::time::timeout or tokio::time::sleep
should return => io::ErrorKind::TimedOut,
*/
