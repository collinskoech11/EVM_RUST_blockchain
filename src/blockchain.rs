use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Transaction {
    from: Address,
    to: Option<Address>,
    value: U256,
    data: Vec<u8>,
    gas_limit: Gas,
    gas_price: U256,
}

pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub data: String,
    pub nonce: u64,
    pub hash: String,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Block {
            index,
            timestamp,
            previous_hash,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, self.data, self.nonce
        ));

        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn mine(&mut self, difficulty: u64) {
        let target = "0".repeat(difficulty as usize);

        loop {
            self.nonce += 1;
            self.hash = self.calculate_hash();

            if self.hash.starts_with(&target) {
                break;
            }
        }
    }
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"));
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String, difficulty: u64) {
        let previous_block = self.blocks.last().unwrap();
        let index = previous_block.index + 1;
        let previous_hash = previous_block.hash.clone();

        let mut new_block = Block::new(index, previous_hash, data);
        new_block.mine(difficulty);

        self.blocks.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }

        true
    }
}



#[cfg(test)]
mod tests {
    use crate::blockchain::Block;
    use crate::blockchain::Blockchain;

    #[test]
    fn test_block_creation() {
        // Test block creation
        let block = Block::new(0, String::from("0"), String::from("Genesis Block"));

        // Add assertions to check the block's fields
        assert_eq!(block.index, 0);
        assert_eq!(block.previous_hash, "0");
        assert_eq!(block.data, "Genesis Block");
        // Add more assertions as needed
    }

    #[test]
    fn test_block_mining() {
        // Test block mining
        let mut block = Block::new(0, String::from("0"), String::from("Genesis Block"));
        block.mine(2); // Mine the block with difficulty level 2

        // Add assertions to check if mining was successful
        assert!(block.hash.starts_with("00")); // Check for a hash with two leading zeros (difficulty level 2)
        // Add more assertions as needed
    }

    #[test]
    fn test_blockchain_creation() {
        // Test blockchain creation
        let blockchain = Blockchain::new();

        // Add assertions to check if the blockchain contains the genesis block
        assert_eq!(blockchain.blocks.len(), 1);
        assert_eq!(blockchain.blocks[0].index, 0);
        // Add more assertions as needed
    }

    #[test]
    fn test_blockchain_add_block() {
        // Test adding a block to the blockchain
        let mut blockchain = Blockchain::new();
        let data = "Block Data".to_string();
        let difficulty = 2;

        blockchain.add_block(data.clone(), difficulty);

        // Add assertions to check if the blockchain has the newly added block
        assert_eq!(blockchain.blocks.len(), 2);
        assert_eq!(blockchain.blocks[1].data, data);
        // Add more assertions as needed
    }

    #[test]
    fn test_blockchain_validity() {
        // Test blockchain validity
        let mut blockchain = Blockchain::new();
        let data = "Block Data".to_string();
        let difficulty = 2;

        blockchain.add_block(data.clone(), difficulty);

        // The blockchain should be valid after adding a block
        assert!(blockchain.is_chain_valid());
    }

    // Add more tests as needed
}
