use bridge_storage::*;
use shared::soroban_data::AnySimpleSorobanData;
use shared::{
    consts::{CHAIN_PRECISION, ORACLE_PRECISION},
    error::Error,
};
use soroban_sdk::{Address, BytesN, Env};

use crate::storage::bridge::Bridge;

pub fn add_pool(env: Env, pool: &Address, token: &BytesN<32>) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Bridge::update(&env, |config| {
        config.pools.set(token.clone(), pool.clone());

        let token_address = Address::from_contract_id(token);
        let token = soroban_sdk::token::Client::new(&env, &token_address);
        let token_decimals = token.decimals();

        let bridging_fee_conversion_factor =
            10u128.pow(ORACLE_PRECISION - token_decimals + CHAIN_PRECISION);
        let from_gas_oracle_factor = 10u128.pow(ORACLE_PRECISION - token_decimals);

        config
            .bridging_fee_conversion_factor
            .set(token_address.clone(), bridging_fee_conversion_factor);
        config
            .from_gas_oracle_factor
            .set(token_address, from_gas_oracle_factor);

        Ok(())
    })
}
