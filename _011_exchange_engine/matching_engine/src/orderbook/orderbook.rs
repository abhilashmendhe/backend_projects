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

    pub fn best_bid(&self) -> Option<(&Reverse<Decimal>, &Decimal)> {
        self.bids.first_key_value()
    }

    pub fn best_ask(&self) -> Option<(&Decimal, &Decimal)> {
        self.asks.first_key_value()
    }

    pub fn spread(&self) -> Option<Decimal> {
        if let Some((ask_price, _ask_quantity)) = self.best_ask() {
            if let Some((bid_price, _bid_quantity)) = self.best_ask() {
                return Some(*ask_price-bid_price);
            }
        }
        None
    }
}
