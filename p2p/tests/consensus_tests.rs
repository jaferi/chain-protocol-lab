use p2p::node::Node;
use p2p::block::gossip_block;


#[test]
fn test_block_consensus() {
    // Create nodes
    let mut nodes: Vec<Node> = vec![];
    for _ in 0..7 {  // 7 nodes for majority testing
        nodes.push(Node::new());
    }

    // 2️⃣ Populate known peers for gossip
    let peer_ids: Vec<String> = nodes.iter().map(|n| n.peer_id.clone()).collect();
    for node in nodes.iter_mut() {
        node.known_peers = peer_ids
            .iter()
            .filter(|id| id != &node.peer_id)
            .cloned()
            .collect();
    }

    // 3️⃣ Node 0 proposes a block
    let mut block = nodes[0].propose_block(
        1,
        vec!["tx1".to_string(), "tx2".to_string()]
    );

    // 4️⃣ Gossip the block to all nodes
    gossip_block(&nodes[0], &mut nodes, &mut block);

    // 5️⃣ Check that majority of nodes voted
    let votes = block.votes.len();
    let majority = (nodes.len() / 2) + 1;

    println!("Block ID: {}, Votes: {}, Majority: {}", block.id, votes, majority);
    assert!(votes >= majority, "Block did not reach majority consensus");

    println!("✅ Consensus test passed for block {}", block.id);
}
