use sqlx::PgPool;

use crate::utils::config::Config;

pub struct AppState {
    config: Config,
    pool: PgPool,
}

impl AppState {
    pub fn new(config: Config, pool: PgPool) -> Self {
        Self { config, pool }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
