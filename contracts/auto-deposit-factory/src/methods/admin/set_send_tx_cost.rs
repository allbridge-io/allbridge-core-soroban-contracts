use bridge_storage::Admin;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::Env;

use crate::storage::config::Config;

pub fn set_send_tx_cost(env: Env, send_tx_cost: u128) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;

    Config::update(&env, |config| {
        config.send_tx_cost = send_tx_cost;
        Ok(())
    })
}
