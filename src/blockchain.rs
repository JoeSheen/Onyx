use super::*;

pub struct Blockchain{
    pub chain: Vec<Block>,
    pub mining_reward: f32,
}

impl Blockchain {
    pub fn new() -> Blockchain{
        return Blockchain{
            chain: vec![],
            mining_reward: MINING_REWARD,
        };
    }

    //pub fn validate_block(){}

    pub fn last_block(&self) -> Option<&Block> {
        return self.chain.last().clone();
    }
}