use shared::{require, Error, Event};
use soroban_sdk::{token, Address, BytesN, Env};

use crate::{
    events::{ReceiveFee, TokensSent},
    external::TokenMessengerMinterClient,
    methods::{internal, view, MAX_FEE_SHARE_DENOMINATOR},
    storage,
};

#[allow(clippy::too_many_arguments)]
pub fn bridge(
    env: Env,
    sender: Address,
    amount: u128,
    recipient: BytesN<32>,
    destination_chain_id: u32,
    gas_amount: u128,
    fee_token_amount: u128,
) -> Result<(), Error> {
    sender.require_auth();
    require!(amount > fee_token_amount, Error::AmountTooLowForFee);

    let config = storage::get_config(&env)?;
    let chain_bridge = storage::get_chain_bridge(&env, destination_chain_id)?;
    let transaction_cost = view::get_transaction_cost(env.clone(), destination_chain_id)?;
    let converted_fee = internal::convert_fee_token_amount(&env, &config, fee_token_amount)?;
    require!(
        gas_amount + converted_fee >= transaction_cost,
        Error::NotEnoughFee
    );

    let contract = env.current_contract_address();
    token::Client::new(&env, &config.usdc_token).transfer(&sender, &contract, &(amount as i128));
    if gas_amount > 0 {
        token::Client::new(&env, &config.native_token).transfer(
            &sender,
            &contract,
            &(gas_amount as i128),
        );
    }

    let bridge_amount = amount - fee_token_amount;
    let mut admin_fee =
        bridge_amount * config.admin_fee_share_bp as u128 / internal::admin_fee_denominator();
    if config.admin_fee_share_bp > 0 && admin_fee == 0 {
        admin_fee = 1;
    }
    let mut net_burn_amount = bridge_amount - admin_fee;
    let dust = net_burn_amount % 10;
    if dust > 0 {
        net_burn_amount -= dust;
        admin_fee += dust;
    }
    require!(net_burn_amount > 0, Error::AmountTooLowForFee);

    let max_fee = net_burn_amount
        .checked_mul(config.max_fee_share as u128)
        .and_then(|value| value.checked_div(MAX_FEE_SHARE_DENOMINATOR as u128))
        .and_then(|value| value.checked_add(1))
        .ok_or(Error::U256Overflow)?;

    token::Client::new(&env, &config.usdc_token).approve(
        &contract,
        &config.token_messenger_minter,
        &(net_burn_amount as i128),
        &(env.ledger().sequence() + 1),
    );

    TokenMessengerMinterClient::new(&env, &config.token_messenger_minter).deposit_for_burn(
        &contract,
        &(net_burn_amount as i128),
        &chain_bridge.domain,
        &recipient,
        &config.usdc_token,
        &chain_bridge.other_bridge,
        &(max_fee as i128),
        &config.min_finality_threshold,
    );

    TokensSent {
        amount: net_burn_amount,
        admin_fee,
        sender,
        recipient,
        destination_chain_id,
    }
    .publish(&env);
    ReceiveFee {
        bridge_transaction_cost: transaction_cost,
        message_transaction_cost: 0,
        extra_gas: gas_amount,
    }
    .publish(&env);

    Ok(())
}
