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

    for uri in vec!["http://localhost:9090/api/v1/query?query=prombe_scrape_time_timestamp"] {
        let mut rec = cl.receiver();
        let mut check = check::PrometheusCheck::new(&uri, &uri, http::HttpTransport {});
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
    let metrics_handler = warp::path!("metrics").map(|| metrics::gather_core_metrics());

    tokio::select! {
        socket = cl.run()=> {
            println!("Socket connected {:?}", socket);
        }
        msg = warp::serve(metrics_handler).run(([127, 0, 0, 1], 3030)) => {
            println!("received message first {:?}", msg);
        }
    }

    Ok(())
}
