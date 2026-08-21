use shared::{require, Error};
use soroban_sdk::{token, Address, Bytes, Env};

use crate::external::MessageTransmitterClient;

pub(crate) fn mint_through_cctp(
    env: &Env,
    contract: &Address,
    token: &Address,
    message_transmitter: &Address,
    message: &Bytes,
    attestation: &Bytes,
) -> Result<i128, Error> {
    let token_client = token::TokenClient::new(env, token);
    let starting_balance = token_client.balance(contract);

    let accepted = MessageTransmitterClient::new(env, message_transmitter).receive_message(
        contract,
        message,
        attestation,
    );
    require!(accepted, Error::NoMessage);

    let minted_amount = token_client
        .balance(contract)
        .checked_sub(starting_balance)
        .ok_or(Error::InvalidArg)?;
    require!(minted_amount > 0, Error::NoMessage);
    Ok(minted_amount)
}
