
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use ethers::prelude::*; // Import ethers-rs components
use ethers::providers::Http;
use ethers::signers::Wallet;
use hex;

pub struct Transaction {
    from: Address,
    to: Option<Address>,
    value: U256,
    data: Vec<u8>,
    gas_limit: U256,
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

pub struct Wallet{
    addr:Box<Wallet>
}

pub struct EVM {
    provider: Http,
    signer: Wallet,
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

impl EVM {
    pub async fn new(provider_url: &str, private_key: &str) -> Self {
        let provider = Http::new(provider_url).expect("Failed to create HTTP provider");
        let signer = Wallet::from_str(private_key, provider.clone())
            .expect("Failed to create wallet from private key");
        
        Self { provider, signer }
    }

    pub async fn deploy_contract(&self, bytecode: &str) -> Result<Address, Box<dyn std::error::Error>> {
        let factory = ContractFactory::new(
            bytecode.as_bytes(),
            self.signer.clone(),
        );

        let contract = factory.deploy(())
            .send()
            .await?;
        
        Ok(contract.address())
    }

    // Add more functions for interacting with the EVM as needed
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

    pub async fn execute_transactions(&mut self,evm: &EVM) {
        // Create an Ethers provider
        let http_provider = Http::new("http://localhost:8545").expect("Failed to create HTTP provider");
        
        // Create a wallet and signer (replace with your private key)
        let wallet = LocalWallet::new([0u8; 32], http_provider.clone());
        let wallet = Wallet::new(wallet, http_provider.clone());

        // Iterate through the transactions in the latest block
        for transaction in &self.blocks.last().unwrap().transactions {
            // Validate and execute each transaction
            // Implement gas calculation, nonce handling, state changes, etc.
            // Handle contract deployments and method calls
            // Deserialize the transaction fields
            let from_address = wallet.address();
            let to_address = transaction.to.clone();
            let data = hex::decode(&transaction.data).expect("Failed to decode transaction data");

            // Create a transaction
            let tx = Transaction {
                nonce: http_provider.get_transaction_count(from_address, None).await.expect("Failed to get nonce"),
                gas_price: 1_000_000_000u64.into(), // Set a gas price
                gas_limit: 100_000u64.into(), // Set a gas limit
                to: to_address,
                value: transaction.value.clone().into(), // Convert to ethers' U256
                data: data.into(),
                chain_id: 1u64, // Replace with the correct chain ID
            };

            // Sign and send the transaction
            let signed_tx = wallet.sign_transaction(tx).await.expect("Failed to sign transaction");
            let tx_hash = http_provider.send_transaction(signed_tx).await.expect("Failed to send transaction");

            // Wait for the transaction to be mined (optional)
            let _receipt = http_provider.wait_for_transaction(tx_hash, None).await.expect("Transaction failed to confirm");


            
            match transaction.to {
                Some(to) => {
                    // Execute a contract method call
                    let result = evm.call_contract(&transaction.from, to, &transaction.data);
                    match result {
                        Ok(_) => {
                            // Handle a successful contract method call
                            // Update the blockchain state as needed
                        }
                        Err(_) => {
                            // Handle a failed contract method call
                        }
                    }
                }
                None => {
                    // This is a contract deployment
                    let contract_address = evm.deploy_contract(&transaction.data);
                    match contract_address {
                        Ok(_) => {
                            // Handle a successful contract deployment
                            // Update the blockchain state as needed
                        }
                        Err(_) => {
                            // Handle a failed contract deployment
                        }
                    }
                }
            }
        }
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
