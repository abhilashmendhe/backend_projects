use std::time::Duration;

use actix_web::web;
use clap::Parser;
use notification_server::{
    make_connections::{get_db_pool, get_redis_conn},
    run,
    services::background_enqueue_job::push_to_redis_queue,
    utils::{app_state::AppState, config::Config, errors::NotificationServerErr},
};
use tokio::time::interval;
use tracing::level_filters::LevelFilter;

/**
 * $ cargo watch -q -c -w src/ -x "run -- --addr localhost -p 60001"
 */

#[derive(Parser, Debug)]
struct ServerCli {
    #[arg(short, long)]
    addr: String,

    #[arg(short, long)]
    port: u16,

    #[arg(short, long, default_value_t = 4)]
    server_workers: usize,

    #[arg(short, long, default_value_t = 4)]
    db_conn_workers: u32,

    #[arg(short, long, default_value_t = 5)]
    binterval_scheduler_t: u64,

    #[arg(long, default_value_t = 4)]
    num_background_workers: u64,

    #[arg(long, default_value_t = 100)]
    background_fetch_limit_rows: i64,
}

#[actix_web::main]
async fn main() -> Result<(), NotificationServerErr> {
    // 1. parse command line args
    let scli = ServerCli::parse();
    let server_addr = scli.addr;
    let port = scli.port;
    let db_conn_workers = scli.db_conn_workers;
    let server_workers = scli.server_workers;
    let binterval_scheduler_t = scli.binterval_scheduler_t;
    let num_background_workers = scli.num_background_workers;
    let background_fetch_limit_rows = scli.background_fetch_limit_rows;

    // 2. enable tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // 3. read .env file
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;
    let high_priority_max_retry = std::env::var("HIGH_PRIORITY_MAX_RETRY")?;
    let low_priority_max_retry = std::env::var("LOW_PRIORITY_MAX_RETRY")?;
    let redis_cache_url = std::env::var("REDIS_CACHE")?;
    let redis_ios_queue_url = std::env::var("REDIS_QUEUE_IOS")?;
    let redis_android_queue_url = std::env::var("REDIS_QUEUE_ANDROID")?;

    // 4. create config
    let config = Config::new(
        port,
        low_priority_max_retry.parse::<u8>()?,
        high_priority_max_retry.parse::<u8>()?,
    );

    // 5. make db connection and create pg pool
    let db_pool = get_db_pool(&db_url, db_conn_workers).await;

    // 6. get redis connections
    let r_cache = get_redis_conn(&redis_cache_url).await?;
    let r_ios_q = get_redis_conn(&redis_ios_queue_url).await?;
    let r_android_q = get_redis_conn(&redis_android_queue_url).await?;

    // 7. Get clone of db, ios, and android queue connection for background jobs
    let db_pool_c = db_pool.clone();
    let r_ios_q_c = r_ios_q.clone();
    let r_android_q_c = r_android_q.clone();

    // 7. Create app state
    let app_state = AppState::new(config, db_pool, r_cache, r_ios_q, r_android_q);

    // 8. Run background enqueue job
    let _ = tokio::spawn(async move {
        let mut scheduler = interval(Duration::from_secs(binterval_scheduler_t));
        loop {
            scheduler.tick().await;
            // println!("running in background");
            push_to_redis_queue(
                db_pool_c.clone(),
                r_ios_q_c.clone(),
                r_android_q_c.clone(),
                background_fetch_limit_rows,
            )
            .await;
        }
    });

    // 9. Start actix server
    run(
        &server_addr,
        port,
        server_workers,
        web::Data::new(app_state),
    )
    .await?;
    Ok(())
}
