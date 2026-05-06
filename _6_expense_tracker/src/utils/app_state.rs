use std::sync::atomic::AtomicUsize;

use std::time::Instant;

use sqlx::PgPool;

use crate::utils::config::Config;

pub enum CategoriesE {
    Food,
    Transport,
    Housing,
    Shopping,
    Bills,
    Entertainment,
    Health,
    Travel,
    Misc,
    UNSET
}

impl From<String> for CategoriesE {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "food" => CategoriesE::Food,
            "transport" => CategoriesE::Transport,
            "housing" => CategoriesE::Housing,
            "bills" => CategoriesE::Bills,
            "entertainment" => CategoriesE::Entertainment,
            "health" => CategoriesE::Health,
            "travel" => CategoriesE::Travel,
            "misc" | "miscellaneous" => CategoriesE::Misc,
            _ => CategoriesE::UNSET
        }
    }
}

pub struct AppState {
    pub visit_count: AtomicUsize,
    pub service_up: Instant,
    pub config: Config,
    pub pool: PgPool,
    pub categories_e: CategoriesE
}

impl AppState {
    pub fn new(config: Config, pool: PgPool) -> Self {
        AppState {
            visit_count: AtomicUsize::new(0),
            service_up: Instant::now(),
            config,
            pool,
            categories_e: CategoriesE::UNSET
        }
    }
}
