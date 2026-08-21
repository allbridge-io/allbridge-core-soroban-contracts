use shared::{require, Error};
use soroban_sdk::{Bytes, BytesN, Env, MuxedAddress};

use crate::{
    cctp_wire::parse_cctp_v2, methods::MAX_FEE_SHARE_DENOMINATOR, storage::Config,
    utils::address_to_bytes32,
};

const ADMIN_FEE_DENOMINATOR: u128 = 10_000;

pub(crate) struct ValidatedMessageData {
    pub source_domain: u32,
    pub burn_token: BytesN<32>,
    pub forward_recipient: MuxedAddress,
}

pub(crate) fn validate_cctp_message(
    env: &Env,
    config: &Config,
    message: &Bytes,
) -> Result<ValidatedMessageData, Error> {
    let parsed = parse_cctp_v2(env, message)?;
    let contract_address = env.current_contract_address();
    let contract_bytes32 = address_to_bytes32(env, &contract_address)?;
    let token_messenger_minter_bytes32 = address_to_bytes32(env, &config.token_messenger_minter)?;

    require!(
        parsed.recipient == token_messenger_minter_bytes32,
        Error::WrongDestinationChain
    );
    require!(
        parsed.destination_caller == contract_bytes32,
        Error::WrongDestinationChain
    );
    require!(
        parsed.mint_recipient == contract_bytes32,
        Error::WrongDestinationChain
    );

    let forward_recipient = super::parse_common_hook_data(env, &parsed.hook_data)?;

    Ok(ValidatedMessageData {
        source_domain: parsed.source_domain,
        burn_token: parsed.burn_token,
        forward_recipient,
    })
}

pub(crate) fn validate_max_fee_share(value: i128) -> Result<(), Error> {
    require!(
        (0..=MAX_FEE_SHARE_DENOMINATOR).contains(&value),
        Error::InvalidArg
    );
    Ok(())
}

pub(crate) fn validate_admin_fee_share(value: u64) -> Result<(), Error> {
    require!(value <= ADMIN_FEE_DENOMINATOR as u64, Error::InvalidArg);
    Ok(())
}

pub(crate) fn admin_fee_denominator() -> u128 {
    ADMIN_FEE_DENOMINATOR
}
