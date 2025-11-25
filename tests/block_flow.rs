#[cfg(test)]
mod tests {
    use core::Block;
    use core::state::State;
    use core::store::BlockStore;
    use core::tx::Transaction;

    use node::{apply_block, propose_block};
    use std::fs;

    #[test]
    fn block_roundtrip_serialization() {
        let txs = vec![
            Transaction::new("alice", "bob", 50),
            Transaction::new("carol", "dave", 20),
        ];

        let b = Block::new(None, 1, txs, [1u8; 32]);

        let encoded = bincode::serialize(&b).expect("failed to serialize block");
        let decoded: Block = bincode::deserialize(&encoded).expect("failed to deserialize block");

        assert_eq!(
            b, decoded,
            "Block should remain identical after serialize → deserialize"
        );
    }

    #[test]
    fn test_state_apply_and_root() {
        let mut s = State::new();
        s.apply_transfer("GENESIS", "alice", 1000).unwrap();
        s.apply_transfer("alice", "bob", 200).unwrap();
        assert_eq!(*s.accounts.get("alice").unwrap(), 800);
        assert_eq!(*s.accounts.get("bob").unwrap(), 200);
        let root = s.root();
        assert_ne!(root, [0u8; 32]);
    }

    #[test]
    fn test_propose_and_apply_block() {
        let mut s = State::new();
        s.apply_transfer("GENESIS", "alice", 1000).unwrap();
        let txs = vec![
            Transaction::new("alice", "bob", 300),
            Transaction::new("bob", "carol", 50),
        ];
        let proposed = propose_block(s.root(), None, 1, 1, txs.clone(), &s);
        // proposer computed state root deterministic
        let mut s2 = s.clone();
        apply_block(&proposed, &mut s2).expect("apply ok");
        assert_eq!(*s2.accounts.get("alice").unwrap(), 700);
        assert_eq!(*s2.accounts.get("bob").unwrap(), 250);
    }

    #[test]
    fn test_block_store_append_and_iter() {
        let path = "test_store.db";
        let _ = fs::remove_file(path);
        let mut store = BlockStore::open(path).expect("open store");
        let mut s = State::new();
        s.apply_transfer("GENESIS", "alice", 1000).unwrap();
        let txs = vec![Transaction::new("alice", "bob", 123)];
        let blk = propose_block(s.root(), None, 1, 1, txs.clone(), &s);
        store.append_block(&blk).unwrap();
        let blocks = store.iter_blocks().unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].transactions, txs);
        let _ = fs::remove_file(path);
    }
}
