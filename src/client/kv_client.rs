use super::error::{Error, Result};
use super::security::{SecurityConfig, SecurityManager};
use grpcio::{ChannelBuilder, Environment};
use kvproto::kvrpcpb::{
    Context, RawPutRequest, RawPutResponse, SplitRegionRequest, SplitRegionResponse,
};
use kvproto::tikvpb_grpc::TikvClient;
use std::sync::Arc;
use std::time::Duration;

const MAX_GRPC_RECV_MSG_LEN: i32 = 1024 * 1024 * 1024;
const MAX_GRPC_SEND_MSG_LEN: i32 = -1;

#[derive(Clone)]
pub struct KvClient {
    client: Arc<TikvClient>,
    address: String,
}

impl KvClient {
    pub fn new(env: Arc<Environment>, addr: &str) -> Result<KvClient> {
        let cb = ChannelBuilder::new(env)
            .max_receive_message_len(MAX_GRPC_RECV_MSG_LEN)
            .max_send_message_len(MAX_GRPC_SEND_MSG_LEN)
            .keepalive_time(Duration::from_secs(10))
            .keepalive_timeout(Duration::from_secs(3));
        let security_cof = SecurityConfig::default();
        let security_mgr = SecurityManager::new(&security_cof).unwrap();
        let channel = security_mgr.connect(cb, addr);
        let tikv_client = TikvClient::new(channel);
        let client = Arc::new(tikv_client);
        Ok(KvClient {
            client,
            address: addr.to_owned(),
        })
    }

    pub fn raw_put(&self, ctx: Context, key: Vec<u8>, value: Vec<u8>) -> Result<RawPutResponse> {
        let mut req = RawPutRequest::default();
        req.set_context(ctx);
        req.set_key(key);
        req.set_value(value);
        match self.client.raw_put(&req) {
            Ok(resp) => Ok(resp),
            Err(err) => Err(Error::KvError(format!("raw put err : {:?}", err))),
        }
    }

    pub fn split_region(&self, ctx: Context, key: Vec<u8>) -> Result<SplitRegionResponse> {
        let mut req = SplitRegionRequest::default();
        req.set_context(ctx);
        req.set_split_key(key);
        match self.client.split_region(&req) {
            Ok(resp) => Ok(resp),
            Err(err) => Err(Error::KvError(format!("split_region err {:?}", err))),
        }
    }
}
