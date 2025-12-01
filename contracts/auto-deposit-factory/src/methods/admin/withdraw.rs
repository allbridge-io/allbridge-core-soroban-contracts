use bridge_storage::*;
use shared::Error;
use soroban_sdk::{token, Address, Env};

pub fn withdraw(
    env: Env,
    sender: Address,
    token_address: Address,
    amount: u128,
) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let contract = env.current_contract_address();

    let token = token::Client::new(&env, &token_address);

    if amount > 0 {
        token.transfer(&contract, &sender, &(amount as i128));
    }

    Ok(())
}
