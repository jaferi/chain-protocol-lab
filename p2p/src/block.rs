use crate::peer::PeerId;
use crate::node::Node;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u32,
    pub messages: Vec<String>,
    pub proposer: PeerId,
    pub votes: HashSet<PeerId>,
}

impl Block {
    pub fn new(id: u32, messages: Vec<String>, proposer: PeerId) -> Self {
        Self {
            id,
            messages,
            proposer,
            votes: HashSet::new(),
        }
    }

    pub fn add_vote(&mut self, voter: PeerId) {
        self.votes.insert(voter);
    }

    pub fn is_finalized(&self, total_nodes: usize) -> bool {
        // simple majority rule
        self.votes.len() > total_nodes / 2
    }
}


pub fn gossip_block(sender: &Node, nodes: &mut Vec<Node>, block: &mut Block) {
    // sender votes for its own block
    block.add_vote(sender.peer_id.clone());

    for peer in nodes.iter_mut() {
        if peer.peer_id != sender.peer_id {
            // simulate vote
            block.add_vote(peer.peer_id.clone());

            // print status
            println!(
                "Node {:?} voted for block {} by {:?}",
                peer.peer_id.short(),
                block.id,
                block.proposer.short()
            );
        }
    }

    // check if block is finalized
    if block.is_finalized(nodes.len()) {
        println!("✅ Block {} finalized with majority!", block.id);
    } else {
        println!("❌ Block {} not yet finalized", block.id);
    }
}
