use onyxlib::*;
use ed25519_dalek::*;

fn main() {
    
    println!("--- START ---");

    let mut w1: Wallet = Wallet::new();
    let mut w2: Wallet = Wallet::new();

    let mut coin: f32 = 500.00;

    for i in 1..4 {
        w1.store(coin);
        w2.store(coin * 2.00);
        println!("w1 coins: {:?}", w1.coin_store);
        println!("w2 coins: {:?}", w2.coin_store);
        coin = coin - (10.0 * i as f32);
    }

    let mut blockchain = Blockchain::new();
    let mut t0: Transaction = Transaction::new(w1.public_key, w2.public_key, 120.00);
    
    t0.sign_transaction(
        Keypair {
            public: w1.public_key,
            secret: w1.secret_key,
        }
    );
    
    if let Some(signature) = t0.signature {
        println!("t0 signature: {:?}", hex::encode(signature));
    } else {
        println!("--- No signature ---");
    }

    let t1 = Transaction::new(w1.public_key, w2.public_key, 85.70);
    let mut genesis_block = Block::new(0, vec![0; 32], vec![t0, t1]);

    genesis_block.mine_block();

    println!("Block[{:?}]:\n   - hash: {:?}\n   - timestamp: {:?}\n   - nonce: {:?}\n   - prev: {:?}\n   - tran_len: {:?}", 
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
        let w3: Wallet = Wallet::new();
        let w4: Wallet = Wallet::new();
        
        let b = blockchain.last_block();
        match b {
            Some(block) => prev_hash = block.hash.clone(),
            None => println!("Error: failed to find last block hash"),
        }

        let mut next_block = Block::new(index, prev_hash.clone(), vec![Transaction::new(w3.public_key, w4.public_key, 10.99)]);
        next_block.mine_block();

        println!("Block[{:?}]:\n   - hash: {:?}\n   - timestamp: {:?}\n   - nonce: {:?}\n   - prev: {:?}\n   - tran_len: {:?}", 
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
}
