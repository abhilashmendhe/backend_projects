use std::sync::{atomic::AtomicUsize, Arc};

use axum::extract::FromRef;


#[derive(Debug, Clone)]
pub struct AppState {
    pub visit_count: Arc<AtomicUsize>
}

impl AppState {
    pub fn new() -> Self {
        Self { visit_count: Arc::new(AtomicUsize::new(0)) }
    }
}

impl FromRef<AppState> for Arc<AtomicUsize> {
    fn from_ref(state: &AppState) -> Self {
        state.visit_count.clone()
    }
}