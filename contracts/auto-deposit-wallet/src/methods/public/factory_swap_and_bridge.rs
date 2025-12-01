use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env, U256};

use crate::{methods::internal, storage::config::Config};

pub fn factory_swap_and_bridge(
    env: Env,
    token_address: Address,
    amount: u128,
    nonce: U256,
) -> Result<(), Error> {
    let config = Config::get(&env)?;
    config.factory.require_auth();

    internal::swap_and_bridge(&env, &config, &token_address, amount, &nonce)
}
