use std::{
    f64::consts::E,
    sync::Mutex
};

#[derive(Debug)]
pub struct BloomFilter {
    m: u64,
    k: u64,
    barr: Mutex<Vec<u8>>,
}

impl BloomFilter {
    pub fn new(n: u64, p: f64) -> Self {
        let m = (-1_f64 * n as f64 * p.log(E)) / ((2.0_f64).log(E) as f64).powi(2);
        let k = (((m / n as f64) * (2_f64).log(E)).round()).max(1_f64);
        let barr = Mutex::new(vec![0 as u8; m as usize]);
        Self {
            m: m.ceil() as u64,
            k: k as u64,
            barr,
        }
    }

    fn locations(&self, item: &str) -> Vec<usize> {
        let mut positions = vec![];
        let h1 = mm3h::murmurhash2_64(item.as_bytes()) as u128;
        for i in 0..self.k {
            let pos = (h1 + (i as u128 * h1)) % self.m as u128;
            positions.push(pos as usize);
        }
        positions
    }

    pub fn insert(&self, item: &str) {
        for pos in self.locations(item) {
            let mut barr_lock = self.barr.lock().unwrap();
            barr_lock[pos] = 1;
        }
    }

    pub fn query(&self, item: &str) -> bool {
        for pos in self.locations(item) {
            let barr_lock = self.barr.lock().unwrap();
            if barr_lock[pos] == 0 {
                return false;
            }
        }
        true
    }
}
