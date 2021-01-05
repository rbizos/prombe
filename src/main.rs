mod check;
mod clock;
mod error;
mod http;
mod metrics;
#[cfg(test)]
mod tests_check;
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

    metrics::setup_core_metrics(10);

    for mut c in check::get_prombe_checks() {
        let mut rec = cl.receiver();
        tokio::spawn(async move {
            loop {
                rec.recv().await.unwrap();
                match c.update().await {
                    Ok(_) => (),
                    Err(e) => println!("error: {}", e),
                }
            }
        });
    }

    let metrics_handler = warp::path!("metrics").map(|| metrics::gather_core_metrics());

    tokio::join!(
        cl.run(),
        warp::serve(metrics_handler).run(([127, 0, 0, 1], 3030))
    );
    Ok(())
}
