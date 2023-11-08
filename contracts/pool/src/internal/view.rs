use crate::storage::pool::Pool;
use crate::storage::user_deposit::UserDeposit;
use shared::{error::Error, soroban_data::AnySimpleSorobanData};
use soroban_sdk::{Address, Env};

pub fn pending_reward(env: Env, user: Address) -> Result<u128, Error> {
    let user_deposit = UserDeposit::get(&env, user);
    let pool = Pool::get(&env)?;
    Ok(
        ((user_deposit.lp_amount * pool.acc_reward_per_share_p) >> Pool::P)
            - user_deposit.reward_debt,
    )
}

pub fn get_pool(env: Env) -> Result<Pool, Error> {
    Pool::get(&env)
}
