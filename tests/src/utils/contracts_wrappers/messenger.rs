use ethers_core::types::Signature;
use shared::{consts::CHAIN_ID, utils::hash_message};
use soroban_sdk::{
    map,
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env, Map, U256,
};

use crate::{
    contracts::messenger,
    utils::{
        consts::GOERLI_CHAIN_ID, contract_id, desoroban_result, get_recover_id, signature_to_bytes,
        unwrap_call_result,
    },
};

use super::{Token, User};

pub const GAS_AMOUNT: u128 = 300_000_000;

#[derive(Debug, Clone)]
pub struct MessengerConfig {
    pub admin: Address,
    pub chain_id: u32,
    pub native_token: Address,
    pub other_chain_ids: BytesN<32>,
    pub gas_oracle: Address,
    pub primary_validator_key: BytesN<65>,
    pub secondary_validator_keys: Map<BytesN<65>, bool>,
}

#[allow(clippy::from_over_into)]
impl Into<messenger::Config> for MessengerConfig {
    fn into(self) -> messenger::Config {
        messenger::Config {
            chain_id: self.chain_id,
            other_chain_ids: self.other_chain_ids,
            primary_validator_key: self.primary_validator_key,
            secondary_validator_keys: self.secondary_validator_keys,
        }
    }
}

impl MessengerConfig {
    pub fn default_config(env: &Env) -> Self {
        MessengerConfig {
            admin: Address::generate(env),
            chain_id: CHAIN_ID,
            native_token: Address::generate(env),
            gas_oracle: Address::generate(env),
            other_chain_ids: BytesN::from_array(
                env,
                &[
                    0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
            ),
            primary_validator_key: BytesN::random(env),
            secondary_validator_keys: map![&env],
        }
    }
}

pub struct Messenger {
    pub id: soroban_sdk::Address,
    pub client: messenger::Client<'static>,
    pub env: Env,
}

impl Messenger {
    pub fn create(env: &Env, config: MessengerConfig) -> Messenger {
        let id = env.register(messenger::WASM, ());
        let client = messenger::Client::new(env, &id);

        client.initialize(
            &config.admin,
            &config.chain_id,
            &config.native_token,
            &config.other_chain_ids,
            &config.gas_oracle,
            &config.primary_validator_key,
            &config.secondary_validator_keys,
        );

        client.set_gas_usage(&2, &GAS_AMOUNT);

        Messenger {
            id,
            client,
            env: env.clone(),
        }
    }

    pub fn send_message(&self, sender: &User, message_hash: &BytesN<32>) -> BytesN<32> {
        unwrap_call_result(
            &self.env,
            desoroban_result::<(), soroban_sdk::ConversionError>(
                self.client
                    .try_send_message(message_hash, &sender.as_address()),
            ),
        );

        message_hash.clone()
    }

    pub fn hash_and_send_message(
        &self,
        sender: &User,
        amount_sp: u128,
        recipient: &Address,
        receive_token: &Token,
        nonce: &U256,
    ) -> BytesN<32> {
        let message_hash = hash_message(
            &self.env,
            amount_sp,
            &contract_id(recipient),
            CHAIN_ID,
            GOERLI_CHAIN_ID,
            &contract_id(&receive_token.id),
            nonce,
        );

        self.send_message(sender, &message_hash)
    }

    pub fn receive_message(
        &self,
        message_hash: &BytesN<32>,
        primary_signature: &Signature,
        secondary_signature: &Signature,
    ) {
        println!("Message: \t\t{:?}", hex::encode(message_hash.to_array()));
        println!("Primary signature: \t{:?}", primary_signature.to_string());
        println!(
            "Secondary signature: \t{:?}",
            secondary_signature.to_string()
        );

        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_receive_message(
                message_hash,
                &signature_to_bytes(&self.env, primary_signature),
                &get_recover_id(primary_signature),
                &signature_to_bytes(&self.env, secondary_signature),
                &get_recover_id(secondary_signature),
            )),
        );
    }

    pub fn other_chain_ids(&self) -> BytesN<32> {
        self.client.get_config().other_chain_ids
    }

    pub fn gas_oracle(&self) -> Address {
        self.client.get_gas_oracle()
    }

    pub fn admin(&self) -> Address {
        self.client.get_admin()
    }

    pub fn primary_validator_key(&self) -> BytesN<65> {
        self.client.get_config().primary_validator_key
    }

    pub fn secondary_validator_keys(&self) -> Map<BytesN<65>, bool> {
        self.client.get_config().secondary_validator_keys
    }

    pub fn has_secondary_validator_key(&self, key: &BytesN<65>) -> bool {
        self.client
            .get_config()
            .secondary_validator_keys
            .contains_key(key.to_owned())
    }

    pub fn upgrade(&self, new_hash: &BytesN<32>) {
        unwrap_call_result(
            &self.env,
            desoroban_result(self.client.try_upgrade(new_hash)),
        );
    }
}
