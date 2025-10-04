use std::sync::{atomic::AtomicUsize, Arc};

use axum::extract::FromRef;
use tokio::sync::Mutex;

use crate::data::kvt_struct::TimeBasedKV;


#[derive(Debug, Clone)]
pub struct AppState {
    pub visit_count: Arc<AtomicUsize>, 
    pub kv_store: Arc<Mutex<TimeBasedKV>>
}

impl AppState {
    pub fn new() -> Self {
        let tbkv = TimeBasedKV::new();
        Self { 
            visit_count: Arc::new(AtomicUsize::new(0)),
            kv_store: Arc::new(Mutex::new(tbkv))
        }
    }
}

impl FromRef<AppState> for Arc<AtomicUsize> {
    fn from_ref(state: &AppState) -> Self {
        state.visit_count.clone()
    }
}

impl FromRef<AppState> for Arc<Mutex<TimeBasedKV>> {
    fn from_ref(state: &AppState) -> Self {
        state.kv_store.clone()
    }
}