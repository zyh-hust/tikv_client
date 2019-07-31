use grpcio::Environment;
use std::collections::HashMap;

use super::comment::RegionLeader;
use super::kv_client::KvClient;
use super::pd_client::PDClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct Client {
    tikv: KvClient,
    pd: PDClient,
    region_cache: HashMap<Vec<u8>, RegionLeader>,
}

impl Client {
    pub fn new(env: Arc<Environment>) -> Self {
        let pd_client = PDClient::new(env.clone(), "172.16.5.31:38800");
        if let Err(err) = pd_client {
            println!("connect to pd failed,err is :{:?}", err);
            panic!("0");
        }
        let tikv_client = KvClient::new(env.clone(), "172.16.5.31:39860");
        if let Err(err) = tikv_client {
            println!("connect to tikv failed,err is :{:?}", err);
            panic!("1");
        }

        Client {
            tikv: tikv_client.unwrap(),
            pd: pd_client.unwrap(),
            region_cache: HashMap::new(),
        }
    }

    pub fn remove(&mut self, key: Vec<u8>) {
        self.region_cache.remove(&key);
    }

    pub fn get_region(&mut self, key: Vec<u8>) -> RegionLeader {
        if let Some(region) = self.region_cache.get(&key.clone()) {
            return region.clone();
        }
        let region_leader = self.pd.get_regionleader(key.clone());
        self.region_cache.insert(key, region_leader.clone());
        region_leader
    }

    pub fn split_region(&mut self, key: Vec<u8>) -> bool {
        let region = self.get_region(key.clone());
        let ctx = region.form();
        let resp = self.tikv.split_region(ctx, key.clone());
        println!("split region resp :{:?}", resp);
        // TODO handle the response
        if resp.has_region_error() {
            self.remove(key);
            return false;
        }
        println!("split region resp :{:?}", resp);
        true
    }

    pub fn raw_put(&mut self, key: Vec<u8>, value: Vec<u8>) -> bool {
        let region = self.get_region(key.clone());
        let ctx = region.form();
        let resp = self.tikv.raw_put(ctx, key.clone(), value);
        println!("put_raw resp :{:?}", resp);
        // TODO handle the response
        if resp.has_region_error() {
            self.remove(key);
            return false;
        }
        println!("put_raw resp :{:?}", resp);
        true
    }

    pub fn scan_regions(&mut self, start_key: Vec<u8>) -> bool {
        self.pd.scan_regions(start_key)
    }
}
