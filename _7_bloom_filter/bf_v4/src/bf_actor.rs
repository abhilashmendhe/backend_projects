use std::{fs::File, io::Write};

use tokio::sync::mpsc;

use crate::{bf::BFCommand, errors::BFError};

#[derive(Debug)]
pub struct BloomFilterActor {
    m: u64,
    k: u64,
    bit_arr: Vec<u8>,
    rx: mpsc::Receiver<BFCommand>,
}

impl BloomFilterActor {
    pub fn new(n: u64, p: f64, data_path: Option<&str>, rx: mpsc::Receiver<BFCommand>) -> Self {

        let (m, k, bit_arr) = if let Some(path) = data_path {
            BloomFilterActor::read_data_file(path)
        } else {
            let m = ((-(n as f64) * p.ln()) / (2f64.ln().powi(2))).ceil() as u64;
            let k = (((m as f64 / n as f64) * (2_f64).ln()).round() as u64).max(1);
            let bit_arr = vec![0 as u8; m as usize];
            (m, k, bit_arr)
        };
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

    pub fn save(&self, path: &str) -> Result<(), BFError> {
        let mut packed = vec![];
        let mut byte = 0;
        for (i, b) in self.bit_arr.iter().enumerate() {
            byte = byte | (b << (i % 8));
            // println!("{byte}");
            if i % 8 == 7 {
                packed.push(byte);
                byte = 0;
            }
        }

        if self.bit_arr.len() % 8 != 0 {
            packed.push(byte);
        }
        let mut file = File::create(path)?;

        // 1. write magic number "BLOOM"
        file.write_all(b"BLOOM")?;

        // 2. write version number
        file.write_all(&1u32.to_le_bytes())?;

        // 3. write M 
        file.write_all(&self.m.to_le_bytes())?;

        // 4. write K
        file.write_all(&self.k.to_le_bytes())?;

        // 5. now write the bit-count of packed bits
        let bit_count = packed.len() as u32;
        file.write_all(&bit_count.to_le_bytes())?;

        // 6. now write the actual packed bits in a file
        file.write_all(&packed)?;
        file.flush()?;
        Ok(())
    }

    pub fn read_data_file(path: &str) -> (u64, u64, Vec<u8>) {
        let data = std::fs::read(path).unwrap();

        // 1. read the magic number and check if it's the correct number
        let magic_bytes = &data[0..5];

        if magic_bytes != b"BLOOM" {
            panic!("Invalid bloom filter data file!");
        }

        // 2. read the version number, and check
        let ver_bytes = &data[5..9];
        let version = u32::from_le_bytes(ver_bytes.try_into().unwrap());
        if version != 1 {
            panic!("Invalid version number");
        }

        // 3. read the `m` value
        let m_bytes = &data[9..17];
        let m = u64::from_le_bytes(m_bytes.try_into().unwrap());
        // println!("m: {}", m);

        //4. read the `k` value
        let k_bytes = &data[17..25];
        let k = u64::from_le_bytes(k_bytes.try_into().unwrap());
        // println!("k: {}", k);

        // 5. now read the packed bit vec size
        let bit_count_bytes = &data[25..29];
        let _bit_count = u32::from_le_bytes(bit_count_bytes.try_into().unwrap());

        let packed = &data[29..];
        let mut bits = vec![];
        for p in packed{
            let vv = (0..8).map(|n| (p >> n) & 1).collect::<Vec<u8>>();
            bits.extend(vv);
        }

        if bits.len() as u64 != m {
            panic!("Incorrect packed bits!");
        }
        (m, k, bits[..(m as usize)].to_vec())  
    }
    pub async fn run(&mut self) -> Result<(), BFError> {
        while let Some(cmd) = self.rx.recv().await {
            match cmd {
                BFCommand::INSERT(item) => self.insert(&item),
                BFCommand::QUERY(item, sender) => {
                    let f = self.query(&item);
                    sender.send((item, f)).unwrap();
                }
                BFCommand::SAVE(path) => self.save(&path)?,
            }
        }
        Ok(())
    }
}
