use core::{Block, State, StateRoot, Transaction, Hash};

/// Propose: given parent, number, timestamp and transactions, compute new state_root
pub fn propose_block(_parent_root: StateRoot, parent_hash: Option<Hash>, number: u64, _timestamp: u64, txs: Vec<Transaction>, state: &State) -> Block {
    // For the MVP we will simulate applying txs to a copy of state to compute new root:
    let mut s = state.clone();
    for tx in &txs {
        // in propose we assume transaction validity; consensus will check
        s.apply_transfer(&tx.from, &tx.to, tx.amount).expect("propose apply");
    }
    let new_root = s.root();
    Block::new(parent_hash, number, txs, new_root)
}

/// Apply block: validate block parent/root and actually mutate state
pub fn apply_block(block: &Block, state: &mut State) -> Result<(), String> {
    // naive checks: apply transactions and compare computed root
    let mut s = state.clone();
    for tx in &block.transactions {
        s.apply_transfer(&tx.from, &tx.to, tx.amount)?;
    }
    let computed = s.root();
    if computed != block.header.state_root {
        return Err("state_root_mismatch".into());
    }
    // commit: replace state with mutated
    *state = s;
    Ok(())
}
