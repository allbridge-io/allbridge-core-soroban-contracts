use bridge_storage::Admin;
use shared::{require, soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{BytesN, Env};

use crate::{
    events::AutoDepositWalletDeployedEvent, methods::internal::get_deposit_wallet_salt,
    storage::config::Config,
};

pub fn deploy_deposit_wallet(
    env: Env,
    recipient_chain_id: u32,
    recipient: BytesN<32>,
    recipient_token: BytesN<32>,
    min_deposit_amount: u128,
) -> Result<(), Error> {
    require!(recipient.is_empty(), Error::EmptyRecipient);
    require!(recipient_token.is_empty(), Error::InvalidArg);
    let config = Config::get(&env)?;
    let admin = Admin::get(&env)?;

    let salt = get_deposit_wallet_salt(
        &env,
        recipient_chain_id,
        &recipient,
        &recipient_token,
        min_deposit_amount,
        &config,
    )?;

    let deployed = env
        .deployer()
        .with_address(env.current_contract_address(), salt)
        .deploy_v2(
            config.wallet_wasm_hash,
            (
                admin.0,
                config.bridge,
                env.current_contract_address(),
                recipient_chain_id,
                recipient,
                recipient_token,
                min_deposit_amount,
            ),
        );

    AutoDepositWalletDeployedEvent { address: deployed }.publish(&env);

    Ok(())
}
