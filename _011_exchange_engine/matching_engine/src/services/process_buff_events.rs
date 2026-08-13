use std::cmp::Reverse;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    orderbook::orderbook::Orderbook,
    services::{
        BuffEvents,
        fetch_rest::{HttpResponseSnapshot, fetch_snapshot},
    },
    utils::error::ExchangeErr,
};

pub async fn process_events(
    ticker: String,
    rest_url: String,
    sender: Sender<BuffEvents>,
    mut receiver: Receiver<BuffEvents>,
) -> Result<(), ExchangeErr> {
    // let mut last_update_id = 0;
    let mut snapshot = HttpResponseSnapshot::default();
    let mut orderbook = Orderbook::new(ticker, snapshot.last_update_id);
    let mut first_b_u = 0;
    let mut found_event = false;
    // let mut prices = vec![];
    while let Some(buff_events) = receiver.recv().await {
        // println!("{:?}", buff_events);
        // println!("last_update_id: {}, first_b_u: {first_b_u}", snapshot.last_update_id);
        match buff_events {
            BuffEvents::Snapshot(http_response_snapshot) => {
                // last_update_id = http_response_snapshot.last_update_id;
                snapshot = http_response_snapshot.clone();
                orderbook.asks.clear();
                orderbook.bids.clear();
                orderbook.last_update_id = snapshot.last_update_id;
                for ask in http_response_snapshot.asks {
                    if !ask[1].is_zero() {
                        orderbook.asks.insert(ask[0], ask[1]);
                    }
                }
                for bid in http_response_snapshot.bids {
                    if !bid[1].is_zero() {
                        orderbook.bids.insert(Reverse(bid[0]), bid[1]);
                    }
                }
            }
            BuffEvents::WSStream(wsresponse) => {
                // prices.push();
                if first_b_u <= 0 {
                    first_b_u = wsresponse.U;
                    if snapshot.last_update_id > 0 && snapshot.last_update_id < first_b_u {
                        println!("1. Not latest snapshot....");
                        fetch_snapshot(rest_url.clone(), sender.clone()).await?;
                        println!("2. Just fetched latest snapshot....\n");
                    }
                }
                if wsresponse.U <= snapshot.last_update_id + 1
                    && snapshot.last_update_id + 1 <= wsresponse.u
                {
                    found_event = true;
                }
                if found_event {
                    for ask in wsresponse.a {
                        if !ask[1].is_zero() {
                            orderbook.asks.insert(ask[0], ask[1]);
                        } else {
                            orderbook.asks.remove(&ask[0]);
                        }
                    }
                    for bid in wsresponse.b {
                        if !bid[1].is_zero() {
                            orderbook.bids.insert(Reverse(bid[0]), bid[1]);
                        } else {
                            orderbook.bids.remove(&Reverse(bid[0]));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
