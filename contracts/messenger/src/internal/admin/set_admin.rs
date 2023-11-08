use bridge_storage::*;
use shared::{error::Error, soroban_data::SimpleSorobanData};
use soroban_sdk::{Address, Env};

pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Admin(new_admin).save(&env);
    Ok(())
}
