use shared::{soroban_data::SimpleSorobanData, utils::safe_cast, Error};
use soroban_sdk::{token, Address, Env};

use crate::storage::config::Config;

pub fn register_token(env: &Env, token: &Address) -> Result<u128, Error> {
    let pow = 10_i128.pow(token::Client::new(env, token).decimals());
    let mut min_amount = 0;

    Config::update(env, |config| {
        min_amount = config.min_deposit_amount * safe_cast::<_, u128>(pow)?;
        config
            .min_deposit_token_amount
            .set(token.clone(), min_amount);
        Ok(())
    })?;

    Ok(min_amount)
}
