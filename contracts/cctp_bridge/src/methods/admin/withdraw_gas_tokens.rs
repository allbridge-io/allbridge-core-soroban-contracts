use bridge_storage::Admin;
use shared::Error;
use soroban_sdk::{token, Address, Env};

use crate::storage;

pub fn withdraw_gas_tokens(env: Env, sender: Address, amount: u128) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let config = storage::get_config(&env)?;
    token::Client::new(&env, &config.native_token).transfer(
        &env.current_contract_address(),
        &sender,
        &(amount as i128),
    );
    Ok(())
}
