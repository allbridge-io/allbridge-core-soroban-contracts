use crate::{bridge, methods::internal::get_bridging_cost_in_tokens, storage::config::Config};
use shared::{
    soroban_data::SimpleSorobanData,
    utils::{address_to_bytes, safe_cast},
    Error,
};
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    vec, Address, Env, IntoVal as _, Symbol, U256,
};

pub fn factory_swap_and_bridge(
    env: Env,
    token_address: Address,
    amount: u128,
    nonce: U256,
) -> Result<(), Error> {
    let config = Config::get(&env)?;
    config.factory.require_auth();

    let bridge_client = bridge::Client::new(&env, &config.bridge);
    let pool = bridge_client
        .get_config()
        .pools
        .get(address_to_bytes(&env, &token_address)?)
        .ok_or(Error::NotFound)?;

    let cost_in_tokens =
        get_bridging_cost_in_tokens(&env, &config, &token_address, &bridge_client)? + 1;

    env.authorize_as_current_contract(vec![
        &env,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token_address.clone(),
                fn_name: Symbol::new(&env, "transfer"),
                args: vec![
                    &env,
                    env.current_contract_address().to_val(),
                    config.bridge.to_val(),
                    safe_cast::<_, i128>(cost_in_tokens)?.into_val(&env),
                ],
            },
            sub_invocations: vec![&env],
        }),
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token_address.clone(),
                fn_name: Symbol::new(&env, "transfer"),
                args: vec![
                    &env,
                    env.current_contract_address().to_val(),
                    pool.to_val(),
                    safe_cast::<_, i128>(amount - cost_in_tokens)?.into_val(&env),
                ],
            },
            sub_invocations: vec![&env],
        }),
    ]);

    bridge_client.swap_and_bridge(
        &env.current_contract_address(),
        &token_address,
        &amount,
        &config.recipient,
        &config.recipient_chain_id,
        &config.recipient_token,
        &nonce,
        &0,
        &cost_in_tokens,
    );

    Ok(())
}
