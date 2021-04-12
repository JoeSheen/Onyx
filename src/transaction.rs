use super::*;

pub struct Transaction {
    pub timestamp: u128,
    pub sender: String,
    pub reciever: String,
    pub amount: f32,
}

impl Transaction {
    // function for creating a new transaction
    pub fn new(sender: String, reciever: String, amt: f32) -> Transaction {
        let time = now();
        return Transaction {
            timestamp: time,
            sender: sender,
            reciever: reciever,
            amount: amt,
        };
    }
}

// implementation of bytes function for Hash trait of transaction obj
impl Hash for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(&self.timestamp.to_ne_bytes());
        bytes.extend(&self.sender.clone().into_bytes());
        bytes.extend(&self.reciever.clone().into_bytes());
        bytes.extend(&self.amount.to_ne_bytes());
        
        return bytes;
    }
}