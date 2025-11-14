use serde::{Serialize, Deserialize};
use ed25519_dalek::{Signature, Verifier};
use crate::node::Node;
use rand::{seq::SliceRandom};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub from: Vec<u8>,        // serialized PeerId (public key)
    pub content: String,      // text message
    pub signature: Signature, // signed content
}


pub fn gossip_message(sender_index: usize, nodes: &mut Vec<Node>, content: &str) {
    let sender = &nodes[sender_index];

    let signature = sender.local.sign(content.as_bytes());

    let msg = Message {
        from: sender.peer_id.public_key.as_bytes().to_vec(),
        content: content.to_string(),
        signature,
    };

    let mut rng = rand::thread_rng();
    let mut peers_to_send = sender.known_peers.clone();
    peers_to_send.shuffle(&mut rng);
    peers_to_send.truncate(3);

    for peer_id in peers_to_send {
        // Find the index of the receiver
        if let Some(receiver_index) = nodes.iter().position(|n| n.peer_id == peer_id) {
            let pubkey = ed25519_dalek::PublicKey::from_bytes(&msg.from).unwrap();

            if pubkey.verify(msg.content.as_bytes(), &msg.signature).is_ok() {
                // Only mutate inside a separate borrow
                let already_received = nodes[receiver_index]
                    .received_msgs
                    .contains(&msg.content);
                if !already_received {
                    nodes[receiver_index]
                        .received_msgs
                        .push(msg.content.clone());
                    println!(
                        "Node {:?} received: {}",
                        nodes[receiver_index].peer_id.short(),
                        msg.content
                    );

                    // Recursive forward
                    gossip_message(receiver_index, nodes, &msg.content);
                }
            }
        }
    }
}
