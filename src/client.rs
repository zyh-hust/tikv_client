use grpcio::{ChannelBuilder,EnvBuilder},
use kvproto::kvrpcpb::{RawPutRequest, SplitRegionRequest};
use kvproto::tikvpb_grpc::TikvClient;
use kvproto::pdpd::GetRegionRequest;
use kvproto::pdpd_grpc::PdClient;

use std::sync::Arc;

pub struct Client {
    kv_client: TikvClient,
    pd_client: Pdclient,
    region_cache: 
}


