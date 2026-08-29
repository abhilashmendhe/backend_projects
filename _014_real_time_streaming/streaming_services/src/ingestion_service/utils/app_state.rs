use std::{sync::atomic::AtomicU64, time::Instant};

use crate::utils::config::Config;

#[derive(Debug)]
pub struct AppState {
    visit_count: AtomicU64,
    alive_time: Instant,
    config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            visit_count: AtomicU64::new(0),
            alive_time: Instant::now(),
            config,
        }
    }

    pub fn visit_count(&self) -> u64 {
        self.visit_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1
    }
    pub fn alive_time(&self) -> Instant {
        self.alive_time
    }
    pub fn config(&self) -> Config {
        self.config
    }
}
