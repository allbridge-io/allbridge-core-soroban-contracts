use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

use crate::{methods::internal, storage::config::Config};

pub fn is_token_registered(env: Env, token: Address) -> Result<bool, Error> {
    Ok(min_deposit_token_amount(env, token).is_ok())
}

pub fn min_deposit_token_amount(env: Env, token: Address) -> Result<u128, Error> {
    let config = Config::get(&env)?;

    config
        .min_deposit_token_amount
        .get(token)
        .ok_or(Error::NotFound)
}

pub fn get_bridging_cost_in_tokens(env: Env, token: Address) -> Result<u128, Error> {
    let config = Config::get(&env)?;
    let bridge_client = crate::bridge::Client::new(&env, &config.bridge);

    Ok(internal::get_bridging_cost_in_tokens(&env, &config, &token, &bridge_client)? + 1)
}
