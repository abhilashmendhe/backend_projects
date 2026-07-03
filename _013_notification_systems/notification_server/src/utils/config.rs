#[derive(Debug, Clone, Copy)]
pub struct Config {
    port: u16,
    high_priority_max_retry: u8,
    low_priority_max_retry: u8,
}

impl Config {
    pub fn new(port: u16, low_priority_max_retry: u8, high_priority_max_retry: u8) -> Self {
        Self {
            port,
            high_priority_max_retry,
            low_priority_max_retry,
        }
    }

    pub fn port(self) -> u16 {
        self.port
    }

    pub fn low_priority_max_retry(self) -> u8 {
        self.low_priority_max_retry
    }

    pub fn high_priority_max_retry(self) -> u8 {
        self.high_priority_max_retry
    }
}
