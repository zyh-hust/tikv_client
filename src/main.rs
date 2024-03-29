mod client;
use client::Client;
use grpcio::EnvBuilder;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Instant;
fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let client = Client::new(env);
    let mut client1 = client.clone();
    let split = vec!["bbb", "ccc", "ddd", "eee"];
    //    let split: Vec<char> = Vec::new();
    for ch in split {
        let mut resp = false;
        while !resp {
            resp = client1.split_region(ch.to_string().into_bytes());
        }
    }
    // get the regions
    let start_key = "a".to_string().into_bytes();
    if client.scan_regions(start_key) {
        println!("scan end!");
    }
    return;
    let mut workers: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..128 {
        let mut client = client.clone();
        let now = Instant::now();
        let head = match i % 3 {
            0 => "aaaaaaaaaaaaaaaa",
            1 => "bbbbbbbbbbbbbbbb",
            2 => "cccccccccccccccc",
            _ => "aaaaaaaaaaaaaaaa",
        };
        let t = thread::spawn(move || {
            let mut count = 1;
            loop {
                let key = format!("{}", head).into_bytes();
                let value = format!("zyhzyhzyhzyzhyzhzyzhyzhzyzhzyhzyzhzyzhzyhzyzhzyzhzyzhzyzhzyhzyzhzyzhzyhzyzhyzhzyzhzyzhyzhzyzhzyzhzyhzyzhzyzhzyhzzyhzyhzyhzyhzyzhyzhzyzhyzhyzhzyzhyzh-{}", count).into_bytes();
                let resp = client.raw_put(key, value);
                if resp == false {
                    println!("thread {} put {} error", i, count);
                }
                if count == 1 && i < 3 {
                    client.cache();
                }
                count += 1;
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
