use axum::extract::FromRef;
use bloomfilter::bf::BloomFilter;

#[derive(Debug, Clone)]
pub struct AppState {
    bloom_filter: BloomFilter,
}

impl AppState {
    pub fn new(bloom_filter: BloomFilter) -> Self {
        Self { bloom_filter }
    }
}

impl FromRef<AppState> for BloomFilter {
    fn from_ref(state: &AppState) -> Self {
        state.bloom_filter.clone()
    }
}
