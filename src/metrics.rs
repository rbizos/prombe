use prometheus::{gather, Encoder, Gauge, TextEncoder};
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    pub static ref SCRAPE_TIME: Gauge = register_gauge!(
        "prombe_scrape_time_timestamp",
        "Number of high fives received"
    )
    .unwrap();
}

pub fn gather_core_metrics() -> String {
    SCRAPE_TIME.set(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64(),
    );
    // uwrapped because this should never return Err
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}
