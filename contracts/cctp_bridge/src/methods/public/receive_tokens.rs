use shared::{require, Error, Event};
use soroban_sdk::{token, Address, Bytes, BytesN, Env};

use crate::{
    cctp_wire::parse_cctp_v2, events::TokensReceived, external::MessageTransmitterClient,
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
    require!(
        parsed.destination_caller == self_bytes32,
        Error::WrongDestinationChain
    );
    require!(
        parsed.mint_recipient == self_bytes32,
        Error::WrongDestinationChain
    );

    let recipient = internal::parse_common_hook_data(&env, &parsed.hook_data)?;
    let recipient_address = recipient.address();

    let accepted = MessageTransmitterClient::new(&env, &config.message_transmitter)
        .receive_message(&self_address, &message, &attestation);
    require!(accepted, Error::NoMessage);

    let wire_net = parsed
        .amount
        .checked_sub(parsed.fee_executed)
        .ok_or(Error::U256Overflow)?;
    require!(wire_net > 0, Error::InvalidArg);
    let received_amount = wire_net.checked_mul(10).ok_or(Error::U256Overflow)?;
    token::TokenClient::new(&env, &config.usdc_token).transfer(
        &self_address,
        &recipient,
        &(received_amount as i128),
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
        amount: received_amount,
        recipient: recipient_address,
        recipient_muxed_id: recipient.id(),
        source_chain_id,
        extra_gas_value: extra_gas_amount,
    }
    .publish(&env);
    Ok(())
}
