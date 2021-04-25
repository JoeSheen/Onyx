use super::*;

pub struct Wallet {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
    pub coin_store: Vec<f32>, // create coin.rs file and add to project
}

impl Wallet {
    // function for creating a new wallet (contains pub/sec keys)
    pub fn new() -> Wallet {
        let keypair: Keypair = Keypair::generate(&mut OsRng);
        return Wallet {
            public_key: keypair.public,
            secret_key: keypair.secret,
            coin_store: vec![0.00],
        };
    }

    // test function
    pub fn store(&mut self, coin: f32) {
        self.coin_store.push(coin);
        println!("Coin Added")
    }
}
