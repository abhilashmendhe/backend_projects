use std::{cmp::Reverse, collections::BTreeMap};

use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Orderbook {
    pub ticker: String,
    pub last_update_id: usize,
    pub asks: BTreeMap<Decimal, Decimal>,
    pub bids: BTreeMap<Reverse<Decimal>, Decimal>,
}

impl Orderbook {
    pub fn new(ticker: String, last_update_id: usize) -> Self {
        Self {
            ticker,
            last_update_id,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn best_bid(&self) {}

    pub fn best_ask(&self) {}

    pub fn spread(&self) {}
}
