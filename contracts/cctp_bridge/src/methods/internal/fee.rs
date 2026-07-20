use shared::{consts::CHAIN_ID, Error, Event};
use soroban_sdk::Env;

use crate::{events::BridgingFeeFromTokens, external::GasOracleClient, storage::Config};

pub(crate) fn convert_fee_token_amount(
    env: &Env,
    config: &Config,
    fee_token_amount: u128,
) -> Result<u128, Error> {
    if fee_token_amount == 0 {
        return Ok(0);
    }
    let fee = config.bridging_fee_conversion_factor * fee_token_amount
        / GasOracleClient::new(env, &config.gas_oracle).get_price(&CHAIN_ID);
    BridgingFeeFromTokens {
        gas: fee,
        fee_token_amount,
    }
    .publish(env);
    Ok(fee)
}
