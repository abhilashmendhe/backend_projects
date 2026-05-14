use bf_v3::{BFCommand, bf::BloomFilter, errors::BFError};

#[tokio::main]
async fn main() -> Result<(), BFError> {
    // 0. Init my bloom filter
    let mut bf = BloomFilter::new(10, 0.1);

    // 1. Client tx, rx channel
    let (tx, mut rx) = tokio::sync::mpsc::channel::<BFCommand>(4);

    // 2. Spawn worker tasks
    let worker = tokio::spawn(async move {
        // let recv_tx = recv_tx.clone();
        while let Some(cmd) = rx.recv().await {
            match cmd {
                BFCommand::INSERT(item) => {
                    bf.insert(&item);
                }
                BFCommand::QUERY(item, recv_tx) => {
                    let v = bf.query(&item);
                    let _ = recv_tx.send((item, v));
                }
                BFCommand::SAVE(_) => {}
            }
        }
    });

    // 3. Make queries to insert and get
    let mut handles = vec![];
    for item in [
        "apple",
        "banana",
        "mango",
        "pineapple",
        "chiku",
        "peach",
        "milk",
        "chocolate",
    ] {
        let tx = tx.clone();
        let t = tokio::spawn(async move {
            tx.send(BFCommand::INSERT(item.to_string())).await?;
            Ok::<(), BFError>(())
        });
        handles.push(t);
    }

    for item in [
        "apple",
        "banana",
        "mango",
        "pinee",
        "chiku",
        "milk-chocolate",
    ] {
        let tx = tx.clone();
        let t = tokio::spawn(async move {
            let (recv_tx, recv_rx) = tokio::sync::oneshot::channel();
            tx.send(BFCommand::QUERY(item.to_string(), recv_tx)).await?;

            let (item, exists) = recv_rx.await?;
            println!("{item}: {exists}");
            Ok::<(), BFError>(())
        });
        handles.push(t);
    }
    
    for h in handles {
        h.await??;
    }
    drop(tx);

    worker.await?;
    Ok(())
}
