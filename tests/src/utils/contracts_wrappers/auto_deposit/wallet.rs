use soroban_sdk::{Address, BytesN, Env, U256};

use crate::{
    contracts::auto_deposit_wallet,
    utils::{desoroban_result, unwrap_call_result, CallResult},
};

pub struct AutoDepositWallet {
    pub id: Address,
    pub client: auto_deposit_wallet::Client<'static>,
}

impl AutoDepositWallet {
    pub fn upload_wallet_contract(env: &Env) -> BytesN<32> {
        env.deployer()
            .upload_contract_wasm(auto_deposit_wallet::WASM)
    }

    fn env(&self) -> &Env {
        &self.client.env
    }

    pub fn swap_and_bridge(&self, token_address: Address, nonce: U256) {
        unwrap_call_result(
            self.env(),
            desoroban_result(self.client.try_swap_and_bridge(&token_address, &nonce)),
        )
    }

    pub fn factory_swap_and_bridge(&self, token_address: Address, amount: u128, nonce: U256) {
        unwrap_call_result(
            self.env(),
            desoroban_result(self.client.try_factory_swap_and_bridge(
                &token_address,
                &amount,
                &nonce,
            )),
        )
    }

    pub fn register_token(&self, token: Address) -> CallResult {
        desoroban_result(self.client.try_register_token(&token))
    }

    pub fn register_tokens(&self, tokens: Vec<Address>) -> CallResult {
        let tokens = soroban_sdk::Vec::from_slice(self.env(), tokens.as_slice());

        desoroban_result(self.client.try_register_tokens(&tokens))
    }

    pub fn transfer_unsupported_token(
        &self,
        token_address: Address,
        recipient: Address,
    ) -> CallResult {
        desoroban_result(
            self.client
                .try_transfer_unsupported_token(&token_address, &recipient),
        )
    }

    pub fn is_token_registered(&self, token: Address) -> bool {
        self.client.is_token_registered(&token)
    }

    pub fn min_deposit_token_amount(&self, token: Address) -> u128 {
        self.client.min_deposit_token_amount(&token)
    }
}
