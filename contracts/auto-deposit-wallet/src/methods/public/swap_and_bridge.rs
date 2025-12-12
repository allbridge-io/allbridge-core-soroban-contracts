use shared::{
    require,
    soroban_data::SimpleSorobanData,
    utils::{address_to_bytes, safe_cast},
    Error,
};
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    token, vec, Address, Env, IntoVal, Symbol, U256,
};

use crate::{
    bridge,
    methods::internal::{self, get_bridging_cost_in_tokens},
    storage::config::Config,
};

pub fn swap_and_bridge(env: Env, token_address: Address, nonce: U256) -> Result<(), Error> {
    let config = Config::get(&env)?;
    let token_client = token::Client::new(&env, &token_address);
    let token_amount = safe_cast::<_, u128>(token_client.balance(&env.current_contract_address()))?;

    let min_amount = match config.min_deposit_token_amount.get(token_address.clone()) {
        Some(v) => v,
        None => internal::register_token(&env, &token_address)?,
    };

    require!(token_amount >= min_amount, Error::ADAmountTooLow);

    let bridge_client = bridge::Client::new(&env, &config.bridge);
    let cost_in_tokens =
        get_bridging_cost_in_tokens(&env, &config, &token_address, &bridge_client)? + 1;
    let pool = bridge_client
        .get_config()
        .pools
        .get(address_to_bytes(&env, &token_address)?)
        .ok_or(Error::NotFound)?;

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
                    safe_cast::<_, i128>(cost_in_tokens).into_val(&env),
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
                    safe_cast::<_, i128>(token_amount - cost_in_tokens).into_val(&env),
                ],
            },
            sub_invocations: vec![&env],
        }),
    ]);

    bridge_client.swap_and_bridge(
        &env.current_contract_address(),
        &token_address,
        &token_amount,
        &config.recipient,
        &config.recipient_chain_id,
        &config.recipient_token,
        &nonce,
        &0,
        &cost_in_tokens,
    );

    Ok(())
}
