use proc_macros::{
    bump_info_instance, data_storage_type, symbol_key, SorobanData, SorobanSimpleData,
};
use shared::{error::Error, soroban_data::SimpleSorobanData};
use soroban_sdk::{contracttype, Env, Map};

#[contracttype]
#[derive(SorobanData, SorobanSimpleData)]
#[symbol_key("GasUsage")]
#[data_storage_type(Instance)]
#[bump_info_instance]
pub struct GasUsage(pub Map<u32, u128>);

impl GasUsage {
    pub fn default(env: &Env) -> Self {
        GasUsage(Map::new(env))
    }

    pub fn get_by_chain(env: &Env, chain_id: u8) -> Result<u128, Error> {
        GasUsage::get(env).and_then(|gas_usage| {
            gas_usage
                .0
                .get(chain_id as u32)
                .ok_or(Error::GasUsageNotSet)
        })
    }

    pub fn set(env: &Env, chian_id: u8, gas_usage_value: u128) {
        let mut gas_usage = GasUsage::get(env).unwrap_or(GasUsage::default(env));

        gas_usage.0.set(chian_id as u32, gas_usage_value);
        gas_usage.save(env);
    }

    pub fn get_gas_usage_with_default(env: Env, chain_id: u32) -> Result<u128, Error> {
        Ok(GasUsage::get_by_chain(&env, chain_id as u8).unwrap_or(0))
    }
}