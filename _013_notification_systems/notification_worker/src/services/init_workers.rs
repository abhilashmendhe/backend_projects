use std::sync::Arc;

use tokio::sync::{Mutex, mpsc::Receiver};

use redis::streams::StreamId;

pub async fn spawn_workers(_platform_spec_stream: &str, num_workers: u32, rx: Receiver<StreamId>) {
    let rx = Arc::new(Mutex::new(rx));
    for nw in 0..num_workers {
        let rx = rx.clone();
        tokio::spawn(async move {
            loop {
                let job = {
                    let mut rx_l = rx.lock().await;
                    rx_l.recv().await
                };
                if job.is_none() {
                    break;
                }

                // process job
                println!("worker {nw} {:?}", job);
            }
            // while let Some(streamid) = rx.recv().await {

            // }
        });
    }
}
