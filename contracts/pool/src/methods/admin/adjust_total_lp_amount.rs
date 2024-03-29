use bridge_storage::*;
use shared::{soroban_data::SimpleSorobanData, utils::safe_cast, Error};
use soroban_sdk::{token, Env};

use crate::storage::{pool::Pool, user_deposit::UserDeposit};

pub fn adjust_total_lp_amount(env: Env) -> Result<(), Error> {
    let mut pool = Pool::get(&env)?;
    let admin = Admin::get(&env)?;

    admin.require_auth();

    let mut user_deposit = UserDeposit::get(&env, admin.as_address());

    let amount = pool.d - pool.total_lp_amount;
    if amount == 0 {
        return Ok(());
    }

    let reward_amount = pool.deposit_lp(&mut user_deposit, amount);
    if reward_amount > 0 {
        let token_client = token::Client::new(&env, &pool.token);
        token_client.transfer(
            &env.current_contract_address(),
            admin.as_ref(),
            &safe_cast(reward_amount)?,
        );
    };

    pool.save(&env);
    user_deposit.save(&env, admin.as_address());

    Ok(())
}
