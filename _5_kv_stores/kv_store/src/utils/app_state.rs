use std::sync::Arc;

use axum::extract::FromRef;
use tokio::sync::Mutex;


#[derive(Debug, Clone)]
pub struct AppState {
    pub visit_count: Arc<Mutex<u64>>
}

impl AppState {
    pub fn new() -> Self {
        Self { visit_count: Arc::new(Mutex::new(0)) }
    }
}

impl FromRef<AppState> for Arc<Mutex<u64>> {
    fn from_ref(state: &AppState) -> Self {
        state.visit_count.clone()
    }
}