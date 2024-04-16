use soroban_sdk::{Address, BytesN, Env};

use crate::{
    contracts::gas_oracle::{self, ChainData},
    utils::{desoroban_result, unwrap_call_result},
};

pub struct GasOracle {
    pub id: soroban_sdk::Address,
    pub client: gas_oracle::Client<'static>,
    pub env: Env,
}

impl GasOracle {
    pub fn create(env: &Env, admin: &Address) -> GasOracle {
        let id = env.register_contract_wasm(None, gas_oracle::WASM);
        let client = gas_oracle::Client::new(env, &id);

        client.initialize(admin);

        GasOracle {
            id,
            client,
            env: env.clone(),
        }
    }

    pub fn initialize(&self, admin: &Address) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_initialize(admin)),
        );
    }

    pub fn get_gas_price(&self, chain_id: u32) -> ChainData {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_get_gas_price(&chain_id)),
        )
    }

    pub fn get_gas_cost_in_native_token(&self, chain_id: u32, gas_amount: u128) -> u128 {
        unwrap_call_result(
            &self.env,
            desoroban_result(
                self.client
                    .try_get_gas_cost_in_native_token(&chain_id, &gas_amount),
            ),
        )
    }

    pub fn set_price(&self, chain_id: u32, price: Option<u128>, gas_price: Option<u128>) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_set_price(&chain_id, &price, &gas_price)),
        );
    }

    pub fn get_admin(&self) -> Address {
        unwrap_call_result(&self.env, desoroban_result(self.client.try_get_admin()))
    }

    pub fn set_admin(&self, admin: &Address) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_set_admin(admin)),
        );
    }

    pub fn upgrade(&self, new_hash: &BytesN<32>) {
        unwrap_call_result(&self.env, desoroban_result(self.client.try_upgrade(new_hash)));
    }
}
