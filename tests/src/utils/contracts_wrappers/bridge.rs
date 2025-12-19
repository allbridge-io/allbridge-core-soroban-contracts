use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env, U256,
};

use crate::{
    contracts::bridge,
    utils::{contract_id, desoroban_result, float_to_uint, float_to_uint_sp, unwrap_call_result},
};

use super::{Token, User};

pub struct Bridge {
    pub id: soroban_sdk::Address,
    pub client: bridge::Client<'static>,
    pub native_token: Token,
    pub env: Env,
}

impl Bridge {
    pub fn create(
        env: &Env,
        admin: &Address,
        messenger: &Address,
        gas_oracle: &Address,
        native_token: Token,
    ) -> Bridge {
        let id = env.register(bridge::WASM, ());
        let client = bridge::Client::new(env, &id);
        let env = id.env().clone();

        client.initialize(admin, messenger, gas_oracle, &native_token.id);

        Bridge {
            id,
            client,
            native_token,
            env,
        }
    }

    /// (bridge, token)
    pub fn generate_and_register_bridge(
        &self,
        env: &Env,
        chain_id: u32,
    ) -> (BytesN<32>, BytesN<32>) {
        let other_bridge = BytesN::random(env);
        let other_token = BytesN::random(env);

        self.client.register_bridge(&chain_id, &other_bridge);
        self.client.add_bridge_token(&chain_id, &other_token);

        (other_bridge, other_token)
    }

    pub fn generate_and_set_rebalancer(&self, env: &Env) -> Address {
        let rebalancer = Address::generate(env);
        self.client.set_rebalancer(&rebalancer);

        rebalancer
    }

    pub fn generate_and_set_stop_authority(&self, env: &Env) -> Address {
        let stop_authority = Address::generate(env);
        self.client.set_stop_authority(&stop_authority);

        stop_authority
    }

    pub fn receive_tokens(
        &self,
        sender: &Address,
        amount: f64,
        recipient: &User,
        source_chain_id: u32,
        receive_token: &Token,
        nonce: &U256,
        receive_amount_min: f64,
        extra_gas: &Option<f64>,
    ) {
        let amount_sp = float_to_uint_sp(amount);
        let extra_gas = extra_gas.map(|x| float_to_uint(x, 7));
        let receive_amount_min = receive_token.float_to_uint(receive_amount_min);

        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_receive_tokens(
                sender,
                &amount_sp,
                &recipient.as_address(),
                &source_chain_id,
                &contract_id(&receive_token.id),
                nonce,
                &receive_amount_min,
                &extra_gas,
            )),
        );
    }

    pub fn swap_and_bridge(
        &self,
        sender: &User,
        token: &Token,
        amount: f64,
        gas_amount: f64,
        fee_token_amount: f64,
        destination_chain_id: u32,
        recipient: &BytesN<32>,
        receive_token: &BytesN<32>,
        nonce: &U256,
    ) {
        let amount = token.float_to_uint(amount);
        let gas_amount = self.native_token.float_to_uint(gas_amount);
        let fee_token_amount = token.float_to_uint(fee_token_amount);

        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_swap_and_bridge(
                &sender.as_address(),
                &token.id,
                &amount,
                recipient,
                &destination_chain_id,
                receive_token,
                nonce,
                &gas_amount,
                &fee_token_amount,
            )),
        );
    }

    pub fn set_admin(&self, admin: &Address) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_set_admin(admin)),
        );
    }

    pub fn set_messenger(&self, messenger: &Address) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_set_messenger(messenger)),
        );
    }

    pub fn set_stop_authority(&self, stop_authority: &Address) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_set_stop_authority(stop_authority)),
        );
    }

    pub fn set_gas_oracle(&self, gas_oracle: &Address) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_set_gas_oracle(gas_oracle)),
        );
    }

    pub fn set_rebalancer(&self, rebalancer: &Address) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_set_rebalancer(rebalancer)),
        );
    }

    pub fn add_bridge_token(&self, chain_id: u32, token_address: &BytesN<32>) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_add_bridge_token(&chain_id, token_address)),
        );
    }

    pub fn register_bridge(&self, chain_id: u32, bridge_address: &BytesN<32>) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_register_bridge(&chain_id, bridge_address)),
        );
    }

    pub fn stop_swap(&self) {
        unwrap_call_result(&self.env, desoroban_result(self.client.try_stop_swap()));
    }

    pub fn start_swap(&self) {
        unwrap_call_result(&self.env, desoroban_result(self.client.try_start_swap()));
    }

    pub fn upgrade(&self, new_hash: &BytesN<32>) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_upgrade(new_hash)),
        );
    }
}
