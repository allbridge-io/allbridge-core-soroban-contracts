use proc_macros::{
    data_storage_type, extend_ttl_info_instance, symbol_key, SorobanData, SorobanSimpleData,
};
use soroban_sdk::{contracttype, Address, BytesN, Env, Map};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, SorobanData, SorobanSimpleData)]
#[symbol_key("Config")]
#[data_storage_type(Instance)]
#[extend_ttl_info_instance]
pub struct Config {
    pub bridge: Address,
    pub factory: Address,
    pub recipient_chain_id: u32,
    pub recipient: BytesN<32>,
    pub recipient_token: BytesN<32>,
    pub min_deposit_amount: u128,
    pub min_deposit_token_amount: Map<Address, u128>,
}

impl Config {
    pub fn new(
        env: &Env,
        factory: Address,
        bridge: Address,
        recipient_chain_id: u32,
        recipient: BytesN<32>,
        recipient_token: BytesN<32>,
        min_deposit_amount: u128,
    ) -> Self {
        Config {
            factory,
            bridge,
            recipient,
            recipient_chain_id,
            recipient_token,
            min_deposit_amount,
            min_deposit_token_amount: Map::new(&env),
        }
    }
}
