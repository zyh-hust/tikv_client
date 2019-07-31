use grpcio::{ChannelBuilder,EnvBuilder},
use kvproto::kvrpcpb::{RawPutRequest, SplitRegionRequest,Context};
use kvproto::tikvpb_grpc::TikvClient;
use kvproto::pdpd::GetRegionRequest;
use kvproto::pdpd_grpc::PdClient; 
use std::collections::HashMap;
use kvproto::metapb::Region;

use std::sync::Arc;

const CQ_COUNT: usize = 1;
const CLIENT_PREFIX: &str = "tikv-client";

pub struct Client {
    tikv: TikvClient,
    pd: Pdclient,   
    region_cache: HashMap<Vec<u8>,Region>,
}

impl Client {
    fn new(env: Arc<Environment>) -> Self { 
        let pd_client = PDClient::new(env,"172.16.5.31:38830");
        let tikv_client = KvClient::new(env,"172.16.5.31:39860").unwrap(); 
        Client {
            kv_client: tikv_client,
            pd_client：pd_client,
            region_cache: hashMap::new(),
        }
    }

    fn get_region(key: Vec<u8>) -> Region {
        if let Some(region) = region_cache.get(&key.clone()) {
            return region;
        }
        let region = self.pd.get_region(key.clone());
        region_cache.insert(key,region.clone());
        region
    }

    fn form(region: Region) -> Context { 
        let mut kvctx = kvrpcpb::Context::new();
        kvctx.set_region_id(region.id);
        kvctx.set_region_epoch(region.take_region_epoch());
        kvctx.set_peer(region.peer().expect("leader must exist").into_inner());
        kvctx 
    }

    pub fn split_region(key: Vec<u8>) -> bool {
        let region = self.get_region(key.clone());
        let ctx = self.form(region);
        let resp = self.tikv.split_region(key,ctx);
    }

    pub fn raw_put(key: Vec<u8>,value: Vec<u8>) -> bool {
        let region = self.get_region(key.clone());
        let ctx = self.form(region);
        let resp = self.tikv.raw_put(ctx,key,value);
    }
}


