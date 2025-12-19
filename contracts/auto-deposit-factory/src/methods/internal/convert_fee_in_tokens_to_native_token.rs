use shared::{consts::CHAIN_ID, require, soroban_data::SimpleSorobanData, utils::safe_cast, Error};
use soroban_sdk::{token, Address, Env};

use crate::storage::{config::Config, get_gas_oracle_client};

pub fn convert_fee_in_tokens_to_native_token(
    env: &Env,
    user: &Address,
    token_address: &Address,
    fee_token_amount: u128,
) -> Result<u128, Error> {
    if fee_token_amount == 0 {
        return Ok(0);
    }

    let config = Config::get(env)?;
    require!(
        config.accepted_tokens.contains_key(token_address.clone()),
        Error::InvalidArg
    );

    let contract = env.current_contract_address();

    let token = token::Client::new(env, token_address);
    let gas_oracle = get_gas_oracle_client(env)?;

    token.transfer(user, &contract, &safe_cast(fee_token_amount)?);

    let fee_conversion_scaling_factor = config
        .fee_conversion_factor
        .get(token_address.clone())
        .ok_or(Error::Uninitialized)?;

    let fee = fee_conversion_scaling_factor * fee_token_amount / gas_oracle.get_price(&CHAIN_ID);

    Ok(fee)
}
