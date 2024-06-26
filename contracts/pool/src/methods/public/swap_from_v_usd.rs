use shared::{soroban_data::SimpleSorobanData, utils::safe_cast, Error, Event};
use soroban_sdk::{token, Address, Env};

use crate::{
    events::SwappedFromVUsd,
    storage::{bridge_address::Bridge, pool::Pool},
};

pub fn swap_from_v_usd(
    env: Env,
    user: Address,
    vusd_amount: u128,
    receive_amount_min: u128,
    zero_fee: bool
) -> Result<u128, Error> {
    let mut pool = Pool::get(&env)?;

    Bridge::require_exist_auth(&env)?;

    let (amount, fee) = pool.swap_from_v_usd(vusd_amount, receive_amount_min, zero_fee)?;

    let token_client = token::Client::new(&env, &pool.token);
    token_client.transfer(&env.current_contract_address(), &user, &safe_cast(amount)?);

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
