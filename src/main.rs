use grpcio::{ChannelBuilder, EnvBuilder};
use kvproto::kvrpcpb::{RawPutRequest, SplitRegionRequest};
use kvproto::tikvpb_grpc::TikvClient;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

//pub struct kv_client{
//    client: TikvClient,
//    caches:
//
//}

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let client = Client::new(env);
    
    let mut workers: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..4 {
        let client = client.clone();
        let t = thread::spawn(move || {
            let (head, split) = match i {
                0 => ("a", "b"),
                1 => ("b", "c"),
                2 => ("c", "d"),
                3 => ("d", "e"),
                _ => ("a", "b"),
            };

            let resp = client.split_region(split.to_owned().into_bytes());
            println!("split {:?} response is {:?}", split, resp);

            let mut count = 1;
            loop {
                let key = format!("{:?}-{}", head, count).into_bytes();
                let value = count.to_string().into_bytes();
                let resp = client.raw_put(key,value);
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
