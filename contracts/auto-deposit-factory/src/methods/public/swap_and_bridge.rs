use shared::Error;
use soroban_sdk::{Address, Env, U256};

use crate::methods::internal::{self};

pub fn swap_and_bridge(
    env: Env,
    wallet: Address,
    token: Address,
    nonce: U256,
) -> Result<(), Error> {
    internal::swap_and_bridge(&env, &wallet, &token, &nonce)
}
