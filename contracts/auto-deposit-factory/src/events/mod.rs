use soroban_sdk::{contractevent, Address, Vec};

#[contractevent(topics = ["AutoDepositWalletDeployed"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoDepositWalletDeployed {
    pub address: Address,
}

#[contractevent(topics = ["DepositAddressCreation"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositAddressCreation {
    pub recipient: Address,
    pub recipient_token: Address,
    pub min_deposit_amount: u128,
    pub chain_ids: Vec<u32>,
}
