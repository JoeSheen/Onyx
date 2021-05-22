use super::*;

pub struct Wallet {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl Wallet {
    // function for creating a new wallet (contains pub/sec keys)
    pub fn new() -> Wallet {
        let keypair: Keypair = Keypair::generate(&mut OsRng);
        let wallet: Wallet = Wallet {
            public_key: keypair.public,
            secret_key: keypair.secret,
        };
        register_mining_addr(wallet.public_key);  // adds public key to MINING_ADDR
        return wallet;
    }
}