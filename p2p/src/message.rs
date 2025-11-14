use serde::{Serialize, Deserialize};
use ed25519_dalek::Signature;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub from: Vec<u8>,        // serialized PeerId (public key)
    pub content: String,      // text message
    pub signature: Signature, // signed content
}

