#![allow(clippy::too_many_arguments)]

use bridge_storage::*;
use shared::error::Error;
use shared::soroban_data::AnySimpleSorobanData;
use shared::utils::bump_instance;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, U256};

use crate::storage::another_bridge::AnotherBridge;
use crate::{
    internal::{
        admin::set_gas_usage,
        admin::{
            add_bridge_token, add_pool, register_bridge, remove_bridge_token, set_gas_oracle,
            set_rebalancer, set_stop_authority, start_swap, stop_swap,
            withdraw_bridging_fee_in_tokens, withdraw_gas_tokens,
        },
        method::{initialize, receive_tokens, swap, swap_and_bridge},
        view::*,
    },
    storage::bridge::Bridge,
};

#[contract]
pub struct BridgeContract;

#[contractimpl]
impl BridgeContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        messenger: Address,
        gas_oracle: Address,
        native_token: Address,
    ) -> Result<(), Error> {
        initialize(env, admin, messenger, gas_oracle, native_token)
    }

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
        bump_instance(&env);

        swap_and_bridge(
            env,
            sender,
            token,
            amount,
            recipient,
            destination_chain_id,
            receive_token,
            nonce,
            gas_amount,
            fee_token_amount,
        )
    }

    pub fn receive_tokens(
        env: Env,
        sender: Address,
        amount: u128,
        recipient: Address,
        source_chain_id: u32,
        receive_token: BytesN<32>,
        nonce: U256,
        receive_amount_min: u128,
        extra_gas: Option<u128>,
    ) -> Result<(), Error> {
        bump_instance(&env);

        receive_tokens(
            env,
            sender,
            amount,
            recipient,
            source_chain_id,
            receive_token,
            nonce,
            receive_amount_min,
            extra_gas,
        )
    }

    pub fn swap(
        env: Env,
        sender: Address,
        amount: u128,
        token: BytesN<32>,
        receive_token: BytesN<32>,
        recipient: Address,
        receive_amount_min: u128,
    ) -> Result<(), Error> {
        bump_instance(&env);

        swap(
            env,
            sender,
            amount,
            token,
            receive_token,
            recipient,
            receive_amount_min,
        )
    }

    // stop authority

    pub fn stop_swap(env: Env) -> Result<(), Error> {
        bump_instance(&env);

        stop_swap(env)
    }

    pub fn start_swap(env: Env) -> Result<(), Error> {
        bump_instance(&env);

        start_swap(env)
    }

    // admin

    pub fn set_gas_oracle(env: Env, new_address: Address) -> Result<(), Error> {
        bump_instance(&env);

        set_gas_oracle(env, new_address)
    }

    pub fn set_stop_authority(env: Env, stop_authority: Address) -> Result<(), Error> {
        bump_instance(&env);

        set_stop_authority(env, stop_authority)
    }

    pub fn set_rebalancer(env: Env, rebalancer: Address) -> Result<(), Error> {
        bump_instance(&env);

        set_rebalancer(env, rebalancer)
    }

    pub fn set_gas_usage(env: Env, chain_id: u32, gas_usage: u128) -> Result<(), Error> {
        bump_instance(&env);

        set_gas_usage(env, chain_id, gas_usage)
    }

    pub fn register_bridge(
        env: Env,
        chain_id: u32,
        bridge_address: BytesN<32>,
    ) -> Result<(), Error> {
        bump_instance(&env);

        register_bridge(env, chain_id, bridge_address)
    }

    pub fn add_bridge_token(
        env: Env,
        chain_id: u32,
        token_address: BytesN<32>,
    ) -> Result<(), Error> {
        bump_instance(&env);

        add_bridge_token(env, chain_id, &token_address)
    }

    pub fn remove_bridge_token(
        env: Env,
        chain_id: u32,
        token_address: BytesN<32>,
    ) -> Result<(), Error> {
        bump_instance(&env);

        remove_bridge_token(env, chain_id, &token_address)
    }

    pub fn add_pool(env: Env, pool: Address, token: BytesN<32>) -> Result<(), Error> {
        bump_instance(&env);

        add_pool(env, &pool, &token)
    }

    pub fn withdraw_gas_tokens(env: Env, sender: Address, amount: u128) -> Result<(), Error> {
        bump_instance(&env);

        withdraw_gas_tokens(env, sender, amount)
    }

    pub fn withdraw_bridging_fee_in_tokens(
        env: Env,
        sender: Address,
        token_address: Address,
    ) -> Result<(), Error> {
        bump_instance(&env);

        withdraw_bridging_fee_in_tokens(env, sender, token_address)
    }

    // view

    pub fn has_processed_message(env: Env, message: BytesN<32>) -> Result<bool, Error> {
        bump_instance(&env);

        has_processed_message(env, message)
    }

    pub fn has_received_message(env: Env, message: BytesN<32>) -> Result<bool, Error> {
        bump_instance(&env);

        has_received_message(&env, &message)
    }

    pub fn get_pool_address(env: Env, token_address: BytesN<32>) -> Result<Address, Error> {
        bump_instance(&env);

        get_pool_address(env, token_address)
    }

    pub fn get_config(env: Env) -> Result<Bridge, Error> {
        bump_instance(&env);

        Bridge::get(&env)
    }

    pub fn get_stop_authority(env: Env) -> Result<Address, Error> {
        bump_instance(&env);

        Ok(StopAuthority::get(&env)?.as_address())
    }

    pub fn get_transaction_cost(env: Env, chain_id: u32) -> Result<u128, Error> {
        bump_instance(&env);

        get_transaction_cost(&env, chain_id as u8)
    }

    pub fn get_gas_usage(env: Env, chain_id: u32) -> Result<u128, Error> {
        bump_instance(&env);

        GasUsage::get_gas_usage_with_default(env, chain_id)
    }

    pub fn get_admin(env: Env) -> Result<Address, Error> {
        bump_instance(&env);

        Ok(Admin::get(&env)?.as_address())
    }

    pub fn get_gas_oracle(env: Env) -> Result<Address, Error> {
        bump_instance(&env);

        Ok(GasOracleAddress::get(&env)?.as_address())
    }

    pub fn get_another_bridge(env: Env, chain_id: u32) -> Result<AnotherBridge, Error> {
        bump_instance(&env);

        AnotherBridge::get(&env, chain_id)
    }
}
