#[derive(Debug)]
pub struct BloomFilter {
    m: u64,
    k: u64,
    bit_arr: Vec<u8>,
}

impl BloomFilter {
    pub fn new(n: u64, p: f64) -> Self {
        // let m = (-1_f64 * n as f64 * p.log(E)) / ((2.0_f64).log(E) as f64).powi(2);
        let m = ((-(n as f64) * p.ln()) / (2f64.ln().powi(2))).ceil() as u64;
        let k = (((m as f64 / n as f64) * (2_f64).ln()).round() as u64).max(1);
        let bit_arr = vec![0 as u8; m as usize];
        Self { m, k, bit_arr }
    }

    fn locations(&self, item: &str) -> Vec<usize> {
        let mut positions = vec![];
        let h1 = mm3h::murmurhash2_64_with_seed(item.as_bytes(), 0) as u128;
        let h2 = mm3h::murmurhash2_64_with_seed(item.as_bytes(), 1) as u128;
        for i in 0..self.k {
            let pos = (h1 + (i as u128 * h2)) % self.m as u128;
            positions.push(pos as usize);
        }
        positions
    }

    pub fn insert(&mut self, item: &str) {
        for pos in self.locations(item) {
            self.bit_arr[pos] = 1;
        }
    }

    pub fn query(&self, item: &str) -> bool {
        for pos in self.locations(item) {
            if self.bit_arr[pos] == 0 {
                return false;
            }
        }
        true
    }
}
