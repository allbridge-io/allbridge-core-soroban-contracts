use shared::event::Event;
use shared::{error::Error, soroban_data::AnySimpleSorobanData};
use soroban_sdk::{token, Address, Env};

use crate::events::RewardsClaimed;
use crate::storage::pool::Pool;
use crate::storage::user_deposit::UserDeposit;

pub fn claim_rewards(env: Env, sender: Address) -> Result<(), Error> {
    sender.require_auth();
    let pool = Pool::get(&env)?;

    let mut user_deposit = UserDeposit::get(&env, sender.clone());
    let amount = pool.claim_rewards(&mut user_deposit)?;
    if amount > 0 {
        let token_client = token::Client::new(&env, &pool.token);
        token_client.transfer(
            &env.current_contract_address(),
            &sender.clone(),
            &(amount as i128),
        );

        RewardsClaimed {
            user: sender.clone(),
            amount,
        }
        .publish(&env);

        user_deposit.save(&env, sender);
    }

    Ok(())
}
