use shared::{require, utils::safe_cast, Error};
use soroban_sdk::{token, Address, Env, U256};

use crate::{methods::internal::get_send_tx_fee_token_amount, wallet};

pub fn swap_and_bridge(
    env: Env,
    wallet: Address,
    token: Address,
    nonce: U256,
) -> Result<(), Error> {
    wallet.require_auth();
    let wallet_client = wallet::Client::new(&env, &wallet);
    let token_client = token::Client::new(&env, &token);

    let min_amount = wallet_client
        .try_min_deposit_token_amount(&token)
        .ok()
        .and_then(|inner| inner.ok())
        .unwrap_or_else(|| wallet_client.register_token(&token));

    let token_amount = token_client.balance(&wallet);
    let token_amount_u128 = safe_cast::<_, u128>(token_amount)?;

    require!(token_amount_u128 >= min_amount, Error::ADAmountTooLow);

    let send_tx_fee_token_amount = get_send_tx_fee_token_amount(&env, token.clone())?;
    token_client.transfer(
        &wallet,
        &env.current_contract_address(),
        &safe_cast(send_tx_fee_token_amount)?,
    );
    wallet_client.factory_swap_and_bridge(
        &token,
        &(token_amount_u128 - send_tx_fee_token_amount),
        &nonce,
    );

    Ok(())
}
