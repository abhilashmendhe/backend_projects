use tokio::sync::mpsc::{Receiver, Sender};

use crate::{orderbook::orderbook::Orderbook, services::BuffEvents, utils::error::ExchangeErr};

pub async fn process_events(
    ticker: String,
    rest_url: String,
    sender: Sender<BuffEvents>,
    mut receiver: Receiver<BuffEvents>,
) -> Result<(), ExchangeErr> {
    let mut orderbook = Orderbook::new(ticker, 0, 0);
    while let Some(buff_events) = receiver.recv().await {
        // println!("{:?}", buff_events);
        // println!(
        //     "last_update_id: {}, first_b_u: {}",
        //     orderbook.last_update_id, orderbook.first_b_u
        // );
        match buff_events {
            BuffEvents::Snapshot(http_response_snapshot) => {
                orderbook.from_snapshot(&http_response_snapshot);
            }
            BuffEvents::WSStream(wsresponse) => {
                orderbook
                    .apply_update(&rest_url, sender.clone(), wsresponse)
                    .await?;
            }
        }
    }
    Ok(())
}
