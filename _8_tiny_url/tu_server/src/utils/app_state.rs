use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::utils::config::Config;

pub struct AppState {
    config: Config,
    pool: PgPool,
    redis_conn: MultiplexedConnection,
}

impl AppState {
    pub fn new(config: Config, pool: PgPool, redis_conn: MultiplexedConnection) -> Self {
        Self {
            config,
            pool,
            redis_conn,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn redis_conn(&self) -> &MultiplexedConnection {
        &self.redis_conn
    }
}
