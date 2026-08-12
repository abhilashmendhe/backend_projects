use std::{cmp::Reverse, collections::BTreeMap};

use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Orderbook {  
    pub last_update_id: usize,
    pub bids: BTreeMap<Decimal, Decimal>,
    pub asks: BTreeMap<Reverse<Decimal>, Decimal>
}

impl Orderbook {
    pub fn new(last_update_id: usize) -> Self {
        Self { last_update_id, bids: BTreeMap::new(), asks: BTreeMap::new() }
    }
}