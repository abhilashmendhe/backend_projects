use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    services::{BuffEvents, fetch_rest::fetch_snapshot},
    utils::error::ExchangeErr,
};

pub async fn process_events(
    rest_url: String,
    sender: Sender<BuffEvents>,
    mut receiver: Receiver<BuffEvents>,
) -> Result<(), ExchangeErr> {
    let mut last_update_id = 0;
    let mut first_b_u = 0;
    // let mut prices = vec![];
    while let Some(buff_events) = receiver.recv().await {
        // println!("{:?}", buff_events);
        println!("last_update_id: {last_update_id}, first_b_u: {first_b_u}");
        match buff_events {
            BuffEvents::Snapshot(http_response_snapshot) => {
                // println!("{:?}", http_response_snapshot);
                last_update_id = http_response_snapshot.last_update_id;
            }
            BuffEvents::WSStream(wsresponse) => {
                // prices.push();
                if first_b_u <= 0 {
                    first_b_u = wsresponse.U;
                    if last_update_id > 0 && last_update_id < first_b_u {
                        println!("1. Not latest snapshot....");
                        fetch_snapshot(rest_url.clone(), sender.clone()).await?;
                        println!("2. Just fetched latest snapshot....\n");
                    }
                }
            }
        }
    }
    Ok(())
}
