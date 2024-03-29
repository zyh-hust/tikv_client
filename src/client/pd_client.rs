use super::comment::RegionLeader;
use super::error::{Error, Result};
use grpcio::{ChannelBuilder, Environment};
use kvproto::pdpb::ScanRegionsRequest;
use kvproto::pdpb::{GetMembersRequest, GetMembersResponse, GetRegionRequest, RequestHeader};
use kvproto::pdpb_grpc::PdClient;
use std::sync::Arc;

fn connect_pd_client(env: Arc<Environment>, addr: &str) -> Result<(PdClient, GetMembersResponse)> {
    let cb = ChannelBuilder::new(env);
    let channel = cb.connect(addr);

    let pd_client = PdClient::new(channel);
    let get_req = GetMembersRequest::new();
    let resp = pd_client.get_members(&get_req).unwrap();
    Ok((pd_client, resp))
}

#[derive(Clone)]
pub struct PDClient {
    cluster_id: u64,
    pd: PdClient,
}

impl PDClient {
    pub fn new(env: Arc<Environment>, addr: &str) -> Result<PDClient> {
        let (pd_client, cluster_id) = PDClient::validate_endpoints(env, addr)?;
        Ok(PDClient {
            cluster_id: cluster_id,
            pd: pd_client,
        })
    }

    pub fn validate_endpoints(env: Arc<Environment>, addr: &str) -> Result<(PdClient, u64)> {
        for _i in 0..100 {
            let (pd_client, resp) = match connect_pd_client(Arc::clone(&env), addr.clone()) {
                // connect this endpoint
                Ok(resp) => resp,
                // Ignore failed PD node.
                Err(e) => {
                    println!("failed to connect pd_client@[{}],because of {:?}", addr, e);
                    continue;
                }
            };
            let cid = resp.get_header().get_cluster_id();
            return Ok((pd_client, cid));
        }
        Err(Error::PdError(format!(
            "unconnect to pd,pd addr is {:?}",
            addr
        )))
    }

    pub fn get_regionleader(&self, key: Vec<u8>) -> Result<RegionLeader> {
        let mut req = GetRegionRequest::new();
        let mut header = RequestHeader::new();
        header.set_cluster_id(self.cluster_id);
        req.set_header(header);
        req.set_region_key(key);
        // 通过rpc调用去获得当前key的region
        // TODO: handle error
        let mut res = self.pd.get_region(&req).unwrap();
        let region = if res.has_region() {
            res.take_region()
        } else {
            // TODO: find a better way to handle this scene
            return Err(Error::PdError(format!("get region error {:?}", res)));
        };

        let leader = if res.has_leader() {
            Some(res.take_leader())
        } else {
            None
        };
        Ok(RegionLeader::new(region, leader))
    }

    pub fn scan_regions(&self, start_key: Vec<u8>) -> bool {
        let mut req = ScanRegionsRequest::new();
        let mut header = RequestHeader::new();
        header.set_cluster_id(self.cluster_id);
        req.set_header(header);
        req.set_start_key(start_key);
        req.set_limit(5);
        let mut res = self.pd.scan_regions(&req).unwrap();
        println!(
            "{:?} \n {:?}\n {:?}",
            res.take_header(),
            res.take_regions(),
            res.take_leaders()
        );
        true
    }
}
