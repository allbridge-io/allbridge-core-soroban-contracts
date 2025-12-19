use shared::{utils::address_to_bytes, Error};
use soroban_sdk::{crypto::Hash, Bytes, BytesN, Env};

use crate::storage::config::Config;

pub fn get_deposit_wallet_salt(
    env: &Env,
    recipient_chain_id: u32,
    recipient: &BytesN<32>,
    recipient_token: &BytesN<32>,
    min_deposit_amount: u128,
    config: &Config,
) -> Result<Hash<32>, Error> {
    let mut bytes = Bytes::new(env);
    bytes.push_back(recipient_chain_id as u8);
    bytes.extend_from_array(&address_to_bytes(env, &config.bridge)?.to_array());
    bytes.extend_from_array(&recipient.to_array());
    bytes.extend_from_array(&recipient_token.to_array());
    bytes.extend_from_array(&min_deposit_amount.to_be_bytes());
    let salt = env.crypto().keccak256(&bytes);
    Ok(salt)
}
