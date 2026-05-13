use std::{sync::Arc, thread};

use bf_v2::BloomFilter;
/**
 * Thread safe bloom filter implementation
 */

fn main() {
    let bf = Arc::new(BloomFilter::new(20, 0.1));

    let items = [
        "apple",
        "banana",
        "mango",
        "ice-cream",
        "chocolates",
        "milk",
        "pineapple",
    ];

    let mut join_handles = vec![];

    for item in items {
        let bf_clone = bf.clone();
        let t = thread::spawn(move || {
            bf_clone.insert(item);
        });
        join_handles.push(t);
    }

    for j in join_handles {
        j.join().unwrap();
    }

    println!("{}", bf.clone().query("app"));
    println!("{}", bf.clone().query("banana"));
}
