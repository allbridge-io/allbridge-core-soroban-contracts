use bridge_storage::*;
use shared::consts::{CHAIN_ID, CHAIN_PRECISION, ORACLE_PRECISION, ORACLE_SCALING_FACTOR};
use shared::soroban_data::SimpleSorobanData;
use shared::utils::bump_instance;
use shared::{error::Error, require};
use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::storage::chain_data::ChainData;

const FROM_ORACLE_TO_CHAIN_SCALING_FACTOR: u128 = 10u128.pow(ORACLE_PRECISION - CHAIN_PRECISION);

#[contract]
pub struct GasOracleContract;

#[contractimpl]
impl GasOracleContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        require!(!Admin::has(&env), Error::Initialized);

        Admin(admin).save(&env);

        Ok(())
    }

    pub fn set_price(
        env: Env,
        chain_id: u32,
        price: Option<u128>,
        gas_price: Option<u128>,
    ) -> Result<(), Error> {
        bump_instance(&env);

        Admin::require_exist_auth(&env)?;
        ChainData::update_gas_price(&env, chain_id, price, gas_price);

        Ok(())
    }

    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        bump_instance(&env);

        Admin::require_exist_auth(&env)?;
        Admin(new_admin).save(&env);

        bump_instance(&env);

        Ok(())
    }

    // view

    pub fn get_gas_price(env: Env, chain_id: u32) -> Result<ChainData, Error> {
        bump_instance(&env);

        ChainData::get(&env, chain_id)
    }

    pub fn get_price(env: Env, chain_id: u32) -> Result<u128, Error> {
        bump_instance(&env);

        ChainData::get(&env, chain_id).map(|chain_data| chain_data.price)
    }

    pub fn get_gas_cost_in_native_token(
        env: Env,
        other_chain_id: u32,
        gas_amount: u128,
    ) -> Result<u128, Error> {
        bump_instance(&env);

        let this_gas_price = ChainData::get(&env, CHAIN_ID)?;
        let other_gas_price = ChainData::get(&env, other_chain_id)?;

        Ok(
            (other_gas_price.gas_price * gas_amount * other_gas_price.price)
                / this_gas_price.price
                / FROM_ORACLE_TO_CHAIN_SCALING_FACTOR,
        )
    }

    pub fn get_transaction_gas_cost_in_usd(
        env: Env,
        other_chain_id: u32,
        gas_amount: u128,
    ) -> Result<u128, Error> {
        bump_instance(&env);

        let other_gas_price = ChainData::get(&env, other_chain_id)?;

        Ok(
            (other_gas_price.gas_price * gas_amount * other_gas_price.price)
                / ORACLE_SCALING_FACTOR,
        )
    }

    pub fn crossrate(env: Env, other_chain_id: u32) -> Result<u128, Error> {
        bump_instance(&env);

        let this_gas_price = ChainData::get(&env, CHAIN_ID)?;
        let other_gas_price = ChainData::get(&env, other_chain_id)?;

        Ok(other_gas_price.price * ORACLE_SCALING_FACTOR / this_gas_price.price)
    }

    pub fn get_admin(env: Env) -> Result<Address, Error> {
        bump_instance(&env);

        Ok(Admin::get(&env)?.as_address())
    }
}
