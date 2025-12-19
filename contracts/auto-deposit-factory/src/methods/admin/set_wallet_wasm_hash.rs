use bridge_storage::Admin;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{BytesN, Env};

use crate::storage::config::Config;

pub fn set_wallet_wasm_hash(env: Env, wallet_wasm_hash: BytesN<32>) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;

    Config::update(&env, |config| {
        config.wallet_wasm_hash = wallet_wasm_hash;
        Ok(())
    })
}
