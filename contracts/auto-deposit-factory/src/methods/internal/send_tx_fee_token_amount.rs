use shared::{consts::CHAIN_ID, soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

use crate::storage::{config::Config, get_gas_oracle_client};

pub fn get_send_tx_fee_token_amount(env: &Env, token_address: Address) -> Result<u128, Error> {
    let config = Config::get(env)?;
    let gas_oracle = get_gas_oracle_client(env)?;

    let fee_conversion_scaling_factor = config
        .fee_conversion_factor
        .get(token_address)
        .ok_or(Error::Uninitialized)?;

    let fee = config.send_tx_cost * gas_oracle.get_price(&CHAIN_ID) / fee_conversion_scaling_factor;

    Ok(fee)
}
