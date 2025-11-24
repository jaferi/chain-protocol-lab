use core::Block; 

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
