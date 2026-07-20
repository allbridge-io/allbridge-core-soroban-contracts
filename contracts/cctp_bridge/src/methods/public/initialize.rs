use bridge_storage::Admin;
use shared::{require, soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

use crate::{
    methods::internal,
    storage::{self, Config},
};

#[allow(clippy::too_many_arguments)]
pub fn initialize(
    env: Env,
    admin: Address,
    usdc_token: Address,
    token_messenger_minter: Address,
    message_transmitter: Address,
    gas_oracle: Address,
    native_token: Address,
    min_finality_threshold: u32,
    max_fee_share: i128,
    admin_fee_share_bp: u64,
    bridging_fee_conversion_factor: u128,
) -> Result<(), Error> {
    require!(!storage::has_config(&env), Error::Initialized);
    internal::validate_max_fee_share(max_fee_share)?;
    internal::validate_admin_fee_share(admin_fee_share_bp)?;
    require!(bridging_fee_conversion_factor > 0, Error::InvalidArg);

    storage::set_config(
        &env,
        &Config {
            admin: admin.clone(),
            usdc_token,
            token_messenger_minter,
            message_transmitter,
            gas_oracle,
            native_token,
            min_finality_threshold,
            max_fee_share,
            admin_fee_share_bp,
            bridging_fee_conversion_factor,
            outbound_nonce: 0,
        },
    );
    Admin(admin).save(&env);
    Ok(())
}
