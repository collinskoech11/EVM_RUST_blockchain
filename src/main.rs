mod blockchain;
mod evm;

use crate::blockchain::Blockchain;
use crate::evm::EVM;

fn main() {
    // Create a new blockchain
    let mut blockchain = Blockchain::new();
    // Interact with the EVM
    let provider_url = "http://localhost:8545"; // Replace with your Ethereum node URL
    let private_key = "YOUR_PRIVATE_KEY"; // Replace with your private key
    let evm = EVM::new(provider_url, private_key);

    // Add blocks to the blockchain with a specified difficulty level for mining
    blockchain.add_block("Block 1 Data".to_string(), 2);
    blockchain.add_block("Block 2 Data".to_string(), 2);
    blockchain.add_block("Block 3 Data".to_string(), 2);

    // Print the blockchain's blocks and verify its integrity
    for block in &blockchain.blocks {
        println!("Index: {}", block.index);
        println!("Timestamp: {}", block.timestamp);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Data: {}", block.data);
        println!("Nonce: {}", block.nonce);
        println!("Hash: {}", block.hash);
        println!();
    }

    // Verify the blockchain's integrity
    if blockchain.is_chain_valid() {
        println!("Blockchain is valid.");
    } else {
        println!("Blockchain is NOT valid.");
    }

    // Execute EVM transactions within the latest block
    blockchain.execute_transactions(evm &EVM);
    

    // // Example usage:
    // let bytecode = "0x606060405260e060020a6000350463123456789c811460215781181...";
    // let contract_address = evm.deploy_contract(bytecode).unwrap();
    // println!("Deployed contract address: {:?}", contract_address);

    // Additional EVM interactions can be added here
}
