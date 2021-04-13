use std::time::{SystemTime, UNIX_EPOCH};
mod transaction;
pub use crate::transaction::Transaction;
mod block;
pub use crate::block::Block;
mod blockchain;
pub use crate::blockchain::Blockchain;

// const values used within crypto system
const MINING_REWARD: f32 = 100.00;
const DIFFICULTY: u128 = 2;

// function for calculating the current time
pub fn now () -> u128 {
    let duration_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time = duration_time.as_secs() as u128 * 1000 + duration_time.subsec_millis() as u128;
    //let time = duration_time.as_millis();
    return time;
}

// Hash trait for obj hashing
pub trait Hash {
    fn hash(&self) -> Vec<u8>{
        return crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes());
    }

    fn bytes(&self) -> Vec<u8>;
}

// function that generates the target hash prefox for each block being validated
// e.g. 3 leading zeroes
pub fn generate_target_hash() -> String {
    let mut target: String = String::new();
    for _i in 0..=DIFFICULTY {
        target.push_str("0");
    }
    return target;
}