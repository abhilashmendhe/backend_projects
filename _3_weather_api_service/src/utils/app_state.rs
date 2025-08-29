use std::sync::{atomic::AtomicU32, Arc};

use axum::extract::FromRef;
use redis::{aio::MultiplexedConnection};

use crate::utils::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub visit_count: Arc<AtomicU32>,
    pub config: Config,
    pub conn: MultiplexedConnection
}

impl AppState {
    pub fn new(config: Config, conn: MultiplexedConnection) -> Self {
        let visit_count = Arc::new(AtomicU32::new(0));
        AppState { visit_count, config, conn }
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

impl FromRef<AppState> for MultiplexedConnection {
    fn from_ref(state: &AppState) -> Self {
        state.conn.clone()
    }
}