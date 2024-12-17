use hyper::{header::CONTENT_TYPE, Body, Request, Response};
use prometheus::{
    register_int_counter, Counter, Encoder, Gauge, HistogramVec, IntCounter, TextEncoder,
};
use std::cell::Cell;

use lazy_static::lazy_static;
use prometheus::{labels, opts, register_counter, register_gauge, register_histogram_vec};

thread_local! {
    pub static IGNORED_BY_PACKET_FILTER_PKT: Cell<u64> = Cell::new(13);
    pub static IGNORED_BY_PACKET_FILTER_BYTE: Cell<u64> = Cell::new(17);
}

pub fn update_thread_local_stats() {
    if IGNORED_BY_PACKET_FILTER_PKT.get() != 0 {
        IGNORED_BY_PACKET_FILTER_PKT_AGG.inc_by(IGNORED_BY_PACKET_FILTER_PKT.get());
        IGNORED_BY_PACKET_FILTER_PKT.set(0);
    }
    if IGNORED_BY_PACKET_FILTER_BYTE.get() != 0 {
        IGNORED_BY_PACKET_FILTER_BYTE_AGG.inc_by(IGNORED_BY_PACKET_FILTER_BYTE.get());
        IGNORED_BY_PACKET_FILTER_BYTE.set(0);
    }
}

lazy_static! {
    static ref IGNORED_BY_PACKET_FILTER_PKT_AGG: IntCounter = register_int_counter!(opts!(
        "ignored_by_packet_filter_pkt",
        "Number of packets ignored by packet filter.",
    ))
    .unwrap();
    static ref IGNORED_BY_PACKET_FILTER_BYTE_AGG: IntCounter = register_int_counter!(opts!(
        "ignored_by_packet_filter_byte",
        "Number of bytes ignored by packet filter.",
    ))
    .unwrap();
    static ref HTTP_COUNTER: Counter = register_counter!(opts!(
        "example_http_requests_total",
        "Number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(opts!(
        "example_http_response_size_bytes",
        "The HTTP response sizes in bytes.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "example_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap();
}

pub async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();

    HTTP_COUNTER.inc();
    let timer = HTTP_REQ_HISTOGRAM.with_label_values(&["all"]).start_timer();

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    HTTP_BODY_GAUGE.set(buffer.len() as f64);

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();

    timer.observe_duration();

    Ok(response)
}
