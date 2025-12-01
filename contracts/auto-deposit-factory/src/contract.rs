use bridge_storage::Admin;
use shared::{soroban_data::SimpleSorobanData, utils::extend_ttl_instance, Error};
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec, U256};

use crate::{
    methods::{
        admin::{
            register_token, set_admin, set_gas_oracle, set_gas_usage, set_send_tx_cost,
            unregister_token, withdraw, withdraw_gas_tokens,
        },
        public::{constructor, create_deposit_wallet, deploy_deposit_wallet, swap_and_bridge},
        view::{
            deposit_wallet_address, deposit_wallet_address_by_salt, get_admin, get_gas_oracle,
            get_gas_usage, get_send_tx_fee_token_amount,
        },
    },
    storage::config::Config,
};

#[contract]
pub struct AutoDepositFactoryContract;

#[contractimpl]
impl AutoDepositFactoryContract {
    pub fn __constructor(
        env: Env,
        admin: Address,
        chain_id: u32,
        native_token_address: Address,
        gas_oracle_address: Address,
        bridge: Address,
        send_tx_cost: u128,
        wallet_wasm_hash: BytesN<32>,
    ) -> Result<(), Error> {
        constructor(
            env,
            admin,
            chain_id,
            native_token_address,
            gas_oracle_address,
            bridge,
            send_tx_cost,
            wallet_wasm_hash,
        )
    }

    pub fn create_deposit_wallet(
        env: Env,
        sender: Address,
        recipient: Address,
        recipient_token: Address,
        min_deposit_amount: u128,
        fee_token_amount: u128,
        chain_ids: Vec<u32>,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);

        create_deposit_wallet(
            env,
            sender,
            recipient,
            recipient_token,
            min_deposit_amount,
            fee_token_amount,
            chain_ids,
        )
    }

    pub fn deploy_deposit_wallet(
        env: Env,
        recipient_chain_id: u32,
        recipient: BytesN<32>,
        recipient_token: BytesN<32>,
        min_deposit_amount: u128,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);

        deploy_deposit_wallet(
            env,
            recipient_chain_id,
            recipient,
            recipient_token,
            min_deposit_amount,
        )
    }

    pub fn swap_and_bridge(
        env: Env,
        wallet_address: Address,
        token_address: Address,
        nonce: U256,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);

        swap_and_bridge(env, wallet_address, token_address, nonce)
    }

    // admin

    pub fn set_send_tx_cost(env: Env, send_tx_cost: u128) -> Result<(), Error> {
        extend_ttl_instance(&env);

        set_send_tx_cost(env, send_tx_cost)
    }

    pub fn register_token(env: Env, token_address: Address) -> Result<(), Error> {
        extend_ttl_instance(&env);

        register_token(env, token_address)
    }

    pub fn unregister_token(env: Env, token_address: Address) -> Result<(), Error> {
        extend_ttl_instance(&env);

        unregister_token(env, token_address)
    }

    pub fn set_gas_usage(env: Env, chain_id: u32, gas_usage: u128) -> Result<(), Error> {
        extend_ttl_instance(&env);

        set_gas_usage(env, chain_id, gas_usage)
    }

    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        extend_ttl_instance(&env);

        set_admin(env, new_admin)
    }

    pub fn set_gas_oracle(env: Env, new_address: Address) -> Result<(), Error> {
        extend_ttl_instance(&env);

        set_gas_oracle(env, new_address)
    }

    pub fn withdraw_gas_tokens(env: Env, sender: Address, amount: u128) -> Result<(), Error> {
        extend_ttl_instance(&env);

        withdraw_gas_tokens(env, sender, amount)
    }

    pub fn withdraw(
        env: Env,
        sender: Address,
        token_address: Address,
        amount: u128,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);

        withdraw(env, sender, token_address, amount)
    }

    // view

    pub fn get_config(env: Env) -> Result<Config, Error> {
        Config::get(&env)
    }

    pub fn get_send_tx_fee_token_amount(env: Env, token_address: Address) -> Result<u128, Error> {
        get_send_tx_fee_token_amount(env, token_address)
    }

    pub fn get_gas_usage(env: Env, chain_id: u32) -> Result<u128, Error> {
        get_gas_usage(env, chain_id)
    }

    pub fn get_admin(env: Env) -> Result<Address, Error> {
        get_admin(env)
    }

    pub fn get_gas_oracle(env: Env) -> Result<Address, Error> {
        get_gas_oracle(env)
    }

    pub fn deposit_wallet_address(
        env: &Env,
        recipient_chain_id: u32,
        recipient: &BytesN<32>,
        recipient_token: &BytesN<32>,
        min_deposit_amount: u128,
    ) -> Result<Address, Error> {
        deposit_wallet_address(
            env,
            recipient_chain_id,
            recipient,
            recipient_token,
            min_deposit_amount,
        )
    }

    pub fn deposit_wallet_address_by_salt(env: Env, salt: BytesN<32>) -> Result<Address, Error> {
        deposit_wallet_address_by_salt(env, salt)
    }

    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Result<(), Error> {
        Admin::require_exist_auth(&env)?;

        env.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}
