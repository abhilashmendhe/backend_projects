use std::{cmp::Reverse, collections::BTreeMap, fmt::Display};

use rust_decimal::Decimal;
use tokio::sync::mpsc::Sender;

use crate::{
    services::{
        BuffEvents,
        fetch_rest::{HttpResponseSnapshot, fetch_snapshot},
        stream_ws::WSResponse,
    },
    utils::error::ExchangeErr,
};

#[derive(Debug, Clone)]
pub struct Orderbook {
    pub ticker: String,
    pub last_update_id: usize,
    pub first_b_u: usize, // first buffered update
    pub found_event: bool,
    pub asks: BTreeMap<Decimal, Decimal>,
    pub bids: BTreeMap<Reverse<Decimal>, Decimal>,
}

impl Orderbook {
    pub fn new(ticker: String, last_update_id: usize, first_b_u: usize, found_event: bool) -> Self {
        Self {
            ticker,
            last_update_id,
            first_b_u,
            found_event,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn from_snapshot(&mut self, snapshot: &HttpResponseSnapshot) {
        self.asks.clear();
        self.bids.clear();
        self.last_update_id = snapshot.last_update_id;
        for ask in &snapshot.asks {
            if ask[0].is_zero() || ask[1].is_zero() {
                continue;
            }
            self.asks.insert(ask[0], ask[1]);
        }
        for bid in &snapshot.bids {
            if bid[0].is_zero() || bid[1].is_zero() {
                continue;
            }
            self.bids.insert(Reverse(bid[0]), bid[1]);
        }
    }

    pub async fn apply_update(
        &mut self,
        rest_url: &str,
        sender: Sender<BuffEvents>,
        wsresponse: WSResponse,
    ) -> Result<(), ExchangeErr> {
        if self.first_b_u <= 0 {
            self.first_b_u = wsresponse.U;
            if self.last_update_id > 0 && self.last_update_id < self.first_b_u {
                // println!("1. Not latest snapshot....");
                fetch_snapshot(rest_url.to_string(), sender.clone()).await?;
                // println!("2. Just fetched latest snapshot....\n");
            }
        }
        if wsresponse.U <= self.last_update_id + 1 && self.last_update_id + 1 <= wsresponse.u {
            self.found_event = true;
        }
        if self.found_event {
            for ask in wsresponse.a {
                if !ask[1].is_zero() {
                    self.asks.insert(ask[0], ask[1]);
                } else {
                    self.asks.remove(&ask[0]);
                }
            }
            for bid in wsresponse.b {
                if !bid[1].is_zero() {
                    self.bids.insert(Reverse(bid[0]), bid[1]);
                } else {
                    self.bids.remove(&Reverse(bid[0]));
                }
            }
        }
        println!("{}", self);
        Ok(())
    }

    pub fn best_bid(&self) -> Option<(Decimal, Decimal)> {
        self.bids
            .first_key_value()
            .map(|(price, quantity)| (price.0, *quantity))
    }

    pub fn best_ask(&self) -> Option<(Decimal, Decimal)> {
        self.asks
            .first_key_value()
            .map(|(price, quantity)| (*price, *quantity))
    }

    pub fn bid_ask_spread(&self) -> Option<Decimal> {
        let (ask_price, _) = self.best_ask()?;
        let (bid_price, _) = self.best_bid()?;

        Some(ask_price - bid_price)
    }

    pub fn mid_price(&self) -> Option<Decimal> {
        let (ask_price, _) = self.best_ask()?;
        let (bid_price, _) = self.best_bid()?;

        Some((ask_price + bid_price) / Decimal::from(2))
    }

    pub fn market_depth(&self) -> Option<Decimal> {
        let (ask_price, _) = self.asks.last_key_value()?;
        let (bid_price, _) = self.bids.last_key_value()?;

        Some(*ask_price - bid_price.0)
    }
}

impl Display for Orderbook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "\n{}\n-------------------\nBid: {:?}\nAsk: {:?}\nSpread: {:?}\n",
            self.ticker,
            self.best_bid(),
            self.best_ask(),
            self.bid_ask_spread()
        );
        write!(f, "{}", s)
    }
}
