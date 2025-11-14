use p2p::node::Node;
use p2p::block::gossip_block;


fn main() {
    let mut nodes: Vec<Node> = vec![];
    for _ in 0..5 {
        nodes.push(Node::new());
    }

    // Create a block from the first node
    let mut block = nodes[0].propose_block(1, vec!["msg1".to_string(), "msg2".to_string()]);

    // Gossip block for voting
    let first_ptr = &nodes[0] as *const Node;
    // Use a raw pointer to avoid simultaneous immutable/mutable borrows of `nodes`.
    // Safety: ensure `nodes` is not reallocated or the element moved while `gossip_block` runs.
    let first_ref: &Node = unsafe { &*first_ptr };
    gossip_block(first_ref, &mut nodes, &mut block);
}
