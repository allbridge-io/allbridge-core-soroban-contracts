use crate::storage::bridge_address::Bridge;
use bridge_storage::*;
use shared::soroban_data::SimpleSorobanData;

use shared::error::Error;
use soroban_sdk::{Address, Env};

pub fn set_stop_authority(env: Env, stop_authority: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    StopAuthority(stop_authority).save(&env);

    Ok(())
}

pub fn set_bridge(env: Env, bridge: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Bridge(bridge).save(&env);
    Ok(())
}

pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Admin(new_admin).save(&env);

    Ok(())
}
