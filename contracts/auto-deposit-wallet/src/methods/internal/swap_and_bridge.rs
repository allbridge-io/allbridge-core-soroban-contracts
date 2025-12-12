use shared::Error;
use soroban_sdk::{Address, Env};

use crate::{bridge, gas_oracle, messenger, storage::config::Config};

pub fn get_bridging_cost_in_tokens(
    env: &Env,
    config: &Config,
    token: &Address,
    bridge_client: &bridge::Client<'_>,
) -> Result<u128, Error> {
    let bridge_config = bridge_client.get_config();
    let messenger_client = messenger::Client::new(env, &bridge_config.messenger);
    let gas_oracle = gas_oracle::Client::new(env, &bridge_client.get_gas_oracle());

    let from_gas_oracle_scaling_factor = bridge_config
        .from_gas_oracle_factor
        .get(token.clone())
        .ok_or(Error::NotFound)?;
    let gas_usage = bridge_client.get_gas_usage(&config.recipient_chain_id)
        + messenger_client.get_gas_usage(&config.recipient_chain_id);
    let cost = gas_oracle.get_transaction_gas_cost_in_usd(&config.recipient_chain_id, &gas_usage);

    Ok(cost / from_gas_oracle_scaling_factor)
}
