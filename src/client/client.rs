use grpcio::Environment;
use std::collections::HashMap;

use super::comment::RegionLeader;
use super::error::Result;
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

    pub fn get_region(&mut self, key: Vec<u8>) -> Result<RegionLeader> {
        if let Some(region) = self.region_cache.get(&key.clone()) {
            return Ok(region.clone());
        }
        let region_leader = match self.pd.get_regionleader(key.clone()) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        self.region_cache.insert(key, region_leader.clone());
        Ok(region_leader)
    }

    pub fn split_region(&mut self, key: Vec<u8>) -> bool {
        let region = match self.get_region(key.clone()) {
            Ok(reg) => reg,
            Err(err) => {
                println!("{:?}", err);
                return false;
            }
        };
        let ctx = region.form();
        let resp = match self.tikv.split_region(ctx, key.clone()) {
            Ok(resp) => resp,
            Err(err) => {
                println!("{:?}", err);
                return false;
            }
        };
        // TODO handle the response
        if resp.has_region_error() {
            self.remove(key);
            println!("split region resp :{:?}", resp);
            return false;
        }
        println!("split region resp :{:?}", resp);
        true
    }

    pub fn raw_put(&mut self, key: Vec<u8>, value: Vec<u8>) -> bool {
        let region = match self.get_region(key.clone()) {
            Ok(res) => res,
            Err(err) => {
                println!("{:?}", err);
                return false;
            }
        };
        let ctx = region.form();
        let resp = match self.tikv.raw_put(ctx, key.clone(), value) {
            Ok(resp) => resp,
            Err(err) => {
                println!("{:?}", err);
                return false;
            }
        };
        // TODO handle the response
        if resp.has_region_error() {
            println!("resp has region error :{:?}", resp);
            self.remove(key);
            return false;
        }
        true
    }

    pub fn scan_regions(&self, start_key: Vec<u8>) -> bool {
        self.pd.scan_regions(start_key)
    }
}
