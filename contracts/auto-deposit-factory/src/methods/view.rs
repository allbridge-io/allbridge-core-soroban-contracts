use bridge_storage::{Admin, GasOracleAddress, GasUsage};
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, BytesN, Env};

use crate::{
    methods::internal::{self, get_deposit_wallet_salt},
    storage::{config::Config, get_gas_oracle_client},
};

pub fn get_send_tx_fee_token_amount(env: Env, token_address: Address) -> Result<u128, Error> {
    internal::get_send_tx_fee_token_amount(&env, token_address)
}

pub fn deposit_wallet_address(
    env: &Env,
    recipient_chain_id: u32,
    recipient: &BytesN<32>,
    recipient_token: &BytesN<32>,
    min_deposit_amount: u128,
) -> Result<Address, Error> {
    let config = Config::get(&env)?;
    let salt = get_deposit_wallet_salt(
        &env,
        recipient_chain_id,
        &recipient,
        &recipient_token,
        min_deposit_amount,
        &config,
    )?;

    Ok(env
        .deployer()
        .with_address(env.current_contract_address(), salt)
        .deployed_address())
}

pub fn deposit_wallet_address_by_salt(env: Env, salt: BytesN<32>) -> Result<Address, Error> {
    Ok(env
        .deployer()
        .with_address(env.current_contract_address(), salt)
        .deployed_address())
}

pub fn get_gas_oracle(env: Env) -> Result<Address, Error> {
    Ok(GasOracleAddress::get(&env)?.as_address())
}

pub fn get_admin(env: Env) -> Result<Address, Error> {
    Ok(Admin::get(&env)?.as_address())
}

pub fn get_gas_usage(env: Env, chain_id: u32) -> Result<u128, Error> {
    GasUsage::get_gas_usage_with_default(env, chain_id)
}

pub fn get_transaction_cost(env: &Env, chain_id: u32) -> Result<u128, Error> {
    let gas_oracle = get_gas_oracle_client(env)?;
    let gas_usage = GasUsage::get_by_chain(env, chain_id)?;

    Ok(gas_oracle.get_gas_cost_in_native_token(&chain_id, &gas_usage))
}
