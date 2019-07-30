use std::sync::Arc;
use std::time::Duration};
use std::result;

use grpcio::{CallOption, Environment};
use grpcio::ChannelBuilder;
use kvproto::{errorpb, kvrpcpb, tikvpb::TikvClient};
use kvproto::kvrpcpb::Context;

pub struct KvClient{
    client: Arc<TikvClient>,
    address: String,
}

impl KvClient{
    pub fn new(env: Arc<Environment>, addr: &str) -> Result<Ok(KvClient),Err>{
        let cb = ChannelBuilder::new(env)
            .keepalive_time(Duration::from_secs(10))
            .keepalive_timeout(Duration::from_secs(3));
        let channel = cb.connect(addr);
        let tikv_client = TikvClient::new(channel);
        let client = Arc::new(tikv_client);
        Ok(KvClient{
            client,
            address: addr.to_owned(),
        })
    }
    pub fn raw_put(&self, ctx: Context, key: String, value: String) {
        let mut req = kvrpcpb::RawPutRequest::new();
        req.set_context(ctx);
        req.set_key(key.into_bytes());
        req.set_value(value.into_bytes());
        // TODO: handle RawPutResponse
        self.client.raw_put(&req);
    }
}