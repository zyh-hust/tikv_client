mod client;
use client::Client;
use grpcio::EnvBuilder;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let client = Client::new(env);

    // get the regions
    let start_key = "a".to_string().into_bytes();
    client.scan_regions(start_key);

    let mut workers: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..4 {
        let mut client = client.clone();
        let t = thread::spawn(move || {
            let head = match i {
                0 => "a",
                1 => "b",
                2 => "c",
                3 => "d",
                _ => "a",
            };

            let mut count = 1;
            loop {
                let key = format!("{:?}-{}", head, count).into_bytes();
                let value = count.to_string().into_bytes();
                let resp = client.raw_put(key, value);
                println!("put count: {:?} resp: {:?}", count, resp);
                count += 1;
                if count == 100000000 {
                    count = 0;
                }
            }
        });
        workers.push(t);
    }

    for t in workers {
        t.join().unwrap();
    }
}
