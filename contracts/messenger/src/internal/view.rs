use crate::storage::get_gas_oracle_client;
use crate::storage::message::Message;
use bridge_storage::*;
use shared::error::Error;
use soroban_sdk::{BytesN, Env};

pub fn has_sent_message(env: Env, message: BytesN<32>) -> Result<bool, Error> {
    Ok(Message::has_sent_message(&env, message))
}

pub fn has_received_message(env: Env, message: BytesN<32>) -> Result<bool, Error> {
    Ok(Message::has_received_message(&env, message))
}

pub fn get_sent_message_sequence(env: Env, message: BytesN<32>) -> Result<u32, Error> {
    Ok(Message::get_sent_message_sequence(&env, message))
}

pub fn get_transaction_cost(env: &Env, chain_id: u8) -> Result<u128, Error> {
    let gas_oracle = get_gas_oracle_client(env)?;
    let gas_usage = GasUsage::get_by_chain(env, chain_id)?;

    Ok(gas_oracle.get_gas_cost_in_native_token(&(chain_id as u32), &gas_usage))
}
