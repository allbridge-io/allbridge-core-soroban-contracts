use bridge_storage::*;
use shared::error::Error;
use shared::soroban_data::SimpleSorobanData;
use shared::utils::bump_instance;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Map};

use crate::internal::admin::{
    add_secondary_validator, remove_secondary_validator, set_admin, set_gas_oracle, set_gas_usage,
    set_other_chain_ids, set_primary_validator,
};
use crate::internal::method::{initialize, receive_message, send_message};
use crate::internal::view::{
    get_sent_message_sequence, get_transaction_cost, has_received_message, has_sent_message,
};
use crate::storage::config::Config;

#[contract]
pub struct Messenger;

#[contractimpl]
impl Messenger {
    #[allow(clippy::too_many_arguments)]
    pub fn initialize(
        env: Env,
        admin: Address,
        chain_id: u32,
        native_token_address: Address,
        other_chain_ids: BytesN<32>,
        gas_oracle_address: Address,
        primary_validator_key: BytesN<65>,
        secondary_validator_keys: Map<BytesN<65>, bool>,
    ) -> Result<(), Error> {
        initialize(
            env,
            admin,
            chain_id,
            native_token_address,
            other_chain_ids,
            gas_oracle_address,
            primary_validator_key,
            secondary_validator_keys,
        )
    }

    pub fn send_message(env: Env, message: BytesN<32>, sender: Address) -> Result<u128, Error> {
        bump_instance(&env);

        send_message(env, message, sender)
    }

    pub fn receive_message(
        env: Env,
        message: BytesN<32>,
        primary_signature: BytesN<64>,
        primary_recovery_id: u32,
        secondary_signature: BytesN<64>,
        secondary_recovery_id: u32,
    ) -> Result<(), Error> {
        bump_instance(&env);

        receive_message(
            env,
            message,
            primary_signature,
            primary_recovery_id,
            secondary_signature,
            secondary_recovery_id,
        )
    }

    // admin

    pub fn set_gas_usage(env: Env, chain_id: u32, gas_usage: u128) -> Result<(), Error> {
        bump_instance(&env);

        set_gas_usage(env, chain_id, gas_usage)
    }

    pub fn add_secondary_validator(env: Env, validator_address: BytesN<65>) -> Result<(), Error> {
        bump_instance(&env);

        add_secondary_validator(env, validator_address)
    }

    pub fn remove_secondary_validator(
        env: Env,
        validator_address: BytesN<65>,
    ) -> Result<(), Error> {
        bump_instance(&env);

        remove_secondary_validator(env, validator_address)
    }

    pub fn set_primary_validator(env: Env, validator_address: BytesN<65>) -> Result<(), Error> {
        bump_instance(&env);

        set_primary_validator(env, validator_address)
    }

    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        bump_instance(&env);

        set_admin(env, new_admin)
    }

    pub fn set_gas_oracle(env: Env, new_address: Address) -> Result<(), Error> {
        bump_instance(&env);

        set_gas_oracle(env, new_address)
    }

    pub fn set_other_chain_ids(env: Env, other_chain_ids: BytesN<32>) -> Result<(), Error> {
        bump_instance(&env);

        set_other_chain_ids(env, other_chain_ids)
    }

    //view

    pub fn get_config(env: Env) -> Result<Config, Error> {
        bump_instance(&env);

        Config::get(&env)
    }

    pub fn has_sent_message(env: Env, message: BytesN<32>) -> Result<bool, Error> {
        bump_instance(&env);

        has_sent_message(env, message)
    }

    pub fn has_received_message(env: Env, message: BytesN<32>) -> Result<bool, Error> {
        bump_instance(&env);

        has_received_message(env, message)
    }

    pub fn get_sent_message_sequence(env: Env, message: BytesN<32>) -> Result<u32, Error> {
        bump_instance(&env);

        get_sent_message_sequence(env, message)
    }

    pub fn get_gas_usage(env: Env, chain_id: u32) -> Result<u128, Error> {
        bump_instance(&env);

        GasUsage::get_gas_usage_with_default(env, chain_id)
    }

    pub fn get_transaction_cost(env: Env, chain_id: u32) -> Result<u128, Error> {
        bump_instance(&env);

        get_transaction_cost(&env, chain_id as u8)
    }

    pub fn get_admin(env: Env) -> Result<Address, Error> {
        bump_instance(&env);
        Ok(Admin::get(&env)?.as_address())
    }

    pub fn get_gas_oracle(env: Env) -> Result<Address, Error> {
        Ok(GasOracleAddress::get(&env)?.as_address())
    }
}
