use proc_macros::Event;
use soroban_sdk::{contracttype, Address, BytesN};

#[derive(Event)]
#[contracttype]
pub struct TokensSent {
    pub amount: u128,
    pub admin_fee: u128,
    pub sender: Address,
    pub recipient: BytesN<32>,
    pub destination_chain_id: u32,
}

#[derive(Event)]
#[contracttype]
pub struct TokensReceived {
    pub message_id: BytesN<32>,
    pub amount: i128,
    pub recipient: Address,
    pub recipient_muxed_id: Option<u64>,
    pub source_chain_id: u32,
    pub extra_gas_value: u128,
}

#[derive(Event)]
#[contracttype]
pub struct ReceiveFee {
    pub bridge_transaction_cost: u128,
    pub message_transaction_cost: u128,
    pub extra_gas: u128,
}

#[derive(Event)]
#[contracttype]
pub struct BridgingFeeFromTokens {
    pub gas: u128,
    pub fee_token_amount: u128,
}
