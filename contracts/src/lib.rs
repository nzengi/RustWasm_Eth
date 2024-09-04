pub mod erc20_contract;
pub mod eth_integration;
pub mod web3_utils;
pub mod governance_contract;

use ethers::contract::Abi;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Function to load the contract ABI from the specified JSON file.
///
/// # Returns
/// * `Abi` - The deserialized ABI used for contract interaction.
///
pub fn load_abi() -> Abi {
    let path = Path::new("../abi/my_contract_abi.json");
    
    // Open the ABI JSON file
    let mut file = File::open(path).expect("Unable to open ABI file");
    
    // Read the file contents
    let mut abi_json = String::new();
    file.read_to_string(&mut abi_json).expect("Unable to read ABI file");

    // Deserialize the JSON content into an Abi struct
    serde_json::from_str(&abi_json).expect("Invalid ABI format")
}
