use std::sync::atomic::AtomicUsize;

use std::time::Instant;

use sqlx::PgPool;

use crate::utils::config::Config;

pub struct AppState {
    pub visit_count: AtomicUsize,
    pub service_up: Instant,
    pub config: Config, 
    pub pool: PgPool
}

impl AppState {
    pub fn new(config: Config, pool: PgPool) -> Self {
        AppState { 
            visit_count: AtomicUsize::new(0), 
            service_up: Instant::now(), 
            config,
            pool
        }
    }
}