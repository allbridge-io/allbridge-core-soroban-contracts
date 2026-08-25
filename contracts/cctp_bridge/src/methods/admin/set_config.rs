use bridge_storage::Admin;
use shared::{require, Error};
use soroban_sdk::{Address, Env};

use crate::{methods::internal, storage};

pub fn set_admin_fee_share(env: Env, admin_fee_share_bp: u64) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    internal::validate_admin_fee_share(admin_fee_share_bp)?;
    let mut config = storage::get_config(&env)?;
    config.admin_fee_share_bp = admin_fee_share_bp;
    storage::set_config(&env, &config);
    Ok(())
}

pub fn set_max_fee_share(env: Env, value: i128) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    internal::validate_max_fee_share(value)?;
    let mut config = storage::get_config(&env)?;
    config.max_fee_share = value;
    storage::set_config(&env, &config);
    Ok(())
}

pub fn set_min_finality_threshold(env: Env, value: u32) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let mut config = storage::get_config(&env)?;
    config.min_finality_threshold = value;
    storage::set_config(&env, &config);
    Ok(())
}

pub fn set_gas_oracle(env: Env, gas_oracle: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let mut config = storage::get_config(&env)?;
    config.gas_oracle = gas_oracle;
    storage::set_config(&env, &config);
    Ok(())
}

pub fn set_token_messenger_minter(env: Env, token_messenger_minter: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let mut config = storage::get_config(&env)?;
    config.token_messenger_minter = token_messenger_minter;
    storage::set_config(&env, &config);
    Ok(())
}

pub fn set_message_transmitter(env: Env, message_transmitter: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let mut config = storage::get_config(&env)?;
    config.message_transmitter = message_transmitter;
    storage::set_config(&env, &config);
    Ok(())
}

pub fn set_fee_conversion_factor(env: Env, value: u128) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    require!(value > 0, Error::InvalidArg);
    let mut config = storage::get_config(&env)?;
    config.bridging_fee_conversion_factor = value;
    storage::set_config(&env, &config);
    Ok(())
}
