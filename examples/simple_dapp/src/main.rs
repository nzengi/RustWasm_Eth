use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::signers::{LocalWallet, Signer};
use std::sync::Arc;
use std::convert::TryFrom;
use tokio::runtime::Runtime;
use dotenv::dotenv;
use std::env;
use rustwasm_eth_contracts::load_abi;

/// Ethereum RPC URL is loaded from the environment for better security and flexibility
const ETHEREUM_RPC_URL: &str = "https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID";

/// Struct representing the Ethereum integration layer
/// Handles connecting to the Ethereum network and interacting with smart contracts
pub struct EthIntegration {
    pub provider: Arc<Provider<Http>>, // Shared reference to Ethereum provider
    pub wallet: LocalWallet, // Wallet to sign transactions
}

impl EthIntegration {
    /// Creates a new instance of the Ethereum integration
    /// Initializes the provider and wallet using a private key
    ///
    /// # Arguments
    ///
    /// * `private_key` - The Ethereum private key to be used for signing transactions
    ///
    /// # Returns
    ///
    /// An instance of `EthIntegration`
    pub async fn new(private_key: &str) -> Self {
        // Initialize HTTP provider using the Ethereum RPC URL
        let provider = Provider::<Http>::try_from(ETHEREUM_RPC_URL)
            .expect("Invalid RPC URL")
            .interval(std::time::Duration::from_secs(10)); // Set a polling interval

        // Parse the private key into a wallet for signing transactions
        let wallet: LocalWallet = private_key.parse().expect("Invalid private key");

        // Return the Ethereum integration with the provider and wallet
        EthIntegration {
            provider: Arc::new(provider),
            wallet,
        }
    }

    /// Retrieves the balance of an Ethereum address in Wei
    ///
    /// # Arguments
    ///
    /// * `address` - The Ethereum address to check the balance of
    ///
    /// # Returns
    ///
    /// The balance in Wei as a `U256`
    pub async fn get_balance(&self, address: Address) -> U256 {
        self.provider
            .get_balance(address, None)
            .await
            .expect("Failed to fetch balance") // Ensure balance retrieval succeeds
    }

    /// Calls a smart contract method without sending a transaction
    /// Useful for reading data from contracts, like ERC-20 token balances
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The address of the contract
    /// * `method_signature` - The contract method to be called
    ///
    /// # Returns
    ///
    /// The result of the contract call as a `U256`
    pub async fn call_contract<M: Middleware>(
        &self, 
        contract_address: Address, 
        method_signature: &str
    ) -> Result<U256, ContractError<M>> {
        // Set up the middleware and contract instance
        let client = SignerMiddleware::new(self.provider.clone(), self.wallet.clone());
        let contract = Contract::new(contract_address, load_abi(), Arc::new(client));

        // Call the specified contract method and return the result
        let result: U256 = contract
            .method::<_, U256>(method_signature, ())
            .expect("Invalid method")
            .call()
            .await?;

        Ok(result)
    }

    /// Sends a transaction to a contract, executing a method that modifies state
    /// Example: transferring tokens, changing ownership, etc.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The address of the contract
    /// * `method_signature` - The contract method to be executed
    ///
    /// # Returns
    ///
    /// The transaction hash as `TxHash`
    pub async fn send_transaction<M: Middleware>(
        &self, 
        contract_address: Address, 
        method_signature: &str
    ) -> Result<TxHash, ContractError<M>> {
        // Set up the middleware and contract instance
        let client = SignerMiddleware::new(self.provider.clone(), self.wallet.clone());
        let contract = Contract::new(contract_address, load_abi(), Arc::new(client));

        // Send the transaction and return the transaction hash
        let tx = contract
            .method::<_, U256>(method_signature, ())
            .expect("Invalid method")
            .send()
            .await?;

        Ok(tx.tx_hash())
    }
}

fn main() {
    dotenv().ok(); // Load environment variables from the .env file

    // Create a new Tokio runtime to manage asynchronous tasks
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        // Load necessary environment variables
        let private_key = env::var("ETH_PRIVATE_KEY").expect("ETH_PRIVATE_KEY not set");
        let contract_address: Address = env::var("CONTRACT_ADDRESS")
            .expect("CONTRACT_ADDRESS not set")
            .parse()
            .expect("Invalid contract address");

        // Initialize Ethereum integration
        let eth_integration = EthIntegration::new(&private_key).await;

        // Retrieve and print the balance of the contract address
        let balance = eth_integration.get_balance(contract_address).await;
        println!("Balance: {}", balance);

        // Example contract method call (e.g., balanceOf)
        let method_signature = "balanceOf";
        match eth_integration.call_contract(contract_address, method_signature).await {
            Ok(result) => println!("Contract call result: {}", result),
            Err(e) => eprintln!("Contract call failed: {:?}", e),
        }
    });
}
