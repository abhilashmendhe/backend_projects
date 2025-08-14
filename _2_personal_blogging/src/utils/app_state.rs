use std::sync::{atomic::AtomicU32, Arc};

use sqlx::PgPool;
use crate::utils::config::Config;
use axum::extract::FromRef;

#[derive(Debug, Clone)]
pub struct AppState {
    pub visit_count: Arc<AtomicU32>,
    pub config: Config,
    pub db: PgPool
}

impl AppState {
    pub fn new(config: Config, db: PgPool) -> Self {
        let visit_count = Arc::new(AtomicU32::new(0));
        Self { visit_count, config, db }
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for Arc<AtomicU32> {
    fn from_ref(state: &AppState) -> Self {
        state.visit_count.clone()
    }
}