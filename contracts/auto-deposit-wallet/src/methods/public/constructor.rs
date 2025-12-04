use bridge_storage::Admin;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, BytesN, Env};

use crate::storage::config::Config;

pub fn constructor(
    env: Env,
    admin: Address,
    bridge: Address,
    factory: Address,
    recipient_chain_id: u32,
    recipient: BytesN<32>,
    recipient_token: BytesN<32>,
    min_deposit_amount: u128,
) -> Result<(), Error> {
    assert!(min_deposit_amount > 0);

    let config = Config::new(
        &env,
        factory,
        bridge,
        recipient_chain_id,
        recipient,
        recipient_token,
        min_deposit_amount,
    );

    Admin(admin).save(&env);

    config.save(&env);

    Ok(())
}
