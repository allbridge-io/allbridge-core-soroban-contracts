use bridge_storage::Admin;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

use crate::storage;

pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let mut config = storage::get_config(&env)?;
    config.admin = new_admin.clone();
    storage::set_config(&env, &config);
    Admin(new_admin).save(&env);
    Ok(())
}
