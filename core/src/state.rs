use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::Hash;
use bincode;
use blake3;

pub type StateRoot = Hash;

/// small deterministic state: BTreeMap of account -> balance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub accounts: BTreeMap<String, u128>,
}

impl State {
    pub fn new() -> Self {
        Self { accounts: BTreeMap::new() }
    }

    pub fn apply_transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let amt = amount as u128;
        // debit
        if from != "GENESIS" {
            let from_bal = self.accounts.get(from).cloned().unwrap_or(0);
            if from_bal < amt {
                return Err("insufficient_balance".into());
            }
            self.accounts.insert(from.to_string(), from_bal - amt);
        }
        // credit
        let to_bal = self.accounts.get(to).cloned().unwrap_or(0);
        self.accounts.insert(to.to_string(), to_bal + amt);
        Ok(())
    }

    /// deterministic canonical state root 
    pub fn root(&self) -> StateRoot {
        let ser = bincode::serialize(&self).expect("serialize state");
        let digest = blake3::hash(&ser);
        let mut out = [0u8; 32];
        out.copy_from_slice(digest.as_bytes());
        out
    }
}
