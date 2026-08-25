use shared::{consts::CHAIN_ID, Error};
use soroban_sdk::{token, Address, Env};

use crate::{
    external::GasOracleClient,
    storage::{self, ChainBridge},
};

pub fn get_chain_bridge(env: Env, chain_id: u32) -> Result<ChainBridge, Error> {
    storage::get_chain_bridge(&env, chain_id)
}

pub fn get_domain_by_chain_id(env: Env, chain_id: u32) -> Result<u32, Error> {
    Ok(storage::get_chain_bridge(&env, chain_id)?.domain)
}

pub fn get_transaction_cost(env: Env, chain_id: u32) -> Result<u128, Error> {
    let config = storage::get_config(&env)?;
    let chain_bridge = storage::get_chain_bridge(&env, chain_id)?;
    Ok(GasOracleClient::new(&env, &config.gas_oracle)
        .get_gas_cost_in_native_token(&chain_id, &chain_bridge.gas_usage))
}

pub fn get_bridging_cost_in_tokens(env: Env, chain_id: u32) -> Result<u128, Error> {
    let config = storage::get_config(&env)?;
    let transaction_cost = get_transaction_cost(env.clone(), chain_id)?;
    let gas_oracle = GasOracleClient::new(&env, &config.gas_oracle);
    Ok(transaction_cost * gas_oracle.get_price(&CHAIN_ID) / config.bridging_fee_conversion_factor)
}

pub fn native_fee_balance(env: Env) -> Result<i128, Error> {
    let config = storage::get_config(&env)?;
    Ok(token::Client::new(&env, &config.native_token).balance(&env.current_contract_address()))
}

pub fn bridging_fee_in_tokens(env: Env, token_address: Address) -> i128 {
    token::Client::new(&env, &token_address).balance(&env.current_contract_address())
}

pub fn admin(env: Env) -> Result<Address, Error> {
    Ok(storage::get_config(&env)?.admin)
}

pub fn usdc_token(env: Env) -> Result<Address, Error> {
    Ok(storage::get_config(&env)?.usdc_token)
}

pub fn token_messenger_minter(env: Env) -> Result<Address, Error> {
    Ok(storage::get_config(&env)?.token_messenger_minter)
}

pub fn message_transmitter(env: Env) -> Result<Address, Error> {
    Ok(storage::get_config(&env)?.message_transmitter)
}

pub fn gas_oracle(env: Env) -> Result<Address, Error> {
    Ok(storage::get_config(&env)?.gas_oracle)
}

pub fn native_token(env: Env) -> Result<Address, Error> {
    Ok(storage::get_config(&env)?.native_token)
}

pub fn min_finality_threshold(env: Env) -> Result<u32, Error> {
    Ok(storage::get_config(&env)?.min_finality_threshold)
}

pub fn max_fee_share(env: Env) -> Result<i128, Error> {
    Ok(storage::get_config(&env)?.max_fee_share)
}

pub fn admin_fee_share_bp(env: Env) -> Result<u64, Error> {
    Ok(storage::get_config(&env)?.admin_fee_share_bp)
}

pub fn bridging_fee_conversion_factor(env: Env) -> Result<u128, Error> {
    Ok(storage::get_config(&env)?.bridging_fee_conversion_factor)
}
