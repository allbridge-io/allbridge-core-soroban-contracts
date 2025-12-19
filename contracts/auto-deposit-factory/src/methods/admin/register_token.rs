use bridge_storage::Admin;
use shared::{
    consts::{CHAIN_PRECISION, ORACLE_PRECISION},
    soroban_data::SimpleSorobanData,
    Error,
};
use soroban_sdk::{token, Address, Env};

use crate::storage::config::Config;

pub fn register_token(env: Env, token_address: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;

    let token = token::Client::new(&env, &token_address);
    let token_decimals = token.decimals();
    let fee_conversion_factor = 10u128.pow(ORACLE_PRECISION - token_decimals + CHAIN_PRECISION);

    Config::update(&env, |config| {
        config.accepted_tokens.set(token_address.clone(), ());
        config
            .fee_conversion_factor
            .set(token_address, fee_conversion_factor);

        Ok(())
    })
}
