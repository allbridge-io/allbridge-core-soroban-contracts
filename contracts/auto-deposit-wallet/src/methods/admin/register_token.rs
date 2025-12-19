use shared::Error;
use soroban_sdk::{Address, Env, Vec};

use crate::methods::internal;

pub fn register_token(env: Env, token: Address) -> Result<u128, Error> {
    internal::register_token(&env, &token)
}

pub fn register_tokens(env: Env, tokens: Vec<Address>) -> Result<(), Error> {
    for token in tokens {
        internal::register_token(&env, &token)?;
    }

    Ok(())
}
