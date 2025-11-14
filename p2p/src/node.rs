use crate::peer::{PeerId, LocalPeer};
use crate::block::Block;

pub struct Node {
    pub local: LocalPeer,
    pub peer_id: PeerId,
    pub known_peers: Vec<PeerId>,
    pub received_msgs: Vec<String>   
}


impl Node {
    pub fn new() -> Self {
        let local = LocalPeer::generate();
        let peer_id = local.peer_id();
        
        let known_peers: Vec<PeerId> = vec![];
        let received_msgs: Vec<String> = vec![];

        Node {local, peer_id, known_peers, received_msgs}
    }

    pub fn propose_block(&self, id: u32, messages: Vec<String>) -> Block {
        Block::new(id, messages, self.peer_id.clone())
    }
}