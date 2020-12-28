use crate::error::CheckError;
use crate::traits::Transport;
use crate::utils::Result;

use async_trait::async_trait;

use hyper::Client;
use hyper_tls::HttpsConnector;

pub struct HttpTransport {}

#[async_trait]
impl Transport for HttpTransport {
    async fn get(&self, uri: &str) -> Result<String> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let resp = client.get(uri.parse()?).await?;
        if resp.status().is_success() {
            return Ok(String::from("Hello, world!"));
        } else {
            return Err(Box::new(CheckError::new(&String::from(format!(
                "{}",
                resp.status()
            )))));
        }
    }
}

/*
timeout can be implemented with select and tokio::time::timeout or tokio::time::sleep
should return => io::ErrorKind::TimedOut,
*/
