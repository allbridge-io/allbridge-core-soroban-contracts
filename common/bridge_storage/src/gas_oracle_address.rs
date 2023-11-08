use proc_macros::{
    bump_info_instance, data_storage_type, symbol_key, SorobanData, SorobanSimpleData,
};
use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(SorobanData, SorobanSimpleData)]
#[symbol_key("GasOrclAd")]
#[data_storage_type(Instance)]
#[bump_info_instance]
pub struct GasOracleAddress(pub Address);

impl GasOracleAddress {
    #[inline]
    pub fn as_address(&self) -> Address {
        self.0.clone()
    }
}
