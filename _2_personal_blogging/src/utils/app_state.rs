
use sqlx::Pool;
use crate::utils::config::Config;

pub struct AppState<T: sqlx::Database> {
    pub config: Config,
    pub db: Pool<T>
}

impl<T: sqlx::Database> AppState<T> {
    pub fn new(config: Config, db: Pool<T>) -> Self {
        Self { config, db }
    }
}