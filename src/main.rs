mod check;
mod clock;
mod error;
mod http;
mod metrics;
mod traits;
mod utils;

// traits
use crate::traits::Check;
use warp::Filter;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

#[tokio::main]
async fn main() -> utils::Result<()> {
    let mut cl = clock::Clock::new(10);

    let mut rec = cl.receiver();
    let mut ingestion_delay = check::PrometheusCheck::new(
        &"ingestion_delay",
        &"http://localhost:9090/api/v1/query",
        &"time() - max_over_time(prombe_scrape_time_timestamp[1h]) ",
        http::HttpTransport {},
    );
    tokio::spawn(async move {
        loop {
            rec.recv().await.unwrap();
            match ingestion_delay.update().await {
                Ok(_) => (),
                Err(e) => println!("error: {}", e),
            }
        }
    });

    let metrics_handler = warp::path!("metrics").map(|| metrics::gather_core_metrics());

    tokio::select! {
        _ = cl.run() => {}
        _  = warp::serve(metrics_handler).run(([127, 0, 0, 1], 3030)) => {}
    }

    Ok(())
}
