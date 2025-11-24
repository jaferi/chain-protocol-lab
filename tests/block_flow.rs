
use core::Block;

#[test]
fn block_roundtrip_serialization() {
    let b = Block::new(None, 1, vec![b"hello".to_vec()], [1u8;32]);

    let encoded = bincode::serialize(&b).unwrap();
    let decoded: Block = bincode::deserialize(&encoded).unwrap();

    assert_eq!(b, decoded);
}
