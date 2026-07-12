use redis::{
    AsyncCommands, RedisResult,
    aio::MultiplexedConnection,
    streams::{StreamId, StreamReadOptions, StreamReadReply},
};
use sqlx::PgPool;
use tokio_util::sync::CancellationToken;

use crate::utils::error::NotificationWorkerErr;

use crate::services::init_workers::spawn_workers;

pub mod make_connections;
pub mod services;
pub mod utils;

pub async fn run(
    shutdown: CancellationToken,
    num_workers: u32,
    priority: u8,
    max_retry_count: u8,
    platform: String,
    r_stream_group_name: String,
    q_stream_opts: StreamReadOptions,
    url_gateway: String,
    callback_url: String,
    db_conn: PgPool,
    q_conn: &mut MultiplexedConnection,
) -> Result<(), NotificationWorkerErr> {
    // 1. create channels
    let (tx, rx) = tokio::sync::mpsc::channel::<StreamId>(1000);
    let _ = spawn_workers(
        shutdown.clone(),
        priority,
        max_retry_count,
        platform.clone(),
        r_stream_group_name.clone(),
        num_workers,
        url_gateway,
        callback_url,
        rx,
        db_conn,
        q_conn,
    )
    .await;

    // 2. run background worker job
    loop {
        let stream = format!("{}-{}", platform, priority);
        let streams = [stream];
        let ids = [">"];
        tokio::select! {
            result = q_conn
                // .xread_options::<String, &str, RedisResult<StreamReadReply>>(
                .xread_options::<String, &str, RedisResult<StreamReadReply>>(
                    &streams,
                    &ids,
                    &q_stream_opts,
                ) => {
                        match result {
                            Ok(stream_read_reply) => {
                                // Process messages
                                match stream_read_reply {
                                    Ok(stream_data) => {
                                        let st_keys = stream_data.keys;
                                        for stk in st_keys {
                                            let streamids = stk.ids;
                                            for st_id in streamids {
                                                // println!("{:?}", st_id);
                                                let _ = tx.send(st_id).await;
                                            }
                                        }
                                    },
                                    Err(err) => {
                                        tracing::error!("stream-read-reply: {:?}", err);
                                    },
                                }

                            }

                            Err(err) => {
                                tracing::error!("redis-xgroup-read: {err}");
                            }
                        }
                    }
            _ = shutdown.cancelled() => {
                println!("\nCtrl-c command received!");
                println!("Gracefully shutting down worker node!");
                break;
            }

        }
    }
    Ok(())
}
