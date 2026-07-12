use redis::{
    AsyncCommands, RedisResult,
    aio::MultiplexedConnection,
    streams::{StreamReadOptions, StreamReadReply},
};
use sqlx::PgPool;

use crate::{services::process_job::process_job, utils::error::NotificationWorkerErr};

pub async fn process_pending_jobs(
    priority: u8,
    max_retry_count: u8,
    platform: String,
    r_stream_group_name: String,
    q_stream_opts: &StreamReadOptions,
    url_gateway: String,
    callback_url: String,
    db_conn: PgPool,
    q_conn: &mut MultiplexedConnection,
) -> Result<(), NotificationWorkerErr> {
    let stream = format!("{}-{}", platform, priority);
    let streams = [stream];
    let ids = ["0"];

    let stream_read_reply = q_conn
        // .xread_options::<String, &str, RedisResult<StreamReadReply>>(
        .xread_options::<String, &str, RedisResult<StreamReadReply>>(&streams, &ids, &q_stream_opts)
        .await??;

    let st_keys = stream_read_reply.keys;
    for stk in st_keys {
        let streamids = stk.ids;
        for job in streamids {
            // println!("{:?}", st_id);
            process_job(
                priority,
                max_retry_count,
                platform.clone(),
                r_stream_group_name.clone(),
                job,
                url_gateway.clone(),
                callback_url.clone(),
                db_conn.clone(),
                q_conn,
            )
            .await?;
        }
    }
    Ok(())
}
