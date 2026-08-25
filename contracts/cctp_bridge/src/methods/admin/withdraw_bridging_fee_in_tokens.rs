use bridge_storage::Admin;
use shared::Error;
use soroban_sdk::{token, Address, Env};

pub fn withdraw_bridging_fee_in_tokens(
    env: Env,
    sender: Address,
    token_address: Address,
) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let token_client = token::Client::new(&env, &token_address);
    let balance = token_client.balance(&env.current_contract_address());
    if balance > 0 {
        token_client.transfer(&env.current_contract_address(), &sender, &balance);
    }
    Ok(())
}
