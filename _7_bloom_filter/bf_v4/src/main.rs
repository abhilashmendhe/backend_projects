use bf_v4::{bf::BloomFilter, errors::BFError};

#[tokio::main]
async fn main() -> Result<(), BFError> {
    let bloom_filter = BloomFilter::spawn(10, 0.1, 4, Some("./my-data/bits1.bin")).await?;

    // let bloom_filter = BloomFilter::spawn(10, 0.1, 4, None).await?;
    // let mut handles = vec![];
    // for item in [
    //     "apple",
    //     "banana",
    //     "mango",
    //     "pineapple",
    //     "chiku",
    //     "peach",
    //     "milk",
    //     "chocolate",
    // ] {
    //     let bf1 = bloom_filter.clone();
    //     let t = tokio::spawn(async move {
    //         bf1.insert(item).await?;
    //         Ok::<(), BFError>(())
    //     });
    //     handles.push(t);
    // }

    for item in [
        "apple",
        "banana",
        "mango",
        "pinee",
        "chiku",
        "milk-chocolate",
    ] {
        let f = bloom_filter.clone().query(item).await?;
        println!("{} -> {}", item, f.1);
    }

    // for h in handles {
    //     h.await??;
    // }

    // let file_worker = tokio::spawn(async move { bloom_filter.save("./my-data/bits1.bin").await });

    // match file_worker.await {
    //     Ok(inner) => match inner {
    //         Ok(_) => println!("save success"),
    //         Err(e) => println!("save error: {:?}", e),
    //     },
    //     Err(e) => println!("join error: {:?}", e),
    // }
    Ok(())
}
