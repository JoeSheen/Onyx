use super::*;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub nonce: u128,
    pub hash: Vec<u8>,
    pub prev_hash: Vec<u8>,
    pub transactions: Vec<Transaction>,
}

impl Block {
    // function for creating a new block instance
    pub fn new(index: u32, prev: Vec<u8>, transactions: Vec<Transaction>) -> Block {
        let time = now();
        return Block {
            index: index,
            timestamp: time,
            nonce: 0,
            hash: vec![0; 32],
            prev_hash: prev,
            transactions: transactions,
        };
    }

    // function for mining a block
    pub fn mine_block(&mut self) {
        for nonce_val in 0..=(u128::MAX) {
            self.nonce = nonce_val;
            let hash = self.hash();
            if validate_hash(hash.clone()) == true {
                self.hash = hash;
                println!("--- Block Mined ---");
                return;
            }
        }
    }
}

// implementation of bytes function for Hash trait of block obj
impl Hash for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(&self.index.to_ne_bytes());
        bytes.extend(&self.timestamp.to_ne_bytes());
        bytes.extend(&self.nonce.to_ne_bytes());
        bytes.extend(&self.prev_hash);
        bytes.extend(&self.transactions
            .iter().flat_map(|transaction| transaction.bytes())
            .collect::<Vec<u8>>()
        );
        
        return bytes;
    }
}

// implementation of is_valid() function for Valid trait of block
impl Valid for Block {
    fn is_valid(&self) -> bool {
        for transaction in &self.transactions {
            if !transaction.is_valid() {
                return false;
            }
        }
        return true;
    }
}

// function to validate hash is correct
pub fn validate_hash(hash: Vec<u8>) -> bool {
    let hex_hash = hex::encode(hash);
    let target = generate_target_hash();
    if hex_hash.starts_with(&target) {
        return true;
    } else {
        return false;
    }
}

// implementation of fmt::Display for Block
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block[{}]:\n   - hash: {}\n   - timestamp: {}\n   - nonce: {}\n   - prev: {}\n   - transaction(s): {}", 
            self.index,
            hex::encode(self.hash.clone()),
            self.timestamp,
            self.nonce.clone(),
            hex::encode(self.prev_hash.clone()),
            self.transactions.len(),
        )
    }
}
