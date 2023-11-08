use shared::event::Event;
use shared::require;
use shared::{error::Error, soroban_data::AnySimpleSorobanData};
use soroban_sdk::{token, Address, Env};

use crate::events::{Deposit, RewardsClaimed};
use crate::storage::pool::Pool;
use crate::storage::user_deposit::UserDeposit;

pub fn deposit(env: Env, sender: Address, amount: u128) -> Result<(), Error> {
    sender.require_auth();
    let mut pool = Pool::get(&env)?;

    require!(pool.can_deposit, Error::Forbidden);

    let mut user_deposit = UserDeposit::get(&env, sender.clone());
    let token_client = token::Client::new(&env, &pool.token);
    token_client.transfer(&sender, &env.current_contract_address(), &(amount as i128));

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

    token_client.transfer(&env.current_contract_address(), &sender, &(rewards as i128));

    Ok(())
}
