use serde::{Deserialize, Serialize};

/// transfer transaction: from -> to amount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

impl Transaction {
    pub fn new(from: impl Into<String>, to: impl Into<String>, amount: u64) -> Self {
        Self { from: from.into(), to: to.into(), amount }
    }
}
