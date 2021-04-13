use onyxlib::*;
fn main() {
    println!("--- START ---");
    let mut blockchain = Blockchain::new();
    let t0 = Transaction::new("Sender".to_owned(), "Reciever".to_owned(), 120.00);
    let t1 = Transaction::new("Bob".to_owned(), "Alice".to_owned(), 85.70);
    let mut genesis_block = Block::new(0, vec![0; 32], vec![t0, t1]);

    genesis_block.mine_block();

    println!(" Block[{:?}]:\n   hash: {:?}\n   timestamp: {:?}\n   nonce: {:?}\n   prev: {:?}\n   tran-len: {:?}", 
            genesis_block.index,
            hex::encode(genesis_block.hash.clone()),
            genesis_block.timestamp,
            genesis_block.nonce.clone(),
            hex::encode(genesis_block.prev_hash.clone()),
            genesis_block.transactions.len(),
        );

    blockchain.validate_block(genesis_block);
    
    let mut prev_hash: Vec<u8> = vec![0; 32];

    for index in 1..=9 {
        let b = blockchain.last_block();
        match b {
            Some(block) => prev_hash = block.hash.clone(),
            None => println!("Error: failed to find last block hash"),
        }
        let mut next_block = Block::new(index, prev_hash.clone(), vec![Transaction::new("sender".to_owned(), "reciever".to_owned(), 10.99)]);
        next_block.mine_block();

        println!(" Block[{:?}]:\n   hash: {:?}\n   timestamp: {:?}\n   nonce: {:?}\n   prev: {:?}\n   tran-len: {:?}", 
            index,
            hex::encode(next_block.hash.clone()),
            next_block.timestamp,
            next_block.nonce.clone(),
            hex::encode(next_block.prev_hash.clone()),
            next_block.transactions.len(),
        );
        blockchain.validate_block(next_block);
    }
    println!("------\nchain len: {:?}", blockchain.chain.len());
    println!("--- END ---");
    //todo!("\n **** Findout about public/private key pairs in rust ****\n")
}
