use shared::Error;
use soroban_sdk::{token, vec, Address, Env, IntoVal, U256};

use crate::{bridge, gas_oracle, messenger, storage::config::Config};

pub fn swap_and_bridge(
    env: &Env,
    config: &Config,
    token: &Address,
    amount: u128,
    nonce: &U256,
) -> Result<(), Error> {
    let bridge_client = bridge::Client::new(env, &config.bridge);
    let cost_in_tokens = get_bridging_cost_in_tokens(env, config, token, &bridge_client)?;

    let token_client = token::Client::new(&env, &token);

    env.current_contract_address().require_auth_for_args(vec![
        env,
        "transfer".into_val(env),
        env.current_contract_address().to_val(),
        config.bridge.to_val(),
        (cost_in_tokens as i128).into_val(env),
    ]);

    bridge_client.swap_and_bridge(
        &env.current_contract_address(),
        token,
        &amount,
        &config.recipient,
        &config.recipient_chain_id,
        &config.recipient_token,
        nonce,
        &0,
        &cost_in_tokens,
    );

    Ok(())
}

fn get_bridging_cost_in_tokens(
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

    Ok(cost / from_gas_oracle_scaling_factor + 1)
}
