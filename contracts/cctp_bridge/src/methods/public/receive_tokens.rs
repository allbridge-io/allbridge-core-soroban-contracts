use shared::{Error, Event};
use soroban_sdk::{token, Address, Bytes, BytesN, Env};

use crate::{
    events::TokensReceived,
    external::TokenMessengerMinterClient,
    methods::internal::{self, ValidatedMessageData},
    storage,
};

pub fn receive_tokens(
    env: Env,
    sender: Address,
    message_id: BytesN<32>,
    message: Bytes,
    attestation: Bytes,
    extra_gas_amount: u128,
) -> Result<(), Error> {
    if extra_gas_amount > 0 {
        sender.require_auth();
    }

    let config = storage::get_config(&env)?;
    let ValidatedMessageData {
        source_domain,
        burn_token,
        forward_recipient,
    } = internal::validate_cctp_message(&env, &config, &message)?;
    let source_chain_id = storage::get_chain_id_by_domain(&env, source_domain)?;
    let local_token = TokenMessengerMinterClient::new(&env, &config.token_messenger_minter)
        .get_local_token(&source_domain, &burn_token)
        .ok_or(Error::InvalidArg)?;
    let self_address = env.current_contract_address();
    let recipient_address = forward_recipient.address();

    let minted_amount = internal::mint_through_cctp(
        &env,
        &self_address,
        &local_token,
        &config.message_transmitter,
        &message,
        &attestation,
    )?;

    token::TokenClient::new(&env, &local_token).transfer(
        &self_address,
        &forward_recipient,
        &minted_amount,
    );

    if extra_gas_amount > 0 {
        token::Client::new(&env, &config.native_token).transfer(
            &sender,
            &recipient_address,
            &(extra_gas_amount as i128),
        );
    }

    TokensReceived {
        message_id,
        amount: minted_amount,
        recipient: recipient_address,
        recipient_muxed_id: forward_recipient.id(),
        source_chain_id,
        extra_gas_value: extra_gas_amount,
    }
    .publish(&env);
    Ok(())
}
