use shared::{soroban_data::SimpleSorobanData, utils::safe_cast, Error, Event};
use soroban_sdk::{token, Address, Env, BytesN};

use crate::events::BalanceClaimed;
use crate::storage::claimable_balance::ClaimableBalance;
use crate::storage::pool::Pool;

pub fn claim_balance(env: Env, user: Address, transfer_id: BytesN<32>) -> Result<(), Error> {
    let pool = Pool::get(&env)?;
    let mut claimable_balance = ClaimableBalance::get(&env, user.clone());

    let amount = claimable_balance.amount;
    if amount > 0 {
        let token_client = token::Client::new(&env, &pool.token);

        claimable_balance.amount = 0;
        claimable_balance.save(&env, user.clone());

        token_client.transfer(&env.current_contract_address(), &user, &safe_cast(amount)?);
    }
    BalanceClaimed { user, amount, transfer_id }.publish(&env);

    Ok(())
}
