use std::sync::Arc;
use std::time::Duration;
use std::error::Result;

use grpcio::{ChannelBuilder, Environment};
use kvproto::tikvpb::TikvClient;
use kvproto::kvrpcpb::{Context, RawPutRequest,RawPutResponse}; 
use kvproto::pdpb::{SplitRegionRequest,SplitRegionResponse};

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
    pub fn raw_put(&self, ctx: Context, key: Vec<u8>, value: Vec<u8>) -> RawPutResponse {
        let mut req = RawPutRequest::default();
        req.set_context(ctx);
        req.set_key(key);
        req.set_value(value);
        self.client.raw_put(&req) 
    }

    pub fn split_region(&self,ctx: Context,key: Vec<u8>) ->  SplitRegionResponse {
        let mut req = SplitRegionRequest::default();
        req.set_context(ctx);
        req.set_split_key(key.into_bytes());
        self.client.split_region(&req)
    }
}