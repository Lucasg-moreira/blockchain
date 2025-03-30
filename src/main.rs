mod block;
mod blockchain;

use block::Block;
use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    let second_block = Block::new(
        1,
        "First transaction".to_string(),
        blockchain.difficulty.to_string()
    );

    blockchain.add_block(second_block);

    println!("{}", blockchain.is_chain_valid());
}
