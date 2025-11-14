use p2p::node::Node;
use p2p::message::gossip_message;


fn main() {
    let mut nodes: Vec<Node> = vec![];
    for _ in 0..10 {
        nodes.push(Node::new());
    }

    // Populate known_peers for each node
    for i in 0..nodes.len() {
        nodes[i].known_peers = nodes
            .iter()
            .filter(|n| n.peer_id != nodes[i].peer_id)
            .map(|n| n.peer_id.clone())
            .collect();
    }

    // Start gossip from first node
    gossip_message(0, &mut nodes, "Hello gossip network!");
}
