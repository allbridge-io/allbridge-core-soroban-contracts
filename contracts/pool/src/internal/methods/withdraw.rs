use shared::event::Event;
use shared::require;
use shared::{error::Error, soroban_data::AnySimpleSorobanData};
use soroban_sdk::{token, Address, Env};

use crate::events::Withdraw;
use crate::storage::pool::Pool;
use crate::storage::user_deposit::UserDeposit;

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

    token_client.transfer(&env.current_contract_address(), &sender, &(amount as i128));

    Ok(())
}
