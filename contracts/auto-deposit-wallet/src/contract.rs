use shared::{utils::extend_ttl_instance, Error};
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec, U256};

use crate::methods::{
    admin::{register_token, register_tokens, transfer_unsupported_token},
    public::{constructor, factory_swap_and_bridge, swap_and_bridge},
    view::{is_token_registered, min_deposit_token_amount},
};

#[contract]
pub struct AutoDepositWalletContract;

#[contractimpl]
impl AutoDepositWalletContract {
    pub fn __constructor(
        env: Env,
        admin: Address,
        bridge: Address,
        factory: Address,
        recipient_chain_id: u32,
        recipient: BytesN<32>,
        recipient_token: BytesN<32>,
        min_deposit_amount: u128,
    ) -> Result<(), Error> {
        constructor(
            env,
            admin,
            bridge,
            factory,
            recipient_chain_id,
            recipient,
            recipient_token,
            min_deposit_amount,
        )
    }

    pub fn swap_and_bridge(env: Env, token_address: Address, nonce: U256) -> Result<(), Error> {
        extend_ttl_instance(&env);

        swap_and_bridge(env, token_address, nonce)
    }

    pub fn factory_swap_and_bridge(
        env: Env,
        token_address: Address,
        amount: u128,
        nonce: U256,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);

        factory_swap_and_bridge(env, token_address, amount, nonce)
    }

    pub fn register_token(env: Env, token: Address) -> Result<(), Error> {
        extend_ttl_instance(&env);

        register_token(env, token)
    }

    pub fn register_tokens(env: Env, tokens: Vec<Address>) -> Result<(), Error> {
        extend_ttl_instance(&env);

        register_tokens(env, tokens)
    }

    // admin

    pub fn transfer_unsupported_token(
        env: Env,
        token_address: Address,
        recipient: Address,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);

        transfer_unsupported_token(env, token_address, recipient)
    }

    // view

    pub fn is_token_registered(env: Env, token: Address) -> Result<bool, Error> {
        is_token_registered(env, token)
    }

    pub fn min_deposit_token_amount(env: Env, token: Address) -> Result<u128, Error> {
        min_deposit_token_amount(env, token)
    }
}
