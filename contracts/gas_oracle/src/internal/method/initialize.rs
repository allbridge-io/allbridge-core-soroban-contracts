use bridge_storage::Admin;
use shared::{error::Error, require, soroban_data::SimpleSorobanData};
use soroban_sdk::{Address, Env};

pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
    require!(!Admin::has(&env), Error::Initialized);

    Admin(admin).save(&env);

    Ok(())
}
