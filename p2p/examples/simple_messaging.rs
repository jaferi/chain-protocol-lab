use p2p::{LocalPeer, Message, Node};
use ed25519_dalek::Verifier;
use rand::{seq::SliceRandom};
use std::{thread, time::Duration};

fn main() {
    simple_message_test_from_node_a_to_b();
    simple_message_test_automatic_nodes();
}

fn simple_message_test_from_node_a_to_b() {
    let node_a = LocalPeer::generate();
    let node_b = LocalPeer::generate();

    let peer_a = node_a.peer_id();
    let peer_b = node_b.peer_id();

    println!("Node A ID: {:?}", peer_a.short());
    println!("Node B ID: {:?}", peer_b.short());

    let content = "Hello Node B!".to_string();
    let signature = node_a.sign(content.as_bytes());

    let msg = Message {
        from: peer_a.public_key.as_bytes().to_vec(),
        content: content.clone(),
        signature,
    };

    let pubkey = ed25519_dalek::PublicKey::from_bytes(&msg.from).unwrap();
    match pubkey.verify(msg.content.as_bytes(), &msg.signature) {
        Ok(_) => println!("✅ Node B verified message from Node A"),
        Err(_) => println!("❌ Verification failed"),
    }
}



fn simple_message_test_automatic_nodes() {
    let mut nodes: Vec<Node> = vec![];

    for _i in 1..100 {
        let node = Node::new();
        nodes.push(node);
    }

    let mut rng = rand::thread_rng();

    for _ in 0..2000 {
        // choose two nodes randomly
        let node_from: &Node = nodes.choose(&mut rng).unwrap();
        let node_to: &Node = nodes.choose(&mut rng).unwrap();

        // skip if two nodes are same
        if node_from.peer_id == node_to.peer_id { continue; }


        let peer_from = node_from.peer_id.clone();
        let peer_to = node_to.peer_id.clone();


        println!("Node Sender ID: {:?}", peer_from.short());
        println!("Node Receiver ID: {:?}", peer_to.short());

        let content: String = format!("Hello from Node {} to {}", peer_from.short(), peer_to.short()).to_string();
        let sender_signature = node_from.local.sign(content.as_bytes());

        let msg: Message = Message { 
            from: peer_from.public_key.as_bytes().to_vec(), 
            content: content.clone(),
            signature: sender_signature 
        };

        let pubkey = ed25519_dalek::PublicKey::from_bytes(&msg.from).unwrap();
        match pubkey.verify(msg.content.as_bytes(), &msg.signature) {
            Ok(_) => println!("✅ Node {} verified message from Node {}", peer_from.short(), peer_to.short()),
            Err(_) => println!("❌ Verification failed"),
        }

        thread::sleep(Duration::from_millis(500));

    }

}
