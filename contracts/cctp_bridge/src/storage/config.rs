use shared::Error;
use soroban_sdk::{contracttype, Address, Env};

use crate::storage::DataKey;

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Config {
    pub admin: Address,
    pub usdc_token: Address,
    pub token_messenger_minter: Address,
    pub message_transmitter: Address,
    pub gas_oracle: Address,
    pub native_token: Address,
    pub min_finality_threshold: u32,
    pub max_fee_share: i128,
    pub admin_fee_share_bp: u64,
    pub bridging_fee_conversion_factor: u128,
    pub outbound_nonce: u128,
}

pub fn has_config(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Config)
}

pub fn get_config(env: &Env) -> Result<Config, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Config)
        .ok_or(Error::Uninitialized)
}

pub fn set_config(env: &Env, config: &Config) {
    env.storage().instance().set(&DataKey::Config, config);
}
