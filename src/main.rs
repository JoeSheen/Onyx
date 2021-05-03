use onyxlib::*;
use ed25519_dalek::*;

fn main() {
    println!("--- START ---");

    let mut blockchain: Blockchain = Blockchain::new();

    let w1: Wallet = Wallet::new();
    let w2: Wallet = Wallet::new();
    let w3: Wallet = Wallet::new();

    let mut t0: Transaction = Transaction::new(w1.public_key, w2.public_key, 500.50);
    let t1: Transaction = Transaction::new(w2.public_key, w3.public_key, 85.70);

    t0.sign_transaction(
        Keypair {
            public: w1.public_key,
            secret: w1.secret_key,
        }
    );

    blockchain.add_transaction(t0);
    blockchain.add_transaction(t1);
    
    let w4: Wallet = Wallet::new();
    let mut t3: Transaction = Transaction::new(w4.public_key, w1.public_key, 10.00);

    t3.sign_transaction(
        Keypair {
            public: w4.public_key,
            secret: w4.secret_key,
        }
    );

    blockchain.mine_pending_transactions(w4.public_key);

    blockchain.add_transaction(t3);

    blockchain.mine_pending_transactions(w1.public_key);

    /*for _i in 0..2 {
        let w5: Wallet = Wallet::new();
        let w6: Wallet = Wallet::new();
        let mut t4: Transaction = Transaction::new(w5.public_key, w6.public_key, 185.70);

        t4.sign_transaction(
            Keypair {
                public: w5.public_key,
                secret: w5.secret_key,
            }
        );

        blockchain.add_transaction(t4);
    }*/

    blockchain.mine_pending_transactions(w2.public_key);

    for block in &blockchain.chain {
        println!("{}", block);
        //println!("   - is_valid: {:?}", next_block.is_valid());
        for transaction in &block.transactions {
            println!("{}", transaction);
        }
    }

    println!("------\nblockchain length: {:?}", blockchain.chain.len());
    //println!("Is blockchain valid? {}", blockchain.is_valid());

    println!("--- END ---");
}
