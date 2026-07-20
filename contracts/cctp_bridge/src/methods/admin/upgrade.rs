use bridge_storage::Admin;
use shared::Error;
use soroban_sdk::{BytesN, Env};

pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    env.deployer().update_current_contract_wasm(new_wasm_hash);
    Ok(())
}
