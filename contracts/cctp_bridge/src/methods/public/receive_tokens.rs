use shared::{require, Error, Event};
use soroban_sdk::{token, Address, Bytes, BytesN, Env};

use crate::{
    cctp_wire::parse_cctp_v2, events::TokensReceived, external::TokenMessengerMinterClient,
    methods::internal, storage, utils::address_to_bytes32,
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
    let parsed = parse_cctp_v2(&env, &message)?;

    let source_chain_id = storage::get_chain_id_by_domain(&env, parsed.source_domain)?;
    let self_address = env.current_contract_address();
    let self_bytes32 = address_to_bytes32(&env, &self_address)?;
    let token_messenger_minter_bytes32 = address_to_bytes32(&env, &config.token_messenger_minter)?;
    require!(
        parsed.recipient == token_messenger_minter_bytes32,
        Error::WrongDestinationChain
    );
    require!(
        parsed.destination_caller == self_bytes32,
        Error::WrongDestinationChain
    );
    require!(
        parsed.mint_recipient == self_bytes32,
        Error::WrongDestinationChain
    );
    let local_token = TokenMessengerMinterClient::new(&env, &config.token_messenger_minter)
        .get_local_token(&parsed.source_domain, &parsed.burn_token)
        .ok_or(Error::InvalidArg)?;

    let recipient = internal::parse_common_hook_data(&env, &parsed.hook_data)?;
    let recipient_address = recipient.address();
    require!(
        recipient_address != local_token && recipient_address != self_address,
        Error::InvalidArg
    );

    let minted_amount = internal::mint_through_cctp(
        &env,
        &self_address,
        &local_token,
        &config.message_transmitter,
        &message,
        &attestation,
    )?;

    token::TokenClient::new(&env, &local_token).transfer(&self_address, &recipient, &minted_amount);

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
        recipient_muxed_id: recipient.id(),
        source_chain_id,
        extra_gas_value: extra_gas_amount,
    }
    .publish(&env);
    Ok(())
}
