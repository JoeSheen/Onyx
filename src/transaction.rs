use super::*;

pub struct Transaction {
    pub timestamp: u128,
    pub sender: PublicKey,
    pub reciever: PublicKey,
    pub amount: f32,
    pub signature: Option<Signature>,
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
            signature: None,
        };
    }

    // function that can be used to sign transactions
    pub fn sign_transaction(&mut self, keypair: Keypair) {
        if self.sender == keypair.public {
            self.signature = Some(keypair.sign(&self.hash()));
        } else {
            println!("Error: Failed to sign transaction");
        }
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
        // allows a signature to be included as part of the hash
        if let Some(signed) = self.signature {
            bytes.extend(&signed.to_bytes());
        }
        return bytes;
    }
}