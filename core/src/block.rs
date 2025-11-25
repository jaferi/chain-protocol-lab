use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{Hash, Transaction};

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
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(parent: Option<Hash>, number: u64, transactions: Vec<Transaction>, state_root: Hash) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            header: Header { parent, number, timestamp, state_root },
            transactions,
        }
    }

    pub fn hash(&self) -> Hash {
        use bincode::Options;
        let encoded = bincode::DefaultOptions::new().serialize(self).expect("serialize");
        let out = blake3::hash(&encoded).as_bytes().clone();
        let mut h = [0u8; 32];
        h.copy_from_slice(&out);
        h
    }
}
