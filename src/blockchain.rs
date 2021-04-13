use super::*;

pub struct Blockchain{
    pub chain: Vec<Block>,
    pub pending_transactions : Vec<Transaction>,
    pub mining_reward: f32,
}

impl Blockchain {
    pub fn new() -> Blockchain{
        return Blockchain {
            chain: vec![],
            pending_transactions : vec![],
            mining_reward: MINING_REWARD,
        };
    }

    //Test function for checking block works correctly 
    pub fn validate_block(&mut self, block: Block) {
        if block.hash == vec![0; 32] {
            return;
        } else {
            self.chain.push(block);
            return;
        }
    }

    pub fn mine_pending_transactions(&mut self, miner_addr: String) {
        let _ma = miner_addr;
    }

    //pub fn add_transaction(self, sender, reciever, amt, keyString, senderKey) {}
    //add_genesis_block

    pub fn last_block(&self) -> Option<&Block> {
        return self.chain.last().clone();
    }
}

//todo!(add a way to register new node/wallet/etc.)
