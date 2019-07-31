use kvproto::kvrpcpb::Context;
use kvproto::metapb::{Peer, Region};

#[derive(Clone)]
pub struct RegionLeader {
    pub region: Region,
    pub leader: Option<Peer>,
}

impl RegionLeader {
    pub fn new(region: Region, leader: Option<Peer>) -> Self {
        RegionLeader { region, leader }
    }

    pub fn id(&self) -> u64 {
        self.region.get_id()
    }

    pub fn form(&self) -> Context {
        let mut kvctx = Context::new();
        kvctx.set_region_id(self.id());
        kvctx.set_region_epoch(self.region.get_region_epoch().clone());
        kvctx.set_peer(self.peer());
        kvctx
    }

    pub fn peer(&self) -> Peer {
        if let Some(peer) = self.leader {
            return peer.clone();
        }
        panic!("no leader peer");
    }
}
