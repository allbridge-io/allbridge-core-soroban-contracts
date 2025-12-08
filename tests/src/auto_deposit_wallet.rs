use soroban_sdk::{testutils::BytesN as _, Bytes, BytesN, U256};

use crate::utils::{auto_deposit::AutoDepositWallet, consts::GOERLI_CHAIN_ID, BridgeEnv};

// #[test]
// fn swap_and_bridge() {
//     let BridgeEnv {
//         ref auto_deposit_factory,
//         ref env,
//         ref yusd_token,
//         ref alice,
//         goerli_token,
//         ..
//     } = BridgeEnv::default();
//
//     let wallet_address = auto_deposit_factory.deploy_deposit_wallet(
//         GOERLI_CHAIN_ID,
//         BytesN::<32>::random(env).clone(),
//         goerli_token,
//         10,
//     );
//     yusd_token.transfer(&alice.as_address(), &wallet_address, 25.0);
//     let wallet = AutoDepositWallet::new(env, wallet_address);
//
//     let nonce = U256::from_be_bytes(
//         env,
//         &Bytes::from_slice(env, &BytesN::<32>::random(env).to_array()),
//     );
//
//     wallet.swap_and_bridge(yusd_token.id.clone(), nonce);
// }

#[test]
#[should_panic = "Context(InvalidAction)"]
fn factory_swap_and_bridge_no_auth() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref env,
        ref yusd_token,
        ref alice,
        goerli_token,
        ..
    } = BridgeEnv::default();

    let wallet_address = auto_deposit_factory.deploy_deposit_wallet(
        GOERLI_CHAIN_ID,
        BytesN::<32>::random(env).clone(),
        goerli_token,
        10,
    );
    yusd_token.transfer(&alice.as_address(), &wallet_address, 10.0);
    let amount = yusd_token.balance_of(&wallet_address);
    let wallet = AutoDepositWallet::new(env, wallet_address);

    let nonce = U256::from_be_bytes(
        env,
        &Bytes::from_slice(env, &BytesN::<32>::random(env).to_array()),
    );

    env.mock_auths(&[]);
    wallet.factory_swap_and_bridge(yusd_token.id.clone(), amount, nonce);
}
