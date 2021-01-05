use prometheus::{gather, Encoder, Gauge, TextEncoder};
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    pub static ref SCRAPE_TIME: Gauge = register_gauge!(
        "prombe_scrape_time_timestamp",
        "Current time as UNIX timestamp"
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

pub fn setup_core_metrics(nb: u8) {
    // this should panic if error as running without it is pointless
    let gauge_vec =
        register_gauge_vec!("prombe_point_constant", "constant 1", &["number"]).unwrap();

    for n in 0..nb {
        gauge_vec.with_label_values(&[&n.to_string()]).set(1.0);
    }
    register_gauge!(
        "prombe_point_total",
        "expected sum of all prombe_point_constant"
    )
    .unwrap()
    .set(nb as f64);
}
