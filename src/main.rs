use onyxlib::*;
fn main() {
    /*let mut blockchain = Blockchain::new();
    let b1 = blockchain.last_block();
    for index in 1..=9 {
        match b1 {
            Some(block) => println!("{}: time: {:?}",index, block.timestamp),
            None => println!("{}: Error", index),
        }
    }*/
    let gen_block = Block::new(0, vec![0; 32], vec![]);
    let hash = gen_block.hash();
    println!("hash: {:?}", hex::encode(hash));
    let test_diff = set_difficulty();
    println!("{:?}", test_diff);
}
