use soroban_sdk::{testutils::BytesN as _, BytesN};

use crate::utils::{
    auto_deposit::DepositAddressCreation, consts::GOERLI_CHAIN_ID, gen_nonce,
    get_latest_event_unchecked, BridgeEnv,
};

#[test]
fn create_deposit_wallet() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref alice,
        ref yusd_token,
        ref env,
        ..
    } = BridgeEnv::default();

    auto_deposit_factory.create_deposit_wallet(
        alice.as_address(),
        alice.as_address(),
        yusd_token.id.clone(),
        10,
        10,
        vec![GOERLI_CHAIN_ID],
    );
    let deposit_address_creation = get_latest_event_unchecked::<DepositAddressCreation>(env);

    assert_eq!(
        deposit_address_creation,
        DepositAddressCreation {
            recipient: alice.as_address(),
            recipient_token: yusd_token.id.clone(),
            min_deposit_amount: 10,
            chain_ids: soroban_sdk::vec![env, 2],
        }
    );
}

#[test]
#[should_panic = "Contract(InvalidChainId)"]
fn create_deposit_wallet_invalid_chain_id() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref alice,
        ref yusd_token,
        ..
    } = BridgeEnv::default();

    auto_deposit_factory.create_deposit_wallet(
        alice.as_address(),
        alice.as_address(),
        yusd_token.id.clone(),
        10,
        10,
        vec![GOERLI_CHAIN_ID, 19],
    );
}

#[test]
#[should_panic = "Contract(ADMinDepositAmountIsZero)"]
fn create_deposit_wallet_min_deposit_is_zero() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref alice,
        ref yusd_token,
        ..
    } = BridgeEnv::default();

    auto_deposit_factory.create_deposit_wallet(
        alice.as_address(),
        alice.as_address(),
        yusd_token.id.clone(),
        0,
        10,
        vec![GOERLI_CHAIN_ID],
    );
}

#[test]
#[should_panic = "Contract(ADNotEnoughFee)"]
fn create_deposit_wallet_not_enough_fee() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref alice,
        ref yusd_token,
        ..
    } = BridgeEnv::default();

    auto_deposit_factory.create_deposit_wallet(
        alice.as_address(),
        alice.as_address(),
        yusd_token.id.clone(),
        10,
        0,
        vec![GOERLI_CHAIN_ID],
    );
}

#[test]
fn deploy_deposit_wallet() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref env,
        ..
    } = BridgeEnv::default();

    let recipient = BytesN::<32>::random(env);
    let recipient_token = BytesN::<32>::random(env);
    let deployed_address = auto_deposit_factory.deploy_deposit_wallet(
        GOERLI_CHAIN_ID,
        recipient.clone(),
        recipient_token.clone(),
        10,
    );
    let expected_address = auto_deposit_factory.client.deposit_wallet_address(
        &GOERLI_CHAIN_ID,
        &recipient,
        &recipient_token,
        &10,
    );
    assert_eq!(deployed_address, expected_address);
}

#[test]
#[should_panic = "Contract(EmptyRecipient)"]
fn deploy_deposit_wallet_empty_recipient() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref env,
        ..
    } = BridgeEnv::default();

    auto_deposit_factory.deploy_deposit_wallet(
        GOERLI_CHAIN_ID,
        BytesN::<32>::from_array(env, &[0; 32]),
        BytesN::<32>::random(env).clone(),
        10,
    );
}

#[test]
#[should_panic = "Contract(InvalidArg)"]
fn deploy_deposit_wallet_empty_recipient_token() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref env,
        ..
    } = BridgeEnv::default();

    auto_deposit_factory.deploy_deposit_wallet(
        GOERLI_CHAIN_ID,
        BytesN::<32>::random(env).clone(),
        BytesN::<32>::from_array(env, &[0; 32]),
        10,
    );
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn deploy_deposit_wallet_min_deposit_is_zero() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref env,
        ..
    } = BridgeEnv::default();

    auto_deposit_factory.deploy_deposit_wallet(
        GOERLI_CHAIN_ID,
        BytesN::<32>::random(env).clone(),
        BytesN::<32>::random(env).clone(),
        0,
    );
}

#[test]
fn swap_and_bridge() {
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
    let nonce = gen_nonce(env);

    auto_deposit_factory.swap_and_bridge(wallet_address, yusd_token.id.clone(), nonce);
}

#[test]
#[should_panic = "Contract(ADAmountTooLow)"]
fn swap_and_bridge_amount_too_low() {
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

    yusd_token.transfer(&alice.as_address(), &wallet_address, 0.05);
    let nonce = gen_nonce(env);

    auto_deposit_factory.swap_and_bridge(wallet_address, yusd_token.id.clone(), nonce);
}
