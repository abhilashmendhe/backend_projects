use std::sync::atomic::AtomicUsize;

use std::time::Instant;

pub struct AppState {
    pub visit_count: AtomicUsize,
    pub service_up: Instant
}

impl AppState {
    pub fn new() -> Self {
        AppState { visit_count: AtomicUsize::new(0), service_up: Instant::now() }
    }
}