use proc_macros::{bump_info, data_storage_type, SorobanData};
use shared::{soroban_data::SorobanData, Error};
use soroban_sdk::{contracttype, Env};

use crate::data_key::{DataKey, PRICE_BUMP_AMOUNT};

#[contracttype]
#[derive(Clone, Default, SorobanData)]
#[data_storage_type(Temporary)]
#[bump_info(PRICE_BUMP_AMOUNT, PRICE_BUMP_AMOUNT)]
pub struct ChainData {
    pub price: u128,
    pub gas_price: u128,
}

impl ChainData {
    pub fn get(env: &Env, chain_id: u32) -> Result<ChainData, Error> {
        ChainData::get_by_key(env, &DataKey::ChainData(chain_id))
            .map_err(|_| Error::NoGasDataForChain)
    }

    pub fn update_gas_price(
        env: &Env,
        chain_id: u32,
        price: Option<u128>,
        gas_price: Option<u128>,
    ) {
        let key = DataKey::ChainData(chain_id);
        let prev_chain_data = ChainData::get_by_key(env, &key).unwrap_or(ChainData::default());

        let chain_data = ChainData {
            price: price.unwrap_or(prev_chain_data.price),
            gas_price: gas_price.unwrap_or(prev_chain_data.gas_price),
        };

        env.storage().temporary().set(&key, &chain_data);
        Self::bump_by_key(env, &key)
    }
}
