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
