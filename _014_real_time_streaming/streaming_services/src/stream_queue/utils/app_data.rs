use crate::utils::logger::WalLogger;

#[derive(Debug)]
pub struct StreamAppData {
    pub logger: WalLogger,
}

impl StreamAppData {
    pub fn new(logger: WalLogger) -> Self {
        Self { logger }
    }
}
