use shared::Error;
use soroban_sdk::{Address, BytesN, Env, U256};

use crate::methods::internal;

pub fn create_swap_and_bridge(
    env: Env,
    recipient_chain_id: u32,
    recipient: BytesN<32>,
    recipient_token: BytesN<32>,
    min_deposit_amount: u128,
    token: Address,
    nonce: U256,
) -> Result<(), Error> {
    let wallet = internal::deploy_deposit_wallet(
        &env,
        recipient_chain_id,
        &recipient,
        &recipient_token,
        min_deposit_amount,
    )?;

    internal::swap_and_bridge(&env, &wallet, &token, &nonce)?;

    Ok(())
}
