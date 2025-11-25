use core::{Block, Transaction};

#[test]
fn block_hash_changes_when_transactions_differ() {
    let tx1 = Transaction::new("alice", "bob", 10);
    let tx2 = Transaction::new("alice", "bob", 20); // different amount → different content

    let b1 = Block::new(
        None,
        0,
        vec![tx1],
        [0u8; 32],
    );

    let b2 = Block::new(
        None,
        0,
        vec![tx2],
        [0u8; 32],
    );

    assert_ne!(
        b1.hash(),
        b2.hash(),
        "Blocks with different transactions must produce different hashes"
    );
}
