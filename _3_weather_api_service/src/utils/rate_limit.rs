use tokio::time::Instant;

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub time: Instant,
    pub count: u32
}

impl RateLimit {
    pub fn new() -> Self {
        Self { time: Instant::now() , count: 0 }
    }
}