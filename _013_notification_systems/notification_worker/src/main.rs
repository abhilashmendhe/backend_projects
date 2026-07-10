use clap::Parser;
use dotenv::dotenv;
use notification_worker::{
    make_connections::{get_db_pool, get_redis_conn},
    run,
    utils::error::NotificationWorkerErr,
};
use redis::{AsyncCommands, streams::StreamReadOptions};

use tracing::level_filters::LevelFilter;

/*
    1. Start notification worker
    $ cargo watch -q -c -w src/ -x "run -- --platform ios --consumer-name consumer-1 --priority low"

    2. Get groups info for specific stream
    $ XINFO GROUPS ios-0

    3. read from stream from starting only 2 data
    $ XREAD COUNT 2 STREAMS ios-0 0

    4. group read from stream only 2 data from start
    $ XREADGROUP GROUP group-1 consumer-1 COUNT 2 STREAMS ios-0 0

    5. ack
    $ XACK ios-0 group-1 1783408591682-0

    6. xrange
    $ XRANGE ios-0 - + COUNT 100
*/

#[derive(Debug, Parser)]
pub struct ServerCli {
    #[arg(long, default_value_t = 4)]
    num_workers: u32,

    #[arg(long, default_value_t = 4)]
    db_conn_workers: u32,

    #[arg(long, default_value_t = 20)]
    fetch_limit_jobs: usize,

    #[arg(long)] // ios, or android
    platform: String,

    #[arg(long)]
    consumer_name: String,

    #[arg(long)]
    priority: String,

    #[arg(long, default_value = "127.0.0.1:9000/push")]
    url_gateway: String,

    #[arg(long, default_value = "")]
    callback_url: String, 
}

#[tokio::main]
async fn main() -> Result<(), NotificationWorkerErr> {
    // 1. Parse server cli
    let scli = ServerCli::parse();
    let num_workers = scli.num_workers;
    let db_conn_workers = scli.db_conn_workers;
    let fetch_limit_jobs = scli.fetch_limit_jobs;
    let platform = scli.platform;
    let consumer_name = scli.consumer_name;
    let priority = scli.priority;
    let url_gateway = scli.url_gateway;
    let callback_url = scli.callback_url;

    let priority_n = if priority.to_lowercase().eq("low") {
        1 as u8
    } else if priority.to_lowercase().eq("high") {
        0
    } else {
        panic!("Didn't pass the right priority option. Should be either `low` or `high`!");
    };
    // 2. enable tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::ERROR)
        .init();

    let process_id = std::process::id();
    tracing::info!("Notification worker running on ::{}", process_id);

    // 3. Read .env file
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;
    let q_url = if platform.eq("ios") {
        std::env::var("REDIS_QUEUE_IOS")?
    } else if platform.eq("andoird") {
        std::env::var("REDIS_QUEUE_ANDROID")?
    } else {
        panic!("Message queue for platform {} doesn't exists!", platform);
    };

    // 4. Get connection
    let db_conn = get_db_pool(&db_url, db_conn_workers).await;
    let mut q_conn = get_redis_conn(&q_url).await?;

    // 4.5 destroy and recreate group
    // XGROUP DESTROY ios-1 group-1
    // XGROUP CREATE ios-1 group-1 0
    let _ = q_conn
        .clone()
        .xgroup_destroy::<String, String, String>(
            format!("{}-{}", platform, priority_n),
            "group-1".to_string(),
        )
        .await;

    // 5. Create xgroup
    let _ = q_conn
        .clone()
        .xgroup_create::<String, String, String, String>(
            format!("{}-{}", platform, priority_n),
            "group-1".to_string(),
            "0".to_string(),
        )
        .await;

    // 6. Stream options
    let q_stream_opts = StreamReadOptions::default()
        .count(fetch_limit_jobs)
        // .block(sleep_milli_secs)
        .block(300)
        .group("group-1", consumer_name);

    // 7. run worker
    run(
        num_workers,
        priority_n,
        platform,
        q_stream_opts,
        url_gateway,
        callback_url,
        db_conn,
        &mut q_conn,
    )
    .await?;
    Ok(())
}
