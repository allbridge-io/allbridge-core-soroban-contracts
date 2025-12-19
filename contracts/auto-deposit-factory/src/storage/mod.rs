use bridge_storage::GasOracleAddress;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::Env;

use crate::gas_oracle;

pub mod config;

pub fn get_gas_oracle_client(env: &Env) -> Result<gas_oracle::Client<'_>, Error> {
    let gas_oracle_address = GasOracleAddress::get(env)?.as_address();
    Ok(gas_oracle::Client::new(env, &gas_oracle_address))
}
