use grpcio::Environment;
use std::collections::HashMap;

use super::comment::RegionLeader;
use super::kv_client::KvClient;
use super::pd_client::PDClient;
use std::sync::Arc;

pub struct Client {
    tikv: KvClient,
    pd: PDClient,
    region_cache: HashMap<Vec<u8>, RegionLeader>,
}

impl Client {
    pub fn new(env: Arc<Environment>) -> Self {
        let pd_client = PDClient::new(env, "172.16.5.31:38830").unwrap();
        let tikv_client = KvClient::new(env, "172.16.5.31:39860").unwrap();
        Client {
            tikv: tikv_client,
            pd: pd_client,
            region_cache: HashMap::new(),
        }
    }

    pub fn get_region(&self, key: Vec<u8>) -> RegionLeader {
        if let Some(region) = self.region_cache.get(&key.clone()) {
            return region.clone();
        }
        let region_leader = self.pd.get_regionleader(key.clone());
        self.region_cache.insert(key, region_leader.clone());
        region_leader
    }

    pub fn split_region(&self, key: Vec<u8>) {
        let region = self.get_region(key.clone());
        let ctx = region.form();
        let resp = self.tikv.split_region(ctx, key);
        println!("split region resp :{:?}", resp);
    }

    pub fn raw_put(&self, key: Vec<u8>, value: Vec<u8>) {
        let region = self.get_region(key.clone());
        let ctx = region.form();
        let resp = self.tikv.raw_put(ctx, key, value);
        println!("put_raw resp :{:?}", resp);
    }
}
