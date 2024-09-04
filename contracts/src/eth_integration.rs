use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::signers::{LocalWallet, Signer};
use std::sync::Arc;
use std::convert::TryFrom;
use crate::web3_utils::load_abi; // Helper function to load contract ABI

/// Ethereum RPC URL
const ETHEREUM_RPC_URL: &str = "https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID";

// Handles Ethereum interactions like balance check and contract calls
pub struct EthIntegration {
    provider: Arc<Provider<Http>>,
    wallet: LocalWallet,
}

impl EthIntegration {
    // Initialize Ethereum integration
    pub async fn new(private_key: &str) -> Self {
        let provider = Provider::<Http>::try_from(ETHEREUM_RPC_URL)
            .expect("Invalid RPC URL");
        let wallet: LocalWallet = private_key.parse().expect("Invalid private key");

        EthIntegration {
            provider: Arc::new(provider),
            wallet,
        }
    }

    // Retrieve balance of a given address
    pub async fn get_balance(&self, address: Address) -> U256 {
        self.provider.get_balance(address, None).await.expect("Failed to fetch balance")
    }

    // Call a contract method
    pub async fn call_contract<M: Middleware>(
        &self, 
        contract_address: Address, 
        method_signature: &str
    ) -> Result<U256, ContractError<M>> {
        let client = SignerMiddleware::new(self.provider.clone(), self.wallet.clone());
        let contract = Contract::new(contract_address, load_abi(), Arc::new(client));

        contract
            .method::<_, U256>(method_signature, ())
            .expect("Invalid method")
            .call()
            .await
    }

    // Send a transaction to the contract
    pub async fn send_transaction<M: Middleware>(
        &self, 
        contract_address: Address, 
        method_signature: &str
    ) -> Result<TxHash, ContractError<M>> {
        let client = SignerMiddleware::new(self.provider.clone(), self.wallet.clone());
        let contract = Contract::new(contract_address, load_abi(), Arc::new(client));

        let tx = contract
            .method::<_, U256>(method_signature, ())
            .expect("Invalid method")
            .send()
            .await?;

        Ok(tx.tx_hash())
    }
}
