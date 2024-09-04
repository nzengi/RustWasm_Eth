use ethers::contract::Abi;
use std::fs::File;
use std::io::Read;

/// Loads the contract ABI from a specified JSON file
pub fn load_abi() -> Abi {
    let mut file = File::open("src/abi/my_contract_abi.json").expect("Unable to open ABI file");
    let mut abi_data = String::new();
    file.read_to_string(&mut abi_data).expect("Failed to read ABI file");
    serde_json::from_str(&abi_data).expect("Failed to parse ABI JSON")
}
