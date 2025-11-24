use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub type Hash = [u8; 32];

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Header {
    pub parent: Option<Hash>,
    pub number: u64,
    pub timestamp: u64,
    pub state_root: Hash,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Vec<u8>>, // opaque tx bytes for now
}

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub type Hash = [u8; 32];

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Header {
    pub parent: Option<Hash>,
    pub number: u64,
    pub timestamp: u64,
    pub state_root: Hash,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Vec<u8>>, // opaque tx bytes for now
}

impl Block {
    pub fn new(parent: Option<Hash>, number: u64, transactions: Vec<Vec<u8>>, state_root: Hash) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            header: Header { parent, number, timestamp, state_root },
            transactions,
        }
    }

    pub fn hash(&self) -> Hash {
        use bincode::Options;
        let encoded = bincode::DefaultOptions::new().serialize(self).expect("serialize");
        let mut out = blake3::hash(&encoded).as_bytes().clone();
        let mut h = [0u8; 32];
        h.copy_from_slice(&out);
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_hash_changes_on_content() {
        let b1 = Block::new(None, 0, vec![b"tx1".to_vec()], [0u8;32]);
        let b2 = Block::new(None, 0, vec![b"tx2".to_vec()], [0u8;32]);
        assert_ne!(b1.hash(), b2.hash());
    }
}
