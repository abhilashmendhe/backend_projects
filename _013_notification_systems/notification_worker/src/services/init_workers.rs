use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::{Mutex, mpsc::Receiver};

use redis::{aio::MultiplexedConnection, streams::StreamId};

use crate::{services::process_job::process_job, utils::error::NotificationWorkerErr};

pub async fn spawn_workers(
    priority: u8,
    max_retry_count: u8,
    platform: String,
    r_stream_group_name: String,
    num_workers: u32,
    url_gateway: String,
    callback_url: String,
    rx: Receiver<StreamId>,
    db_conn: PgPool,
    q_conn: &mut MultiplexedConnection,
) -> Result<(), NotificationWorkerErr> {
    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..num_workers {
        let platform = platform.clone();
        let r_stream_group_name = r_stream_group_name.clone();
        let rx = rx.clone();
        let db_conn = db_conn.clone();
        let url_gateway = url_gateway.clone();
        let callback_url = callback_url.clone();
        let q_conn = q_conn.clone();
        tokio::spawn(async move {
            loop {
                if let Some(job) = {
                    let mut rx_l = rx.lock().await;
                    rx_l.recv().await
                } {
                    // process job
                    // println!("worker {nw} {:?}", job);
                    let _ = process_job(
                        priority,
                        max_retry_count,
                        platform.clone(),
                        r_stream_group_name.clone(),
                        job,
                        url_gateway.clone(),
                        callback_url.clone(),
                        db_conn.clone(),
                        &mut q_conn.clone(),
                    )
                    .await;
                } else {
                    break;
                };
            }
        });
    }
    Ok(())
}
