use crate::block::Block;

pub struct Blockchain {
    blocks: Vec<Block>,
    pub difficulty: String
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain { blocks: Vec::new(), difficulty: "000".to_string() };

        let genesis_block = Block::new(
            0,
            "Genesis block".to_string(),
            blockchain.difficulty.to_string()
        );

        blockchain.add_block(genesis_block);

        blockchain
    }

    pub fn add_block(&mut self, mut block: Block) {
        let last_block = self.find_last_block();

        match last_block {
            Some(value) => {
                block.previous_hash = value.hash.clone();
            },
            None => {
                block.previous_hash = "".to_string();
            }
        }

        self.blocks.push(block);
    }

    pub fn find_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn print_blocks(&self) {
        println!("{:#?}", &self.blocks);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in (1..self.blocks.len()).rev() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];

            let current_calculated_hash = current_block.calculate_hash();

            if current_block.hash != current_calculated_hash
                || current_block.previous_hash != previous_block.hash
                || current_block.index != previous_block.index + 1
            {
                return false;
            }
        }
        true
    }
    
}