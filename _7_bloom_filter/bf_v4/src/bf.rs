use crate::{bf_actor::BloomFilterActor, errors::BFError};

#[derive(Debug)]
pub enum BFCommand {
    INSERT(String),
    QUERY(String, tokio::sync::oneshot::Sender<(String, bool)>),
    SAVE(String),
}

#[derive(Debug, Clone)]
pub struct BloomFilter {
    tx: tokio::sync::mpsc::Sender<BFCommand>,
}

impl BloomFilter {
    pub async fn insert(self, item: &str) -> Result<(), BFError> {
        self.tx.send(BFCommand::INSERT(item.to_string())).await?;
        Ok(())
    }

    pub async fn save(self, item: &str) -> Result<(), BFError> {
        self.tx.send(BFCommand::SAVE(item.to_string())).await?;
        Ok(())
    }

    pub async fn query(self, item: &str) -> Result<(String, bool), BFError> {
        let (o_tx, o_rx) = tokio::sync::oneshot::channel();
        self.tx
            .send(BFCommand::QUERY(item.to_string(), o_tx))
            .await?;
        let values = o_rx.await?;
        Ok(values)
    }

    pub async fn spawn(n: u64, p: f64, num_workers: usize) -> Result<Self, BFError> {
        let (tx, rx) = tokio::sync::mpsc::channel(num_workers);
        let worker = tokio::spawn(async move {
            BloomFilterActor::new(n, p, rx).run().await?;
            Ok::<(), BFError>(())
        });
        let _ = worker;
        Ok(BloomFilter { tx })
    }
}
