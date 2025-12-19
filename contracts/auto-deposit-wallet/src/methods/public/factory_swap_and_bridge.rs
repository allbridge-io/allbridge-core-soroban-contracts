use crate::{methods::internal::swap_and_bridge, storage::config::Config};
use shared::{soroban_data::SimpleSorobanData, utils::safe_cast, Error};
use soroban_sdk::{token, Address, Env, U256};

pub fn factory_swap_and_bridge(
    env: Env,
    token_address: Address,
    amount: u128,
    factory_fee_amount: u128,
    nonce: U256,
) -> Result<(), Error> {
    let config = Config::get(&env)?;
    config.factory.require_auth();

    let token_client = token::Client::new(&env, &token_address);
    token_client.transfer(
        &env.current_contract_address(),
        &config.factory,
        &safe_cast(factory_fee_amount)?,
    );

    swap_and_bridge(&env, &config, &token_address, amount, &nonce)
}
