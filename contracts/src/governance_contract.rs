use ethers::prelude::*;
use std::sync::Arc;

pub struct GovernanceContract<M: Middleware> {
    contract: Contract<M>,
}

impl<M: Middleware> GovernanceContract<M> {
    // Initializes the governance contract
    pub fn new(address: Address, client: Arc<M>, abi: Abi) -> Self {
        let contract = Contract::new(address, abi, client);
        GovernanceContract { contract }
    }

    // Votes for a proposal
    pub async fn vote(&self, proposal_id: U256, support: bool) -> Result<TxHash, ContractError<M>> {
        self.contract
            .method::<_, TxHash>("vote", (proposal_id, support))
            .expect("Invalid method")
            .send()
            .await
            .map(|tx| tx.tx_hash())
    }

    // Gets the proposal details
    pub async fn get_proposal(&self, proposal_id: U256) -> Result<U256, ContractError<M>> {
        self.contract
            .method::<_, U256>("getProposal", proposal_id)
            .expect("Invalid method")
            .call()
            .await
    }
}
