use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use crate::utils::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    config: Config,
    db_pool: PgPool,
    redis_cache: MultiplexedConnection,
    redis_ios_q: MultiplexedConnection,
    redis_android_q: MultiplexedConnection,
}

impl AppState {
    pub fn new(
        config: Config,
        db_pool: PgPool,
        redis_cache: MultiplexedConnection,
        redis_ios_q: MultiplexedConnection,
        redis_android_q: MultiplexedConnection,
    ) -> Self {
        Self {
            config,
            db_pool,
            redis_cache,
            redis_ios_q,
            redis_android_q,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn db_pool(&self) -> &PgPool {
        &self.db_pool
    }

    pub fn redis_cache(&self) -> MultiplexedConnection {
        self.redis_cache.clone()
    }

    pub fn redis_ios_q(&self) -> MultiplexedConnection {
        self.redis_ios_q.clone()
    }

    pub fn redis_android_q(&self) -> MultiplexedConnection {
        self.redis_android_q.clone()
    }
}
