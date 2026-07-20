#![allow(clippy::too_many_arguments)]

use shared::{utils::extend_ttl_instance, Error};
use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN, Env};

use crate::{
    methods::{admin, public, view},
    storage::ChainBridge,
};

#[contract]
pub struct CctpBridgeContract;

#[contractimpl]
impl CctpBridgeContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        usdc_token: Address,
        token_messenger_minter: Address,
        message_transmitter: Address,
        gas_oracle: Address,
        native_token: Address,
        min_finality_threshold: u32,
        max_fee_share: i128,
        admin_fee_share_bp: u64,
        bridging_fee_conversion_factor: u128,
    ) -> Result<(), Error> {
        public::initialize(
            env,
            admin,
            usdc_token,
            token_messenger_minter,
            message_transmitter,
            gas_oracle,
            native_token,
            min_finality_threshold,
            max_fee_share,
            admin_fee_share_bp,
            bridging_fee_conversion_factor,
        )
    }

    pub fn bridge(
        env: Env,
        sender: Address,
        amount: u128,
        recipient: BytesN<32>,
        destination_chain_id: u32,
        gas_amount: u128,
        fee_token_amount: u128,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);
        public::bridge(
            env,
            sender,
            amount,
            recipient,
            destination_chain_id,
            gas_amount,
            fee_token_amount,
        )
    }

    pub fn bridge_with_hook(
        env: Env,
        sender: Address,
        amount: u128,
        recipient: BytesN<32>,
        destination_chain_id: u32,
        gas_amount: u128,
        fee_token_amount: u128,
        hook_data: Bytes,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);
        public::bridge_with_hook(
            env,
            sender,
            amount,
            recipient,
            destination_chain_id,
            gas_amount,
            fee_token_amount,
            hook_data,
        )
    }

    pub fn receive_tokens(
        env: Env,
        sender: Address,
        message_id: BytesN<32>,
        message: Bytes,
        attestation: Bytes,
        extra_gas_amount: u128,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);
        public::receive_tokens(env, sender, message_id, message, attestation, extra_gas_amount)
    }

    pub fn register_chain_bridge(
        env: Env,
        chain_id: u32,
        gas_usage: u128,
        domain: u32,
        other_bridge: BytesN<32>,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::register_chain_bridge(env, chain_id, gas_usage, domain, other_bridge)
    }

    pub fn update_chain_bridge(
        env: Env,
        chain_id: u32,
        gas_usage: u128,
        domain: u32,
        other_bridge: BytesN<32>,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::update_chain_bridge(env, chain_id, gas_usage, domain, other_bridge)
    }

    pub fn set_admin_fee_share(env: Env, admin_fee_share_bp: u64) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::set_admin_fee_share(env, admin_fee_share_bp)
    }

    pub fn set_max_fee_share(env: Env, value: i128) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::set_max_fee_share(env, value)
    }

    pub fn set_min_finality_threshold(env: Env, value: u32) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::set_min_finality_threshold(env, value)
    }

    pub fn set_gas_oracle(env: Env, gas_oracle: Address) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::set_gas_oracle(env, gas_oracle)
    }

    pub fn set_token_messenger_minter(
        env: Env,
        token_messenger_minter: Address,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::set_token_messenger_minter(env, token_messenger_minter)
    }

    pub fn set_message_transmitter(env: Env, message_transmitter: Address) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::set_message_transmitter(env, message_transmitter)
    }

    pub fn set_fee_conversion_factor(env: Env, value: u128) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::set_fee_conversion_factor(env, value)
    }

    pub fn withdraw_gas_tokens(env: Env, sender: Address, amount: u128) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::withdraw_gas_tokens(env, sender, amount)
    }

    pub fn withdraw_bridging_fee_in_tokens(
        env: Env,
        sender: Address,
        token_address: Address,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::withdraw_bridging_fee_in_tokens(env, sender, token_address)
    }

    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        extend_ttl_instance(&env);
        admin::set_admin(env, new_admin)
    }

    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Result<(), Error> {
        admin::upgrade(env, new_wasm_hash)
    }

    pub fn get_chain_bridge(env: Env, chain_id: u32) -> Result<ChainBridge, Error> {
        view::get_chain_bridge(env, chain_id)
    }

    pub fn get_domain_by_chain_id(env: Env, chain_id: u32) -> Result<u32, Error> {
        view::get_domain_by_chain_id(env, chain_id)
    }

    pub fn get_transaction_cost(env: Env, chain_id: u32) -> Result<u128, Error> {
        view::get_transaction_cost(env, chain_id)
    }

    pub fn get_bridging_cost_in_tokens(env: Env, chain_id: u32) -> Result<u128, Error> {
        view::get_bridging_cost_in_tokens(env, chain_id)
    }

    pub fn native_fee_balance(env: Env) -> Result<i128, Error> {
        view::native_fee_balance(env)
    }

    pub fn bridging_fee_in_tokens(env: Env, token_address: Address) -> i128 {
        view::bridging_fee_in_tokens(env, token_address)
    }

    pub fn admin(env: Env) -> Result<Address, Error> {
        view::admin(env)
    }

    pub fn usdc_token(env: Env) -> Result<Address, Error> {
        view::usdc_token(env)
    }

    pub fn token_messenger_minter(env: Env) -> Result<Address, Error> {
        view::token_messenger_minter(env)
    }

    pub fn message_transmitter(env: Env) -> Result<Address, Error> {
        view::message_transmitter(env)
    }

    pub fn gas_oracle(env: Env) -> Result<Address, Error> {
        view::gas_oracle(env)
    }

    pub fn native_token(env: Env) -> Result<Address, Error> {
        view::native_token(env)
    }

    pub fn min_finality_threshold(env: Env) -> Result<u32, Error> {
        view::min_finality_threshold(env)
    }

    pub fn max_fee_share(env: Env) -> Result<i128, Error> {
        view::max_fee_share(env)
    }

    pub fn admin_fee_share_bp(env: Env) -> Result<u64, Error> {
        view::admin_fee_share_bp(env)
    }

    pub fn bridging_fee_conversion_factor(env: Env) -> Result<u128, Error> {
        view::bridging_fee_conversion_factor(env)
    }
}
