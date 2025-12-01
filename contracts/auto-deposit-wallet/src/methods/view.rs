use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

use crate::storage::config::Config;

pub fn is_token_registered(env: Env, token: Address) -> Result<bool, Error> {
    let config = Config::get(&env)?;

    Ok(config.min_deposit_token_amount.get(token).is_some())
}

pub fn min_deposit_token_amount(env: Env, token: Address) -> Result<u128, Error> {
    let config = Config::get(&env)?;

    config
        .min_deposit_token_amount
        .get(token)
        .ok_or(Error::NotFound)
}
