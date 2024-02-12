use shared::{require, soroban_data::SimpleSorobanData, utils::safe_cast, Error, Event};
use soroban_sdk::{token, Address, Env};

use crate::{
    events::{Deposit, RewardsClaimed},
    storage::{pool::Pool, user_deposit::UserDeposit},
};

pub fn deposit(env: Env, sender: Address, amount: u128) -> Result<(), Error> {
    sender.require_auth();
    let mut pool = Pool::get(&env)?;

    require!(pool.can_deposit, Error::Forbidden);

    let mut user_deposit = UserDeposit::get(&env, sender.clone());
    let token_client = token::Client::new(&env, &pool.token);

    token_client.transfer(
        &sender,
        &env.current_contract_address(),
        &safe_cast(amount)?,
    );

    let (rewards, lp_amount) = pool.deposit(amount, &mut user_deposit)?;

    pool.save(&env);
    user_deposit.save(&env, sender.clone());

    Deposit {
        user: sender.clone(),
        amount: lp_amount,
    }
    .publish(&env);

    RewardsClaimed {
        user: sender.clone(),
        amount: rewards,
    }
    .publish(&env);

    token_client.transfer(
        &env.current_contract_address(),
        &sender,
        &safe_cast(rewards)?,
    );

    Ok(())
}
