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
it 