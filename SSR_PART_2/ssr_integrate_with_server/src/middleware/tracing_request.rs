#![cfg(feature = "ssr")]

use axum::http::Response;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use tower_http::trace::OnResponse;
use tracing::Span;

// Custom tracing implementation for response latency logging.
#[derive(Debug, Clone)]
pub struct LatencyOnResponse;

pub struct Latency(Duration);

impl<B> OnResponse<B> for LatencyOnResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, _span: &Span) {
        tracing::info!(
            latency = %Latency(latency),
            status = %response.status().as_u16(),
            "finished processing request."
        )
    }
}

impl Display for Latency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            write!(f, "{} ms", self.0.as_millis())
        } else {
            write!(f, "{} us", self.0.as_micros())
        }
    }
}
