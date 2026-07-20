use bridge_storage::Admin;
use shared::{require, Error};
use soroban_sdk::{BytesN, Env};

use crate::storage::{self, ChainBridge};

pub fn update_chain_bridge(
    env: Env,
    chain_id: u32,
    gas_usage: u128,
    domain: u32,
    other_bridge: BytesN<32>,
) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let current = storage::get_chain_bridge(&env, chain_id)?;
    require!(gas_usage > 0, Error::GasUsageNotSet);
    if domain != current.domain {
        require!(!storage::has_domain(&env, domain), Error::InvalidArg);
        storage::remove_domain(&env, current.domain);
    }
    storage::set_chain_bridge(
        &env,
        &ChainBridge {
            chain_id,
            gas_usage,
            domain,
            other_bridge,
        },
    );
    Ok(())
}
