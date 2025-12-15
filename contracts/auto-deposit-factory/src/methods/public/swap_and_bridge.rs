use shared::{require, utils::safe_cast, Error};
use soroban_sdk::{token, Address, Env, U256};

use crate::{methods::internal::get_send_tx_fee_token_amount, wallet};

pub fn swap_and_bridge(
    env: Env,
    wallet: Address,
    token: Address,
    nonce: U256,
) -> Result<(), Error> {
    require!(wallet.exists(), Error::ADWalletNotDeployed);
    let wallet_client = wallet::Client::new(&env, &wallet);
    let token_client = token::Client::new(&env, &token);

    let min_amount = wallet_client
        .try_min_deposit_token_amount(&token)
        .ok()
        .and_then(|inner| inner.ok())
        .unwrap_or_else(|| wallet_client.register_token(&token));

    let token_amount = safe_cast::<_, u128>(token_client.balance(&wallet))?;

    require!(token_amount >= min_amount, Error::ADAmountTooLow);

    let send_tx_fee_token_amount = get_send_tx_fee_token_amount(&env, token.clone())?;

    wallet_client.factory_swap_and_bridge(
        &token,
        &(token_amount - send_tx_fee_token_amount),
        &send_tx_fee_token_amount,
        &nonce,
    );

    Ok(())
}
