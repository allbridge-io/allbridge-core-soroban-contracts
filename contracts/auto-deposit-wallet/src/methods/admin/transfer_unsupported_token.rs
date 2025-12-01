use bridge_storage::Admin;
use shared::{require, utils::address_to_bytes, Error};
use soroban_sdk::{token, Address, Env};

use crate::bridge;

pub fn transfer_unsupported_token(
    env: Env,
    token_address: Address,
    recipient: Address,
) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;

    let bridge_client = bridge::Client::new(&env, &token_address);
    let is_not_supported = bridge_client
        .try_get_pool_address(&address_to_bytes(&env, &token_address)?)
        .is_err();
    require!(is_not_supported, Error::ADBridgingIsSupported);

    let token_client = token::Client::new(&env, &token_address);

    let amount = token_client.balance(&env.current_contract_address());
    if amount > 0 {
        token_client.transfer(&env.current_contract_address(), &recipient, &amount);
    }

    Ok(())
}
