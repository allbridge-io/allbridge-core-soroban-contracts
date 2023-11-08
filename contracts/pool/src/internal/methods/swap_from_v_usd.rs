use crate::events::SwappedFromVUsd;
use crate::storage::bridge_address::Bridge;
use shared::event::Event;
use shared::{error::Error, soroban_data::AnySimpleSorobanData};
use soroban_sdk::{token, Address, Env};

use crate::storage::pool::Pool;

pub fn swap_from_v_usd(
    env: Env,
    user: Address,
    vusd_amount: u128,
    receive_amount_min: u128,
    zero_fee: bool,
) -> Result<u128, Error> {
    let mut pool = Pool::get(&env)?;

    Bridge::require_exist_auth(&env)?;

    let token_client = token::Client::new(&env, &pool.token);
    let (amount, fee) = pool.swap_from_v_usd(vusd_amount, receive_amount_min, zero_fee)?;
    token_client.transfer(&env.current_contract_address(), &user, &(amount as i128));

    pool.save(&env);
    SwappedFromVUsd {
        token: pool.token,
        amount,
        vusd_amount,
        recipient: user,
        fee,
    }
    .publish(&env);

    Ok(amount)
}
