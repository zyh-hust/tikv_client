use std:: sync::Arc,
use std::time::Duration;
use grpcio::{Environment，ChannelBuilder};
use kvproto::pdpb::{GetMembersRequest, GetRegionRequest, RequestHeader};
use super::error::{Error, Result};
use kvproto::metapb::Region;

fn connect_pd_client(
    env: Arc<Environment>,
    addr: &str,
) -> Result<(PdClient, GetMembersResponse)> { 
    let cb = ChannelBuilder::new(env)
        .keepalive_time(Duration::from_secs(10))
        .keepalive_timeout(Duration::from_secs(3));
    let channel = cb.connect(addr);

    let pd_client = PdClient::new(channel); 
    let get_req = GetMembersRequest::new();
    let resp = pd_client.get_members(&get_req).unwrap();
    Ok((pd_client, resp))
}

pub struct PdClient{
    cluster_id: u64,
    pd: PdClient,
}

impl PDClient{
    pub fn new(env: Arc<Environment>, addr: &str) -> Result<PDClient>{
        let (pd_client,cluster_id) = slef.validate_endpoints(env,addr)?;
        Ok(PDClient {
            cluster_id: cluster_id,
            pd: pd_client,
        })
    }

    pub fn validate_endpoints(
        env: &Arc<Environment>,
        addr: &str
    ) -> Result<(PdClient, cluster_id: u64)> { 
        let mut cluster_id = None;
        for i in 100 {
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
            return Ok(pd_client,cid);
        }
        Err(Error::PdError("unconnected to addr {:?}",addr));
    }

    fn get_region(&self, key: Vec<u8>) -> Region {
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
        }else{
            // TODO: find a better way to handle this scene
            panic!("not get a region");
        };
        region
    }
}