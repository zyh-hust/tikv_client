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
    let ch = ChannelBuilder::new(env).connect("172.16.5.31:39860");
    let client = TikvClient::new(ch);

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

            let mut req_split = SplitRegionRequest::default();
            req_split.set_split_key(split.to_string().into_bytes());

            let resp = client.split_region(&req_split);
            println!("split {:?} response is {:?}", split, resp);

            let mut count = 1;

            loop {
                let mut rawputreq = RawPutRequest::default();
                rawputreq.set_key(format!("{:?}-{}", head, count).into_bytes());
                rawputreq.set_value(count.to_string().into_bytes());
                let resp = client.raw_put(&rawputreq);
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
