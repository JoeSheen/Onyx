use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use ed25519_dalek::*;
use rand::rngs::OsRng;
mod transaction;
pub use crate::transaction::Transaction;
mod block;
pub use crate::block::Block;
mod blockchain;
pub use crate::blockchain::Blockchain;
mod wallet;
pub use crate::wallet::Wallet;

// const values used within crypto system
const MINING_REWARD: f32 = 100.00;
const DIFFICULTY: u128 = 2;
const BLOCKCHAIN_PUBLIC_KEY_ARRAY: [u8; 32] = [
    215,  90, 152,   1, 130, 177,  10, 183, 213,  75, 254, 211, 201, 100,   7,  58,
    14, 225, 114, 243, 218, 166,  35,  37, 175,   2,  26, 104, 247,   7,   81, 26
]; //d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a

// function for calculating the current time in milliseconds
pub fn now () -> u128 {
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
}

// Hash trait for obj hashing
pub trait Hash {
    fn hash(&self) -> Vec<u8> {
        return crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes());
    }

    fn bytes(&self) -> Vec<u8>;
}

// trait to validate that a blockchain object is correct
pub trait Valid {
    fn is_valid(&self) -> bool;
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

// function to convert the BLOCKCHAIN_PUBLIC_KEY_ARRAY into PublicKey
pub fn generate_blockchain_key(bpk: &[u8; 32]) -> PublicKey {
    let key = match PublicKey::from_bytes(bpk) {
        Ok(key) => key,
        Err(error) => panic!("Problem with blockchain public key: {:?}", error),
    };

    return key;
}
