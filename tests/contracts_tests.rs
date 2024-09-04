use rustwasm_eth_contracts::erc20_contract::ERC20Contract;
use rustwasm_eth_contracts::governance_contract::GovernanceContract;
use ethers::prelude::*;
use std::sync::Arc;

// Example test function for ERC20
#[tokio::test]
async fn test_erc20_contract() {
    let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID")
        .expect("Invalid provider");
    let client = Arc::new(provider);
    
    // Load ABI and create contract instance
    let abi = load_abi("erc20_abi.json").expect("Failed to load ABI");
    let erc20 = ERC20Contract::new("0xYourContractAddress".parse().unwrap(), client.clone(), abi);
    
    let balance = erc20.balance_of("0xYourAddress".parse().unwrap()).await.expect("Failed to fetch balance");
    println!("Balance: {:?}", balance);
    
    assert!(balance > U256::from(0));
}

// Example test function for Governance contract
#[tokio::test]
async fn test_governance_contract() {
    let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID")
        .expect("Invalid provider");
    let client = Arc::new(provider);
    
    // Load ABI and create contract instance
    let abi = load_abi("governance_abi.json").expect("Failed to load ABI");
    let governance = GovernanceContract::new("0xYourGovernanceContractAddress".parse().unwrap(), client.clone(), abi);
    
    let proposal = governance.get_proposal(U256::from(1)).await.expect("Failed to fetch proposal");
    println!("Proposal Details: {:?}", proposal);
    
    assert!(proposal > U256::from(0));
}
