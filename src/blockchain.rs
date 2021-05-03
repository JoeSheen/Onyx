use super::*;
//TODO coment and finish code

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions : Vec<Transaction>,
    pub mining_reward: f32,
    pub bpk: [u8; 32], //bpk = blockchain
    pub id: u32,
}

impl Blockchain {
    //
    pub fn new() -> Blockchain {
        return Blockchain {
            chain: vec![],
            pending_transactions : vec![],
            mining_reward: MINING_REWARD,
            bpk: BLOCKCHAIN_PUBLIC_KEY_ARRAY,
            id: 0,
        };
    }

    //
    pub fn mine_pending_transactions(&mut self, miner_key: PublicKey) {
        // adds genesis block if chain is empty
        if self.chain.len() == 0 {
            self.add_genesis_block();
        }
        
        // prevents a miner from being rewarded for mining an empty pending_transactions
        if self.pending_transactions.len() == 0 {
            return;
        }

        // creates the mining reward transaction (blockchain -> miner)
        self.pending_transactions.push(
            Transaction::new(generate_blockchain_key(&self.bpk), miner_key, self.mining_reward)
        );

        // creates and mines block
        let mut prev_hash: Vec<u8> = vec![0; 32];
        let prev_block: Option<&Block> = self.chain.last();
        match prev_block {
            Some(block) => prev_hash = block.hash.clone(),
            None => println!("Error: failed to find last block hash"), //panic!()
        }
        
        let mut block: Block = Block::new(self.increment_id(), prev_hash, self.pending_transactions.to_vec());
        block.mine_block();

        //checks block to see if its invalid
        if block.is_valid() != true {
            panic!("Error: failed to validate block") 
        }

        // pushes block to blockchain and perfroms cleanup on pending_transactions
        self.chain.push(block);
        self.pending_transactions.clear();
    }

    //TODO -> finish
    pub fn add_transaction(&mut self, transaction: Transaction) {
        // adds genesis block if the blockchain is empty
        if self.chain.len() == 0 {
            self.add_genesis_block();
        }

        /*if transaction.is_valid() != true {
            return;
        }*/

        self.pending_transactions.push(transaction);
    }

    //
    fn add_genesis_block(&mut self) {
        let mut genesis_block: Block = Block::new(self.id, vec![0; 32], vec![]);
        genesis_block.mine_block();
        self.chain.push(genesis_block);
    }

    //
    fn increment_id(&mut self) -> u32 {
        self.id = self.id + 1;
        return self.id;
    }
}

// implementation of is_valid function from Valid trait for blockchain
impl Valid for Blockchain {
    fn is_valid(&self) -> bool {
        let block_vec = &self.chain;
        for (i, block) in block_vec.iter().enumerate() {
            if block.hash != block.hash() {
                return false;
            }
            if i > 0 { // genesis block doesnt need this check
                if block.index <= block_vec[i - 1].index {
                    return false;
                } else if block.prev_hash != block_vec[i - 1].hash {
                    return false;
                }
            }
            if !block.is_valid() {
                return false;
            }
        }
        return true;
    }
}
