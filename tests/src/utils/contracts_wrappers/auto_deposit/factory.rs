use soroban_sdk::{contracttype, Address, BytesN, Env, U256};

use crate::{
    contracts::auto_deposit_factory::{self, Config},
    utils::{desoroban_result, unwrap_call_result, CallResult},
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoDepositWalletDeployed {
    pub address: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositAddressCreation {
    pub recipient: Address,
    pub recipient_token: Address,
    pub min_deposit_amount: u128,
    pub chain_ids: soroban_sdk::Vec<u32>,
}

pub struct AutoDepositFactory {
    pub id: Address,
    pub client: auto_deposit_factory::Client<'static>,
}

impl AutoDepositFactory {
    pub fn create(
        env: &Env,
        admin: Address,
        native_token_address: Address,
        gas_oracle_address: Address,
        bridge: Address,
        send_tx_cost: u128,
        wallet_wasm_hash: BytesN<32>,
    ) -> Self {
        let id = env.register(
            auto_deposit_factory::WASM,
            (
                admin,
                native_token_address,
                gas_oracle_address,
                bridge,
                send_tx_cost,
                wallet_wasm_hash,
            ),
        );
        let client = auto_deposit_factory::Client::new(env, &id);

        Self { id, client }
    }

    fn env(&self) -> &Env {
        &self.client.env
    }

    pub fn create_deposit_wallet(
        &self,
        sender: Address,
        recipient: Address,
        recipient_token: Address,
        min_deposit_amount: u128,
        gas_amount: u128,
        fee_token_amount: u128,
        chain_ids: Vec<u32>,
    ) {
        let chain_ids = soroban_sdk::Vec::from_slice(self.env(), chain_ids.as_slice());

        unwrap_call_result(
            self.env(),
            desoroban_result(self.client.try_create_deposit_wallet(
                &sender,
                &recipient,
                &recipient_token,
                &min_deposit_amount,
                &gas_amount,
                &fee_token_amount,
                &chain_ids,
            )),
        );
    }

    pub fn deploy_deposit_wallet(
        &self,
        recipient_chain_id: u32,
        recipient: BytesN<32>,
        recipient_token: BytesN<32>,
        min_deposit_amount: u128,
    ) -> Address {
        unwrap_call_result(
            self.env(),
            desoroban_result(self.client.try_deploy_deposit_wallet(
                &recipient_chain_id,
                &recipient,
                &recipient_token,
                &min_deposit_amount,
            )),
        );
        crate::utils::get_latest_event_unchecked::<AutoDepositWalletDeployed>(self.env()).address
    }

    pub fn swap_and_bridge(&self, wallet_address: Address, token_address: Address, nonce: U256) {
        unwrap_call_result(
            self.env(),
            desoroban_result(self.client.try_swap_and_bridge(
                &wallet_address,
                &token_address,
                &nonce,
            )),
        );
    }

    pub fn create_swap_and_bridge(
        &self,
        recipient_chain_id: u32,
        recipient: BytesN<32>,
        recipient_token: BytesN<32>,
        min_deposit_amount: u128,
        token_address: Address,
        nonce: U256,
    ) {
        unwrap_call_result(
            self.env(),
            desoroban_result(self.client.try_create_swap_and_bridge(
                &recipient_chain_id,
                &recipient,
                &recipient_token,
                &min_deposit_amount,
                &token_address,
                &nonce,
            )),
        );
    }

    pub fn register_token(&self, token_address: Address) -> CallResult {
        desoroban_result(self.client.try_register_token(&token_address))
    }

    pub fn unregister_token(&self, token_address: Address) -> CallResult {
        desoroban_result(self.client.try_unregister_token(&token_address))
    }

    pub fn config(&self) -> Config {
        self.client.get_config()
    }

    pub fn admin(&self) -> Address {
        self.client.get_admin()
    }

    pub fn gas_oracle(&self) -> Address {
        self.client.get_gas_oracle()
    }

    pub fn upgrade(&self, new_hash: &BytesN<32>) {
        unwrap_call_result(
            self.env(),
            desoroban_result(self.client.try_upgrade(new_hash)),
        )
    }
}
