pub type Hash = [u8; 32];

pub mod block;
pub mod state;
pub mod tx;
pub mod store;


pub use block::{Block, Header};
pub use tx::Transaction;
pub use state::{State, StateRoot};
pub use store::BlockStore;