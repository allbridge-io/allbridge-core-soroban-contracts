use bridge_storage::Admin;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

use crate::storage::config::Config;

pub fn unregister_token(env: Env, token_address: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;

    Config::update(&env, |config| {
        config.accepted_tokens.remove(token_address.clone());
        config.fee_conversion_factor.remove(token_address);

        Ok(())
    })
}
