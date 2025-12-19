use shared::Error;
use soroban_sdk::{BytesN, Env};

use crate::methods::internal::{self};

pub fn deploy_deposit_wallet(
    env: Env,
    recipient_chain_id: u32,
    recipient: BytesN<32>,
    recipient_token: BytesN<32>,
    min_deposit_amount: u128,
) -> Result<(), Error> {
    internal::deploy_deposit_wallet(
        &env,
        recipient_chain_id,
        &recipient,
        &recipient_token,
        min_deposit_amount,
    )?;

    Ok(())
}
