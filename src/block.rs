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
        return Block{
            index: index,
            timestamp: time,
            nonce: 0,
            hash: vec![0; 32],
            prev_hash: prev,
            transactions: transactions,
        };
    }

    // function for mineing a block
    pub fn mine_block(&mut self) {
        //TODO: Finish
    }
}

// implementation of bytes func for Hash trait of block obj
impl Hash for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(&self.index.to_ne_bytes());
        bytes.extend(&self.timestamp.to_ne_bytes());
        bytes.extend(&self.nonce.to_ne_bytes());
        bytes.extend(&self.prev_hash);
        // TODO: add transactions -> bytes.extend(&self.transactions);

        return bytes;
    }
}