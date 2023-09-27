use ethers::prelude::*;
use ethers::providers::Http;
use ethers::signers::Wallet;

pub struct Wallet{
    addr:Box<Wallet>,
}

pub struct EVM {
    provider: Http,
    signer: Wallet,
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
