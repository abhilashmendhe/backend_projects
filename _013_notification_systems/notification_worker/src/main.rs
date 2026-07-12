use std::time::Duration;

use clap::Parser;
use dotenv::dotenv;
use notification_worker::{
    make_connections::{get_db_pool, get_redis_conn},
    run,
    services::{
        background_clean_streams::clean_streams, process_pending_jobs::process_pending_jobs,
    },
    utils::error::NotificationWorkerErr,
};
use redis::{AsyncCommands, streams::StreamReadOptions};

use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::level_filters::LevelFilter;

/*
    1. Start notification worker
    $ cargo watch -q -c -w src/ -x "run -- --fetch-limit-jobs 5 --platform ios --consumer-name consumer-2 --priority high --max-retry-count 5"

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

    #[arg(long)]
    max_retry_count: String,

    #[arg(long, default_value = "127.0.0.1:9000/push")]
    url_gateway: String,

    #[arg(long, default_value = "")]
    callback_url: String,

    #[arg(long, default_value = "group")]
    redis_stream_group_name: String,

    #[arg(long, default_value_t = 30)]
    binterval_scheduler_t: u64,

    #[arg(long, default_value_t = 100)]
    stream_trim_max_entries: usize,
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
    let max_retry_count = scli.max_retry_count;
    let url_gateway = scli.url_gateway;
    let callback_url = scli.callback_url;
    let redis_stream_group_name = scli.redis_stream_group_name;
    let binterval_scheduler_t = scli.binterval_scheduler_t;
    let stream_trim_max_entries = scli.stream_trim_max_entries;

    let priority_n = if priority.to_lowercase().eq("low") {
        1 as u8
    } else if priority.to_lowercase().eq("high") {
        0
    } else {
        panic!("Didn't pass the right priority option. Should be either `low` or `high`!");
    };

    let r_stream_group_name = format!("{}-{}", redis_stream_group_name, priority_n);
    let max_retry_count_n = max_retry_count.parse::<u8>()?;

    // 2. enable tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
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

    // 5. Create xgroup
    match q_conn
        .clone()
        .xgroup_create::<String, String, String, String>(
            format!("{}-{}", platform, priority_n),
            r_stream_group_name.clone(),
            "0".to_string(),
        )
        .await
    {
        Ok(_) => {}
        Err(_) => {
            let _ = q_conn
                .xgroup_create_mkstream::<String, String, String, String>(
                    format!("{}-{}", platform, priority_n),
                    r_stream_group_name.clone(),
                    "$".to_string(),
                )
                .await;
        }
    };

    // 6. Stream options
    let q_stream_opts = StreamReadOptions::default()
        .count(fetch_limit_jobs)
        // .block(sleep_milli_secs)
        .block(100)
        .group(r_stream_group_name.clone(), consumer_name);

    // 8. Process pending jobs before starting background jobs
    process_pending_jobs(
        priority_n,
        max_retry_count_n,
        platform.clone(),
        r_stream_group_name.clone(),
        &q_stream_opts,
        url_gateway.clone(),
        callback_url.clone(),
        db_conn.clone(),
        &mut q_conn,
    )
    .await?;

    let shutdown = CancellationToken::new();

    let cleaner_token = shutdown.clone();
    // let run_token = shutdown.clone();

    // 7. start background cleaning
    let platform1 = platform.clone();
    let q_conn1 = q_conn.clone();
    let cleaner_token1 = cleaner_token.clone();
    let cleaner = tokio::spawn(async move {
        let mut scheduler = interval(Duration::from_secs(binterval_scheduler_t));
        let platform = platform1.clone();
        let mut q_conn = q_conn1.clone();
        loop {
            tokio::select! {
                _ = cleaner_token1.cancelled() => {
                    println!("Cleaner stopping");
                    break;
                }

                _ = scheduler.tick() => {
                    // println!("tikcin...");
                    let _ = clean_streams(
                        stream_trim_max_entries,
                        format!("{}-{}", platform.clone(), priority_n),
                        &mut q_conn,
                    )
                    .await;
                }
            }
        }
    });
    println!("will start fetch worker");
    // 8. run worker
    let shutdown1 = shutdown.clone();
    let mut q_conn1 = q_conn.clone();
    let fetch_worker = tokio::spawn(async move {
        run(
            shutdown1,
            num_workers,
            priority_n,
            max_retry_count_n,
            platform.clone(),
            r_stream_group_name.clone(),
            q_stream_opts,
            url_gateway.clone(),
            callback_url.clone(),
            db_conn.clone(),
            &mut q_conn1,
        ).await
    });
    // 9. create a background job that periodically XAUTOCLAIM stale messages from dead consumers
    /*
       Every 30-60 seconds
       XAUTOCLAIM idle > 5 min
       Process
       XACK
    */
    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await?;
    println!("Ctrl+C received");
    shutdown.cancel();
    fetch_worker.await??;
    cleaner.await?;

    Ok(())
}
