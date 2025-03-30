use chrono::prelude::*;
use sha2::{Digest, Sha256};


#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64
}

impl Block {
    pub fn new(index: u64, data: String, difficulty: String) -> Self {
        let timestamp = Utc::now().to_string();

        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash: "".to_string(),
            hash: String::new(),
            nonce: 0
        };

        block.mine(&difficulty);

        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!("{}{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash, self.nonce);

        let mut hasher = Sha256::new();
        
        hasher.update(input);

        format!("{:x}", hasher.finalize())
    }

    pub fn mine(&mut self, difficulty: &String) {
        while !self.hash.starts_with(difficulty) {
            self.nonce = self.nonce + 1;
            self.hash = self.calculate_hash();
        }
    }
}