#![allow(clippy::too_many_arguments)]

use send_and_swap_to_vusd::send_and_swap_to_v_usd;
use shared::{error::Error, require, soroban_data::SimpleSorobanData, utils::is_bytesn32_empty};
use soroban_sdk::{Address, BytesN, Env, U256};

use crate::{
    internal::method::{
        convert_bridging_fee_in_tokens_to_native_token, send_and_swap_to_vusd, send_tokens,
    },
    storage::bridge::Bridge,
};

pub fn swap_and_bridge(
    env: Env,
    sender: Address,
    token: BytesN<32>,
    amount: u128,
    recipient: BytesN<32>,
    destination_chain_id: u32,
    receive_token: BytesN<32>,
    nonce: U256,
    gas_amount: u128,
    fee_token_amount: u128,
) -> Result<(), Error> {
    Bridge::get(&env)?.assert_can_swap()?;
    sender.require_auth();

    require!(amount > fee_token_amount, Error::AmountTooLowForFee);
    require!(
        !is_bytesn32_empty(&recipient),
        Error::BridgeToTheZeroAddress
    );

    let fee_token_amount_in_native =
        convert_bridging_fee_in_tokens_to_native_token(&env, &sender, &token, fee_token_amount)?;

    let amount_after_fee = amount - fee_token_amount;
    let v_usd_amount = send_and_swap_to_v_usd(&env, &token, &sender, amount_after_fee)?;

    send_tokens(
        &env,
        v_usd_amount,
        &recipient,
        destination_chain_id,
        &receive_token,
        &nonce,
        gas_amount,
        fee_token_amount_in_native,
        &sender,
    )
}
