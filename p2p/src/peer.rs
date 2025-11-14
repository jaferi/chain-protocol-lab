use core::str;

use ed25519_dalek::{Keypair, PublicKey, Signature, Signer};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PeerId {
    // The identity of a node
    // Safe to share on the network
    pub public_key: PublicKey,
}


pub struct LocalPeer {
    // it should be secured and private
    pub keypair: Keypair,
}

pub struct Node {
    pub peer: LocalPeer,
    pub id: PeerId,
}

impl LocalPeer {
    // Create a new identity (keypair) for a node.
    pub fn generate() -> Self {
        let mut csprng = OsRng::default();

        let keypair = Keypair::generate(&mut csprng);
        Self { keypair }
    }

    // Convert this local peer into the public PeerId (safe to share)
    pub fn peer_id(&self) -> PeerId {
        PeerId {
            public_key: self.keypair.public.clone(),
        }
    }

    // Sign a message using the node’s secret key.
    // Consensus protocols use this (BFT, Tendermint, HotStuff, PoS).
    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.keypair.sign(msg)
    }
}

impl PeerId {
    // human-readable ID (for debugging)
    pub fn short(&self) -> String {
        let bytes = self.public_key.as_bytes();
        format!("{:02x}{:02x}{:02x}{:02x}...", bytes[0], bytes[1], bytes[2], bytes[3])
    }
}

impl Node {
    pub fn new() -> Self {
        let peer = LocalPeer::generate();
        let id = peer.peer_id();
        Node {peer, id}
    }
}