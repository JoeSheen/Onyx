use super::*;

pub struct Transaction {
    pub timestamp: u128,
    pub sender: PublicKey,
    pub reciever: PublicKey,
    pub amount: f32,
}

impl Transaction {
    // function for creating a new transaction
    pub fn new(sender_key: PublicKey, reciever_key: PublicKey, amt: f32) -> Transaction {
        let time = now();
        return Transaction {
            timestamp: time,
            sender: sender_key,
            reciever: reciever_key,
            amount: amt,
        };
    }
}

// implementation of bytes function for Hash trait of transaction obj
impl Hash for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(&self.timestamp.to_ne_bytes());
        bytes.extend(&self.sender.to_bytes());
        bytes.extend(&self.reciever.to_bytes());
        bytes.extend(&self.amount.to_ne_bytes());
        
        return bytes;
    }
}