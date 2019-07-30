use grpcio::{ChannelBuilder,EnvBuilder},
use kvproto::kvrpcpb::{RawPutRequest, SplitRegionRequest};
use kvproto::tikvpb_grpc::TikvClient;
use kvproto::pdpd::GetRegionRequest;
use kvproto::pdpd_grpc::PdClient;
use kvproto::kvrpcpb::Context;


use std::sync::Arc;

const CQ_COUNT: usize = 1;
const CLIENT_PREFIX: &str = "tikv-client";

pub struct Client {
    tikv: TikvClient,
    pd: Pdclient,
    env: Arc<Environment>,
    security_mgr: Arc<SecurityManager>,
    timeout: Duration,
    cluster_id: u64,
}

impl Client {
    fn new() -> Self {
        let env = Arc::new(EnvBuilder::new().build());
        let tikv_ch = ChannelBuilder::new(env.clone()).connect("172.16.5.31:39860");
        let pd_ch = ChannelBuilder::new(env.clone()).connect("172.16.5.31:38830");
        Client {
            kv_client: TikvClient::new(tikv_ch),
            pd_client：PdClient::new(tikv_ch),
            region_cache: ,
        }
    }

    fn get_regin(key: String) -> Region {

    }

    fn form(region: Region) -> Context { 
        let mut kvctx = kvrpcpb::Context::new();
        kvctx.set_region_id(region.id);
        kvctx.set_region_epoch(region.take_region_epoch());
        kvctx.set_peer(region.peer().expect("leader must exist").into_inner());
        kvctx 
    }

    pub fn split_region(key: String) -> bool {
        let region = self.get_regin(key.clone());
        let ctx = self.form(region);
        self.tikv.split_region(key,ctx);
    }

    pub fn raw_put(key: String,value: String) -> bool {
        let region = self.get_regin(key.clone());
        let ctx = self.form(region);
        self.tikv.raw_put(ctx,key,value);
    }
}


