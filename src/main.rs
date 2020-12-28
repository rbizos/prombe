mod clock;
mod error;
mod http;
mod prometheus;
mod traits;
mod utils;

use crate::traits::Check;

#[tokio::main]
async fn main() -> utils::Result<()> {
    // This is where we will setup our HTTP client requests.
    let mut cl = clock::Clock::new(10);
    for uri in vec!["http://localhost:9090/api/v1/query?query=prometheus_engine_queries"] {
        let mut rec = cl.receiver();
        let mut check = prometheus::PrometheusCheck::new(&uri, &uri, http::HttpTransport {});
        tokio::spawn(async move {
            loop {
                rec.recv().await.unwrap();
                match check.update().await {
                    Ok(_) => println!("ok"),
                    Err(e) => println!("error: {}", e),
                }
            }
        });
    }
    cl.run().await;
    Ok(())
}
