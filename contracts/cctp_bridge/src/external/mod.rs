use soroban_sdk::{contractclient, Address, Bytes, BytesN, Env};

#[contractclient(name = "GasOracleClient")]
#[allow(dead_code)]
pub trait GasOracle {
    fn get_price(env: Env, chain_id: u32) -> Result<u128, shared::Error>;
    fn get_gas_cost_in_native_token(
        env: Env,
        other_chain_id: u32,
        gas_amount: u128,
    ) -> Result<u128, shared::Error>;
}

#[contractclient(name = "TokenMessengerMinterClient")]
#[allow(dead_code)]
pub trait TokenMessengerMinter {
    fn get_local_token(env: Env, remote_domain: u32, remote_token: BytesN<32>) -> Option<Address>;

    #[allow(clippy::too_many_arguments)]
    fn deposit_for_burn(
        env: Env,
        caller: Address,
        amount: i128,
        destination_domain: u32,
        mint_recipient: BytesN<32>,
        burn_token: Address,
        destination_caller: BytesN<32>,
        max_fee: i128,
        min_finality_threshold: u32,
    );

    #[allow(clippy::too_many_arguments)]
    fn deposit_for_burn_with_hook(
        env: Env,
        caller: Address,
        amount: i128,
        destination_domain: u32,
        mint_recipient: BytesN<32>,
        burn_token: Address,
        destination_caller: BytesN<32>,
        max_fee: i128,
        min_finality_threshold: u32,
        hook_data: Bytes,
    );
}

#[contractclient(name = "MessageTransmitterClient")]
#[allow(dead_code)]
pub trait MessageTransmitter {
    fn receive_message(env: Env, caller: Address, message: Bytes, attestation: Bytes) -> bool;
}
