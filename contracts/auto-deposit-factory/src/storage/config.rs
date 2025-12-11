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
    pub wallet_wasm_hash: BytesN<32>,
    /// Cost of send bridgeAndSwap transaction in native tokens
    pub send_tx_cost: u128,
    pub bridge: Address,
    pub accepted_tokens: Map<Address, ()>,
    /// precomputed values of the scaling factor required for paying the bridging fee with stable tokens
    pub fee_conversion_factor: Map<Address, u128>,
}

impl Config {
    pub fn new(
        env: &Env,
        bridge: Address,
        send_tx_cost: u128,
        wallet_wasm_hash: BytesN<32>,
    ) -> Self {
        Config {
            wallet_wasm_hash,
            send_tx_cost,
            bridge,
            fee_conversion_factor: Map::new(env),
            accepted_tokens: Map::new(env),
        }
    }
}
