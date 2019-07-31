use super::comment::RegionLeader;
use super::error::{Error, Result};
use grpcio::{ChannelBuilder, Environment};
use kvproto::pdpb::{GetMembersRequest, GetMembersResponse, GetRegionRequest, RequestHeader};
use kvproto::pdpb_grpc::PdClient;
use std::sync::Arc;
use std::time::Duration;

fn connect_pd_client(env: Arc<Environment>, addr: &str) -> Result<(PdClient, GetMembersResponse)> {
    let cb = ChannelBuilder::new(env)
        .keepalive_time(Duration::from_secs(10))
        .keepalive_timeout(Duration::from_secs(3));
    let channel = cb.connect(addr);

    let pd_client = PdClient::new(channel);
    let get_req = GetMembersRequest::new();
    let resp = pd_client.get_members(&get_req).unwrap();
    Ok((pd_client, resp))
}

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
        let mut cluster_id = None;
        for i in 0..100 {
            let (pd_client, resp) = match connect_pd_client(Arc::clone(&env), addr.clone()) {
                // connect this endpoint
                Ok(resp) => resp,
                // Ignore failed PD node.
                Err(e) => {
                    println!("failed to connect pd_client@[{}]", addr);
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

    pub fn get_regionleader(&self, key: Vec<u8>) -> RegionLeader {
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
            panic!("not get a region");
        };

        let leader = if res.has_leader() {
            Some(res.take_leader())
        } else {
            None
        };
        RegionLeader::new(region, leader)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pd_client() {
        let env = Arc::new(EnvBuilder::new().build());
        let pd = PDClient::new(env, "172.16.5.31:38830");
        println!("{:?}", pd);

        let key = "b".to_string().into_bytes();
        println!("{:?}", pd.get_region(key));
    }
}
