use bridge_storage::NativeToken;
use shared::{require, utils::safe_cast, Error};
use soroban_sdk::{Address, Env, Vec};

use crate::{
    events::DepositAddressCreation,
    methods::{internal::convert_fee_in_tokens_to_native_token, view::get_transaction_cost},
};

pub fn create_deposit_wallet(
    env: Env,
    sender: Address,
    recipient: Address,
    recipient_token: Address,
    min_deposit_amount: u128,
    gas_amount: u128,
    fee_token_amount: u128,
    chain_ids: Vec<u32>,
) -> Result<(), Error> {
    sender.require_auth();
    require!(min_deposit_amount > 0, Error::ADMinDepositAmountIsZero);

    let fee = gas_amount
        + convert_fee_in_tokens_to_native_token(&env, &sender, &recipient_token, fee_token_amount)?;

    if gas_amount > 0 {
        let native_token = NativeToken::get_client(&env)?;
        native_token.transfer(
            &sender,
            &env.current_contract_address(),
            &safe_cast(gas_amount)?,
        );
    }

    let mut required_fee = 0;
    for id in &chain_ids {
        required_fee += get_transaction_cost(&env, id).map_err(|_| Error::InvalidChainId)?;
    }

    require!(fee >= required_fee, Error::ADNotEnoughFee);

    DepositAddressCreation {
        recipient,
        recipient_token,
        min_deposit_amount,
        chain_ids,
    }
    .publish(&env);

    Ok(())
}
