use tokio::{fs::File, io::AsyncWriteExt, sync::mpsc};

use crate::{bf::BFCommand, errors::BFError};

#[derive(Debug)]
pub struct BloomFilterActor {
    m: u64,
    k: u64,
    bit_arr: Vec<u8>,
    rx: mpsc::Receiver<BFCommand>,
}

impl BloomFilterActor {
    pub fn new(n: u64, p: f64, rx: mpsc::Receiver<BFCommand>) -> Self {
        let m = ((-(n as f64) * p.ln()) / (2f64.ln().powi(2))).ceil() as u64;
        let k = (((m as f64 / n as f64) * (2_f64).ln()).round() as u64).max(1);
        let bit_arr = vec![0 as u8; m as usize];
        Self { m, k, bit_arr, rx }
    }

    fn locations(&self, item: &str) -> Vec<usize> {
        let mut positions = vec![];
        let h1 = mm3h::murmurhash2_64_with_seed(item.as_bytes(), 1) as u128;
        let h2 = fxhash::hash64(item.as_bytes()) as u128;
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

    pub async fn save(&self, path: &str) -> Result<(), BFError> {
        let mut packed = vec![];
        let mut byte = 0;
        for (i, b) in self.bit_arr.iter().enumerate() {
            byte = byte | (b << (i % 8));
            if i % 8 == 7 {
                packed.push(byte);
                byte = 0;
            }
        }
        if packed.len() % 8 != 0 {
            packed.push(byte);
        }
        let mut file = File::create("bits.bin").await?;
        let bit_count = packed.len() as u32;
        file.write_all(&bit_count.to_le_bytes()).await?;
        file.write_all(&packed).await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), BFError> {
        while let Some(cmd) = self.rx.recv().await {
            match cmd {
                BFCommand::INSERT(item) => self.insert(&item),
                BFCommand::QUERY(item, sender) => {
                    let f = self.query(&item);
                    sender.send((item, f)).unwrap();
                }
                BFCommand::SAVE(path) => self.save(&path).await?,
            }
        }
        Ok(())
    }
}
