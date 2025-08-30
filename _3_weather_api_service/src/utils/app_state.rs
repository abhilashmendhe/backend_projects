use std::sync::{atomic::AtomicU32, Arc};

use axum::extract::FromRef;
use redis::{aio::MultiplexedConnection};
use tokio::{sync::Mutex, time::Instant};

use crate::utils::{config::Config, rate_limit::RateLimit};

#[derive(Debug, Clone)]
pub struct AppState {
    pub visit_count: Arc<AtomicU32>,
    pub running: Instant,
    pub config: Config,
    pub conn: MultiplexedConnection,
    pub rate_limit: Arc<Mutex<RateLimit>>
}

impl AppState {
    pub fn new(config: Config, conn: MultiplexedConnection) -> Self {
        let running = Instant::now();
        let visit_count = Arc::new(AtomicU32::new(0));
        let rate_limit = Arc::new(Mutex::new(RateLimit::new()));
        AppState { visit_count, running, config, conn, rate_limit }
    }
}

impl FromRef<AppState> for Arc<Mutex<RateLimit>> {
    fn from_ref(state: &AppState) -> Self {
        state.rate_limit.clone()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

impl FromRef<AppState> for Arc<AtomicU32> {
    fn from_ref(state: &AppState) -> Self {
        state.visit_count.clone()
    }
}

impl FromRef<AppState> for Instant {
    fn from_ref(state: &AppState) -> Self {
        state.running.clone()
    }
}

impl FromRef<AppState> for MultiplexedConnection {
    fn from_ref(state: &AppState) -> Self {
        state.conn.clone()
    }
}