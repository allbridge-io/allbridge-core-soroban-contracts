use shared::{require, soroban_data::SimpleSorobanData, utils::safe_cast, Error, Event};
use soroban_sdk::{token, Address, Env};

use crate::{
    events::Withdraw,
    storage::{pool::Pool, user_deposit::UserDeposit},
};

pub fn withdraw(env: Env, sender: Address, amount_lp: u128) -> Result<(), Error> {
    sender.require_auth();
    let mut pool = Pool::get(&env)?;

    require!(pool.can_withdraw, Error::Forbidden);

    let mut user_deposit = UserDeposit::get(&env, sender.clone());
    let token_client = token::Client::new(&env, &pool.token);
    let amount = pool.withdraw(&mut user_deposit, amount_lp)?;

    pool.save(&env);
    user_deposit.save(&env, sender.clone());

    Withdraw {
        user: sender.clone(),
        amount: amount_lp,
    }
    .publish(&env);

    token_client.transfer(
        &env.current_contract_address(),
        &sender,
        &safe_cast(amount)?,
    );

    Ok(())
}
