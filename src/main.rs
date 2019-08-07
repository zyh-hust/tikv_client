mod client;
use client::Client;
use grpcio::EnvBuilder;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let client = Client::new(env);

    // get the regions
    let start_key = "a".to_string().into_bytes();
    if client.scan_regions(start_key) {
        println!("scan end!");
    }

    let mut workers: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..128 {
        let mut client = client.clone();
        let mut now = Instant::now();
        let t = thread::spawn(move || {
            let head = "abczyh";
            let mut count = 1;
            loop {
                let key = format!("{:?}-{}", head, count).into_bytes();
                let value = format!("zyhzyhzyhzyhzyzhyzhzyzhyzhyzhzyzhyzh-{}", count).into_bytes();
                let resp = client.raw_put(key, value);
                count += 1;
                if count == 10000000 {
                    println!("thread {} end", i);
                    break;
                }
                if count % 10000 == 0 && i == 0 {
                    println!("time is {:?}", now.elapsed().as_secs());
                }
            }
        });
        workers.push(t);
    }

    for t in workers {
        t.join().unwrap();
    }
}
