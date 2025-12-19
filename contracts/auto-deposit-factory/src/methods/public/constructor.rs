use bridge_storage::{Admin, GasOracleAddress, NativeToken};
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, BytesN, Env};

use crate::storage::config::Config;

pub fn constructor(
    env: Env,
    admin: Address,
    native_token_address: Address,
    gas_oracle_address: Address,
    bridge: Address,
    send_tx_cost: u128,
    wallet_wasm_hash: BytesN<32>,
) -> Result<(), Error> {
    let config = Config::new(&env, bridge, send_tx_cost, wallet_wasm_hash);

    Admin(admin).save(&env);
    GasOracleAddress(gas_oracle_address).save(&env);
    NativeToken(native_token_address).save(&env);

    config.save(&env);

    Ok(())
}
