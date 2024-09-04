use ethers::prelude::*;
use std::sync::Arc;

pub struct ERC20Contract<M: Middleware> {
    contract: Contract<M>,
}

impl<M: Middleware> ERC20Contract<M> {
    // Initializes the ERC20 contract
    pub fn new(address: Address, client: Arc<M>, abi: Abi) -> Self {
        let contract = Contract::new(address, abi, client);
        ERC20Contract { contract }
    }

    // Retrieves the balance of an address
    pub async fn balance_of(&self, address: Address) -> Result<U256, ContractError<M>> {
        self.contract
            .method::<_, U256>("balanceOf", address)
            .expect("Invalid method")
            .call()
            .await
    }

    // Transfers tokens to a recipient
    pub async fn transfer(&self, to: Address, amount: U256) -> Result<TxHash, ContractError<M>> {
        self.contract
            .method::<_, TxHash>("transfer", (to, amount))
            .expect("Invalid method")
            .send()
            .await
            .map(|tx| tx.tx_hash())
    }
}
