use super::*;

pub struct Transaction {
    pub timestamp: u128
}

impl Transaction {
    pub fn new() -> Transaction{
        let time = now();
        return Transaction{
            timestamp: time,
        };
    }
}