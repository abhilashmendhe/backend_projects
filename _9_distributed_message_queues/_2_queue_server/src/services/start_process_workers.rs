use std::sync::Arc;

use redis::aio::MultiplexedConnection;
use tokio::sync::Mutex;

use crate::{
    models::{payload_req::PayloadReq, payload_send::SendPayload},
    services::add_redis_list::add_to_redis_list,
};

pub async fn start_process_workers(
    redis_conn: MultiplexedConnection,
    num_process_workers: usize,
    rx: tokio::sync::mpsc::Receiver<PayloadReq>,
) {
    let shared_rx = Arc::new(Mutex::new(rx));
    // let shared_redis_conn = Arc::new(redis_conn);
    for _ in 0..num_process_workers {
        let rx_clone = shared_rx.clone();
        let shared_redis_conn = redis_conn.clone();
        actix_web::rt::spawn(async move {
            loop {
                let mut rx_lock = rx_clone.lock().await;
                let message = rx_lock.recv().await;
                drop(rx_lock);

                if let Some(payload_req) = message {
                    // Send it to redis
                    let payload_send = SendPayload::new(payload_req, 0, 5);
                    add_to_redis_list(shared_redis_conn.clone(), payload_send).await;
                } else {
                    break;
                }
            }
        });
    }
}
