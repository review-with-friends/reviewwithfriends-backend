use opentelemetry_api::trace;
use std::error::Error;

pub fn add_error_span(error: &dyn Error) {
    trace::get_active_span(|span| {
        span.record_error(error);
    });
}
