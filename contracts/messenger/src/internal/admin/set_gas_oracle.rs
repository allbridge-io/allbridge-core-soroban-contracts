use bridge_storage::*;
use shared::{error::Error, soroban_data::SimpleSorobanData};
use soroban_sdk::{Address, Env};

pub fn set_gas_oracle(env: Env, new_address: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    GasOracleAddress(new_address).save(&env);
    Ok(())
}
