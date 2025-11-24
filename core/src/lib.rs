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
}