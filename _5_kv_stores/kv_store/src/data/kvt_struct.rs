use std::collections::HashMap;

#[derive(Debug)]
pub struct TimeBasedKV {
    pub kv: HashMap<String, Vec<(i32, String)>>
}

impl TimeBasedKV {

    pub fn new() -> Self {
        Self {
            kv: HashMap::new()
        }
    }
    
    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        
        if let Some(values) = self.kv.get_mut(&key) {
            values.push((timestamp, value));
        } else {
            let mut ts = Vec::new();
            let _  = ts.try_reserve(10000000);
            ts.push((timestamp, value));
            self.kv.insert(key, ts);
        }
    }
    
    pub fn get(&self, key: String, timestamp: i32) -> String {

        if let Some(values) = self.kv.get(&key) {
            
            let mut low: i32 = 0;
            let mut high: i32 = values.len() as i32 - 1;

            while low <= high {
                let mid = low + (high-low)/2;

                if values[mid as usize].0 == timestamp {
                    low = mid;
                    break;
                } else if values[mid as usize].0 < timestamp {
                    low = mid + 1;
                } else {

                    high = mid - 1;
                }
            }

            if high >= 0 && values[high as usize].0 <= timestamp {
                return values[high as usize].1.to_string();
            } else if low >= 0 && low < values.len() as i32 && values[low as usize].0 <= timestamp {
                return values[low as usize].1.to_string();
            }            
        }
        "".to_string()
    }

}
