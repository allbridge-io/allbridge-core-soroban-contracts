use shared::{require, soroban_data::SimpleSorobanData, utils::safe_cast, Error};
use soroban_sdk::{token, Address, Env, U256};

use crate::{
    methods::internal::{self},
    storage::config::Config,
};

pub fn swap_and_bridge(env: Env, token_address: Address, nonce: U256) -> Result<(), Error> {
    let config = Config::get(&env)?;
    let token_client = token::Client::new(&env, &token_address);
    let token_amount = safe_cast::<_, u128>(token_client.balance(&env.current_contract_address()))?;

    let min_amount = match config.min_deposit_token_amount.get(token_address.clone()) {
        Some(v) => v,
        None => internal::register_token(&env, &token_address)?,
    };

    require!(token_amount >= min_amount, Error::ADAmountTooLow);

    internal::swap_and_bridge(&env, &config, &token_address, token_amount, &nonce)
}
