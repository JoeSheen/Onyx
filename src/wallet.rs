use super::*;

pub struct Wallet {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
    pub coins: f32,
}

impl Wallet {
    // function for creating a new wallet (contains pub/sec keys)
    pub fn new() -> Wallet {
        let keypair: Keypair = Keypair::generate(&mut OsRng);
        return Wallet {
            public_key: keypair.public,
            secret_key: keypair.secret,
            coins: 0.00,
        };
    }
}
