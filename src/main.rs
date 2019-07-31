mod client;
use client::Client;
use grpcio::EnvBuilder;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let client = Client::new(env);

    let start_key = "a".to_string().into_bytes();
    let resp = client.scan_regions(start_key);
    if resp {
        return;
    }
    let mut workers: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..4 {
        let mut client = client.clone();
        let t = thread::spawn(move || {
            let (head, split) = match i {
                0 => ("a", "b"),
                1 => ("b", "c"),
                2 => ("c", "d"),
                3 => ("d", "e"),
                _ => ("a", "b"),
            };

            let mut resp = false;
            while !resp {
                resp = client.split_region(split.to_owned().into_bytes());
                println!("split {:?} response is {:?}", split, resp);
                thread::sleep(Duration::from_secs(10));
            }

            thread::sleep(Duration::from_secs(100));
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
