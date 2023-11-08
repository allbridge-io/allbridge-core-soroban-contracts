use crate::events::SwappedToVUsd;
use crate::storage::bridge_address::Bridge;
use shared::event::Event;
use shared::{error::Error, soroban_data::AnySimpleSorobanData};
use soroban_sdk::{Address, Env};

use crate::storage::pool::Pool;

pub fn swap_to_v_usd(env: Env, user: Address, amount: u128, zero_fee: bool) -> Result<u128, Error> {
    user.require_auth();
    let mut pool = Pool::get(&env)?;

    Bridge::require_exist_auth(&env)?;

    let current_pool = env.current_contract_address();
    let token_client = soroban_sdk::token::Client::new(&env, &pool.token);

    token_client.transfer(&user, &current_pool, &(amount as i128));

    let (vusd_amount, fee) = pool.swap_to_v_usd(amount, zero_fee)?;
    pool.save(&env);

    SwappedToVUsd {
        token: pool.token,
        amount,
        vusd_amount,
        sender: user,
        fee,
    }
    .publish(&env);

    Ok(vusd_amount)
}
