use redis::aio::MultiplexedConnection;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::utils::error::NotificationWorkerErr;

pub async fn get_db_pool(db_url: &str, db_conn_workers: u32) -> PgPool {
    match PgPoolOptions::new()
        .max_connections(db_conn_workers)
        .connect(&db_url)
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            println!("🔥 Failed to connect to DB: {:?}", err);
            std::process::exit(1);
        }
    }
}

pub async fn get_redis_conn(
    redis_url: &str,
) -> Result<MultiplexedConnection, NotificationWorkerErr> {
    let redis_client = redis::Client::open(redis_url)?;
    let redis_conn;
    loop {
        match redis_client.get_multiplexed_async_connection().await {
            Ok(conn) => {
                redis_conn = conn;
                break;
            }
            Err(err) => {
                tracing::error!("Connection to `{redis_url}, `{:?}", err);
                let _ = std::thread::sleep(std::time::Duration::from_millis(5000));
            }
        }
    }
    Ok(redis_conn)
}
