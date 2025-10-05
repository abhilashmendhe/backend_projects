use std::collections::HashMap;

#[derive(Debug)]
pub struct TimeBasedKV {
    pub kv: HashMap<String, Vec<(u64, String)>>
}

impl TimeBasedKV {

    pub fn new() -> Self {
        Self {
            kv: HashMap::new()
        }
    }
    
    pub fn set(&mut self, key: String, value: String, timestamp: u64) {
        
        if let Some(values) = self.kv.get_mut(&key) {
            values.push((timestamp, value));
        } else {
            let mut ts = Vec::new();
            let _  = ts.try_reserve(10000000);
            ts.push((timestamp, value));
            self.kv.insert(key, ts);
        }
    }
    
    pub fn get(&self, key: String, timestamp: u64) -> String {

        let mut result = String::new();
        if let Some(values) = self.kv.get(&key) {
            
            let mut low = 0;
            let mut high = values.len()  - 1;

            if timestamp < values[low].0 {
                return result;
            } else if timestamp > values[high].0 {
                return values[high].1.clone();
            }
            while low <= high {
                let mid = (high+low)/2;
                if values[mid].0 == timestamp {
                    result = values[mid].1.clone();
                    break;
                } else if values[mid].0 <= timestamp {
                    low = mid + 1;
                    result = values[mid].1.clone();
                } else {
                    match mid.checked_sub(1) {
                        Some(res) => {
                            high = res;
                        },
                        None => break,
                    }
                }
            } 
        }
        result
    }

}
