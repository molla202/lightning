use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct Metrics {
    pub cpu_time: Duration,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}