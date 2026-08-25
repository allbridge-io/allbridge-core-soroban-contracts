use shared::Error;
use soroban_sdk::{contracttype, BytesN, Env};

use crate::storage::DataKey;

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct ChainBridge {
    pub chain_id: u32,
    pub gas_usage: u128,
    pub domain: u32,
    pub other_bridge: BytesN<32>,
}

pub fn get_chain_bridge(env: &Env, chain_id: u32) -> Result<ChainBridge, Error> {
    env.storage()
        .persistent()
        .get(&DataKey::ChainBridge(chain_id))
        .ok_or(Error::UnknownAnotherChain)
}

pub fn has_chain_bridge(env: &Env, chain_id: u32) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::ChainBridge(chain_id))
}

pub fn set_chain_bridge(env: &Env, chain_bridge: &ChainBridge) {
    env.storage()
        .persistent()
        .set(&DataKey::ChainBridge(chain_bridge.chain_id), chain_bridge);
    env.storage().persistent().set(
        &DataKey::Domain(chain_bridge.domain),
        &chain_bridge.chain_id,
    );
}

pub fn has_domain(env: &Env, domain: u32) -> bool {
    env.storage().persistent().has(&DataKey::Domain(domain))
}

pub fn remove_domain(env: &Env, domain: u32) {
    env.storage().persistent().remove(&DataKey::Domain(domain));
}

pub fn get_chain_id_by_domain(env: &Env, domain: u32) -> Result<u32, Error> {
    env.storage()
        .persistent()
        .get(&DataKey::Domain(domain))
        .ok_or(Error::InvalidChainId)
}
