use shared::{
    utils::{address_to_bytes, safe_cast},
    Error,
};
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    vec, Address, Env, IntoVal, Symbol, U256,
};

use crate::{bridge, gas_oracle, messenger, storage::config::Config};

pub fn swap_and_bridge(
    env: &Env,
    config: &Config,
    token: &Address,
    amount: u128,
    nonce: &U256,
) -> Result<(), Error> {
    let bridge_client = bridge::Client::new(env, &config.bridge);
    let cost_in_tokens = get_bridging_cost_in_tokens(env, config, token, &bridge_client)? + 1;
    let amount_without_fee = amount - cost_in_tokens;

    let pool = bridge_client
        .get_config()
        .pools
        .get(address_to_bytes(&env, &token)?)
        .ok_or(Error::NotFound)?;

    env.authorize_as_current_contract(vec![
        env,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token.clone(),
                fn_name: Symbol::new(&env, "transfer"),
                args: vec![
                    env,
                    env.current_contract_address().to_val(),
                    config.bridge.to_val(),
                    safe_cast::<_, i128>(cost_in_tokens)?.into_val(env),
                ],
            },
            sub_invocations: vec![&env],
        }),
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token.clone(),
                fn_name: Symbol::new(&env, "transfer"),
                args: vec![
                    env,
                    env.current_contract_address().to_val(),
                    pool.to_val(),
                    safe_cast::<_, i128>(amount_without_fee)?.into_val(env),
                ],
            },
            sub_invocations: vec![&env],
        }),
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: pool.clone(),
                fn_name: Symbol::new(&env, "swap_to_v_usd"),
                args: vec![
                    env,
                    env.current_contract_address().to_val(),
                    amount_without_fee.into_val(env),
                    false.into_val(env),
                ],
            },
            sub_invocations: vec![env],
        }),
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
