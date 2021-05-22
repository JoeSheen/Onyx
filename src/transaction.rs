use super::*;

#[derive(Clone)]
pub struct Transaction {
    pub timestamp: u128,
    pub sender: PublicKey,
    pub reciever: PublicKey,
    pub amount: f32,
    pub signature: Option<Signature>,
    pub hash: Vec<u8>,
}

impl Transaction {
    // function for creating a new transaction
    pub fn new(sender_key: PublicKey, reciever_key: PublicKey, amt: f32) -> Transaction {
        let time: u128 = now();
        let mut transaction: Transaction = Transaction {
            timestamp: time,
            sender: sender_key,
            reciever: reciever_key,
            amount: amt,
            signature: None,
            hash: vec![0; 32],
        };
        transaction.hash = transaction.hash();
        return transaction;
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

// implementation of is_valid function for valid trait for transactions
impl Valid for Transaction {
    fn is_valid(&self) -> bool {
        if self.hash != vec![0; 32] {
            return true;
        } else {
            return false;
        }
    }
}

// implementation of fmt::Display for Transaction
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "      - timestamp: {}\n      - sender: {}\n      - reciever: {}\n      - amount: {}\n      - signature: {}\n      - hash: {}", 
            self.timestamp,
            hex::encode(self.sender),
            hex::encode(self.reciever),
            self.amount,
            if let Some(signature) = self.signature {
                hex::encode(signature)
            } else {
                "None".to_owned()
            },
            hex::encode(self.hash.clone()),
        )
    }
}
