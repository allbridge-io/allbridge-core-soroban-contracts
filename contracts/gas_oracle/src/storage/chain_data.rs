use proc_macros::{data_storage_type, extend_ttl_info, SorobanData};
use shared::{soroban_data::SorobanData, Error};
use soroban_sdk::{contracttype, Env};

pub const PRICE_BUMP_AMOUNT: u32 = 34560; // 2 days

#[contracttype]
#[derive(Clone, Default, SorobanData)]
#[data_storage_type(Temporary)]
#[extend_ttl_info(PRICE_BUMP_AMOUNT, PRICE_BUMP_AMOUNT)]
pub struct ChainData {
    pub price: u128,
    pub gas_price: u128,
}

impl ChainData {
    pub fn get(env: &Env, chain_id: u32) -> Result<ChainData, Error> {
        ChainData::get_by_key(env, &chain_id).map_err(|_| Error::NoGasDataForChain)
    }

    pub fn update_gas_price(
        env: &Env,
        chain_id: u32,
        price: Option<u128>,
        gas_price: Option<u128>,
    ) {
        let prev_chain_data = ChainData::get_by_key(env, &chain_id).unwrap_or(ChainData::default());

        let chain_data = ChainData {
            price: price.unwrap_or(prev_chain_data.price),
            gas_price: gas_price.unwrap_or(prev_chain_data.gas_price),
        };

        env.storage().temporary().set(&chain_id, &chain_data);
        Self::extend_ttl_by_key(env, &chain_id)
    }
}
