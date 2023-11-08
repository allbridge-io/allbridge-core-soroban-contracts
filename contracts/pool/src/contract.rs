use bridge_storage::*;
use shared::error::Error;
use shared::soroban_data::SimpleSorobanData;
use shared::utils::bump_instance;
use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::internal::methods::initialize::initialize;
use crate::internal::view::{get_pool, pending_reward};
use crate::internal::{
    admin::{
        adjust_total_lp_amount::*, claim_fee::*, config_addresses::*, config_pool::*, start_stop::*,
    },
    methods::{claim_rewards::*, deposit::*, swap_from_v_usd::*, swap_to_v_usd::*, withdraw::*},
};
use crate::storage::bridge_address::Bridge;
use crate::storage::pool::Pool;
use crate::storage::user_deposit::UserDeposit;

#[contract]
pub struct PoolContract;

#[contractimpl]
impl PoolContract {
    #[allow(clippy::too_many_arguments)]
    pub fn initialize(
        env: Env,
        admin: Address,
        bridge: Address,
        a: u128,
        token: Address,
        fee_share_bp: u128,
        balance_ratio_min_bp: u128,
        admin_fee_share_bp: u128,
    ) -> Result<(), Error> {
        initialize(
            env,
            admin,
            bridge,
            a,
            token,
            fee_share_bp,
            balance_ratio_min_bp,
            admin_fee_share_bp,
        )
    }

    pub fn deposit(env: Env, sender: Address, amount: u128) -> Result<(), Error> {
        bump_instance(&env);

        deposit(env, sender, amount)
    }

    pub fn withdraw(env: Env, sender: Address, amount_lp: u128) -> Result<(), Error> {
        bump_instance(&env);

        withdraw(env, sender, amount_lp)
    }

    pub fn swap_to_v_usd(
        env: Env,
        user: Address,
        amount: u128,
        zero_fee: bool,
    ) -> Result<u128, Error> {
        bump_instance(&env);

        swap_to_v_usd(env, user, amount, zero_fee)
    }

    pub fn swap_from_v_usd(
        env: Env,
        user: Address,
        vusd_amount: u128,
        receive_amount_min: u128,
        zero_fee: bool,
    ) -> Result<u128, Error> {
        bump_instance(&env);

        swap_from_v_usd(env, user, vusd_amount, receive_amount_min, zero_fee)
    }

    pub fn claim_rewards(env: Env, sender: Address) -> Result<(), Error> {
        bump_instance(&env);

        claim_rewards(env, sender)
    }

    /// `admin`
    pub fn set_fee_share(env: Env, fee_share_bp: u128) -> Result<(), Error> {
        bump_instance(&env);

        set_fee_share(env, fee_share_bp)
    }

    pub fn adjust_total_lp_amount(env: Env) -> Result<(), Error> {
        bump_instance(&env);

        adjust_total_lp_amount(env)
    }

    pub fn set_balance_ratio_min_bp(env: Env, balance_ratio_min_bp: u128) -> Result<(), Error> {
        bump_instance(&env);

        set_balance_ratio_min_bp(env, balance_ratio_min_bp)
    }

    pub fn stop_deposit(env: Env) -> Result<(), Error> {
        bump_instance(&env);

        stop_deposit(env)
    }

    pub fn start_deposit(env: Env) -> Result<(), Error> {
        bump_instance(&env);

        start_deposit(env)
    }

    pub fn stop_withdraw(env: Env) -> Result<(), Error> {
        bump_instance(&env);

        stop_withdraw(env)
    }

    pub fn start_withdraw(env: Env) -> Result<(), Error> {
        bump_instance(&env);

        start_withdraw(env)
    }

    pub fn set_stop_authority(env: Env, stop_authority: Address) -> Result<(), Error> {
        bump_instance(&env);

        set_stop_authority(env, stop_authority)
    }

    pub fn set_bridge(env: Env, bridge: Address) -> Result<(), Error> {
        bump_instance(&env);

        set_bridge(env, bridge)
    }

    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        bump_instance(&env);

        set_admin(env, new_admin)
    }

    pub fn set_admin_fee_share(env: Env, admin_fee_share_bp: u128) -> Result<(), Error> {
        bump_instance(&env);

        set_admin_fee_share(env, admin_fee_share_bp)
    }

    pub fn claim_admin_fee(env: Env) -> Result<(), Error> {
        bump_instance(&env);

        claim_admin_fee(env)
    }

    /// `view`
    pub fn pending_reward(env: Env, user: Address) -> Result<u128, Error> {
        bump_instance(&env);

        pending_reward(env, user)
    }

    pub fn get_pool(env: Env) -> Result<Pool, Error> {
        bump_instance(&env);

        get_pool(env)
    }

    pub fn get_admin(env: Env) -> Result<Address, Error> {
        bump_instance(&env);

        Ok(Admin::get(&env)?.as_address())
    }

    pub fn get_stop_authority(env: Env) -> Result<Address, Error> {
        bump_instance(&env);

        Ok(StopAuthority::get(&env)?.as_address())
    }

    pub fn get_bridge(env: Env) -> Result<Address, Error> {
        bump_instance(&env);

        Ok(Bridge::get(&env)?.as_address())
    }

    pub fn get_user_deposit(env: Env, user: Address) -> Result<UserDeposit, Error> {
        bump_instance(&env);

        Ok(UserDeposit::get(&env, user))
    }
}
