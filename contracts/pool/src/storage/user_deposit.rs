use crate::storage::data_key::DataKey;
use proc_macros::{bump_info_instance, data_storage_type, SorobanData};
use shared::soroban_data::SorobanData;
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Default, SorobanData)]
#[data_storage_type(Instance)]
#[bump_info_instance]
pub struct UserDeposit {
    pub lp_amount: u128,
    pub reward_debt: u128,
}

impl UserDeposit {
    pub fn get(env: &Env, address: Address) -> UserDeposit {
        UserDeposit::get_by_key(env, &DataKey::UserDeposit(address)).unwrap_or_default()
    }

    pub fn save(&self, env: &Env, address: Address) {
        self.save_by_key(env, &DataKey::UserDeposit(address));
    }
}
