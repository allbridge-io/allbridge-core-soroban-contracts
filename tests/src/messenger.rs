use shared::{
    consts::CHAIN_ID,
    utils::{hash_message, hash_with_sender, hash_with_sender_address},
};
use soroban_sdk::{
    map,
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env, U256,
};

use crate::utils::{
    consts::GOERLI_CHAIN_ID, contract_id, gen_nonce, message_hash_vec_to_byte, sign_message,
    vec_to_bytes, BridgeEnv, Messenger, MessengerConfig,
};

#[test]
fn messenger_init() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let native_token = env
        .register_stellar_asset_contract_v2(admin.clone())
        .address();
    let gas_oracle = Address::generate(&env);

    let init_config = MessengerConfig {
        admin,
        native_token,
        gas_oracle,
        secondary_validator_keys: map![&env, (BytesN::random(&env), true)],
        ..MessengerConfig::default_config(&env)
    };

    let messenger = Messenger::create(&env, init_config.clone());

    assert_eq!(messenger.client.get_config(), init_config.into());
}

#[test]
fn messenger_send_message() {
    let bridge_env = BridgeEnv::default();
    let BridgeEnv { env, .. } = bridge_env;

    let mut message = BytesN::random(&env);
    message.set(0, CHAIN_ID as u8);
    message.set(1, GOERLI_CHAIN_ID as u8);
    let hash_with_sender =
        hash_with_sender_address(&env, &message, &bridge_env.alice.as_address()).unwrap();

    bridge_env
        .messenger
        .send_message(&bridge_env.alice, &message);

    let expected_fee = 30_000_000;

    assert!(bridge_env
        .messenger
        .client
        .has_sent_message(&hash_with_sender));
    assert_eq!(
        bridge_env
            .native_token
            .client
            .balance(&bridge_env.messenger.id),
        expected_fee
    );
}

#[test]
#[should_panic = "Contract(InvalidChainId)"]
fn send_message_to_unsupported_chain() {
    let bridge_env = BridgeEnv::default();
    let BridgeEnv { env, .. } = bridge_env;

    let mut message = BytesN::random(&env);
    message.set(0, 8);
    message.set(1, GOERLI_CHAIN_ID as u8);

    bridge_env
        .messenger
        .send_message(&bridge_env.alice, &message);
}

#[test]
#[should_panic = "Contract(InvalidOtherChainId)"]
fn send_message_with_wrong_chain_id() {
    let bridge_env = BridgeEnv::default();
    let BridgeEnv { env, .. } = bridge_env;

    let mut message = BytesN::random(&env);
    message.set(0, CHAIN_ID as u8);
    message.set(1, 8);

    bridge_env
        .messenger
        .send_message(&bridge_env.alice, &message);
}

#[test]
fn receive_message() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);

    let primary_validator_slice = hex::decode("04ba5734d8f7091719471e7f7ed6b9df170dc70cc661ca05e688601ad984f068b0d67351e5f06073092499336ab0839ef8a521afd334e53807205fa2f08eec74f4").unwrap();
    let secondary_validator_slice = hex::decode("049d9031e97dd78ff8c15aa86939de9b1e791066a0224e331bc962a2099a7b1f0464b8bbafe1535f2301c72c2cb3535b172da30b02686ab0393d348614f157fbdb").unwrap();
    let primary_signature_slice = hex::decode("e8d012d6892859ec0fb6a44d4693dd64d84854f804bfe89aad293e5f05754f9b3dbcb2ff6580db858a99b33daa0064a8851cda2ad532c27bc6fb2f0e55aaa200").unwrap();
    let secondary_signature_slice =    hex::decode("fb8fbfa594f889da925f57ef766871776568482d87f7364246d78459641d63ab655902e80af59d2ad772a3b277d51aaec3df354f6048b7edc9967109e808d616").unwrap();
    let message_slice =
        hex::decode("000354657374206d657373616765000000000000000000000000000000000000").unwrap();

    let primary_validator = vec_to_bytes::<65>(&env, primary_validator_slice);
    let secondary_validator = vec_to_bytes::<65>(&env, secondary_validator_slice);
    let message = vec_to_bytes::<32>(&env, message_slice);
    let primary_signature = vec_to_bytes::<64>(&env, primary_signature_slice);
    let secondary_signature = vec_to_bytes::<64>(&env, secondary_signature_slice);

    let messenger = Messenger::create(
        &env,
        MessengerConfig {
            admin,
            primary_validator_key: primary_validator,
            secondary_validator_keys: map![&env, (secondary_validator, true)],
            ..MessengerConfig::default_config(&env)
        },
    );

    messenger
        .client
        .receive_message(&message, &primary_signature, &1, &secondary_signature, &1);

    assert!(messenger.client.has_received_message(&message));
}

#[test]
pub fn receive_message_full() {
    let bridge_env = BridgeEnv::default();
    let BridgeEnv { env, .. } = bridge_env;

    let user = Address::generate(&env);
    let yaro_token = Address::generate(&env);
    let goerli_bridge = Address::generate(&env);

    let message_hash = hash_message(
        &env,
        100_000_000,
        &contract_id(&user),
        GOERLI_CHAIN_ID,
        CHAIN_ID,
        &contract_id(&yaro_token),
        &U256::from_u32(&env, 8247),
    );
    let message_hash_with_sender =
        hash_with_sender(&env, &message_hash, &contract_id(&goerli_bridge));
    let message_hash = message_hash_with_sender.to_array().to_vec();

    let primary_signature = sign_message(&env, &message_hash, &bridge_env.primary_validator_wallet);
    let secondary_signature =
        sign_message(&env, &message_hash, &bridge_env.secondary_validator_wallet);

    bridge_env.messenger.receive_message(
        &message_hash_vec_to_byte(&env, &message_hash),
        &primary_signature,
        &secondary_signature,
    );
}

#[test]
#[should_panic = "Contract(HasMessage)"]
fn send_message_twice() {
    let bridge_env = BridgeEnv::default();
    let BridgeEnv { env, .. } = bridge_env;
    let BridgeEnv {
        ref alice,
        ref yaro_token,
        ref messenger,
        ..
    } = bridge_env;

    let message = bridge_env.messenger.hash_and_send_message(
        alice,
        100_000,
        &alice.as_address(),
        yaro_token,
        &gen_nonce(&env),
    );

    messenger.send_message(alice, &message);
}

#[test]
#[should_panic = "Contract(InvalidPrimarySignature)"]
fn confirm_message_with_broken_validator() {
    let mut bridge_env = BridgeEnv::default();

    bridge_env.override_primary_validator(
        "a07d0e5f33e159bd7b471d0e79e4211205f4e89949247ec01ba7559b71acee77",
    );

    bridge_env.hash_and_receive_message(
        100_000,
        &bridge_env.alice.as_address(),
        &bridge_env.yaro_token,
        &gen_nonce(&bridge_env.env),
    );
}

#[test]
#[should_panic = "Contract(InvalidSecondarySignature)"]
fn confirm_message_with_broken_secondary_validator() {
    let mut bridge_env = BridgeEnv::default();

    bridge_env.override_secondary_validator(
        "a07d0e5f33e159bd7b471d0e79e4211205f4e89949247ec01ba7559b71acee77",
    );

    bridge_env.hash_and_receive_message(
        100_000,
        &bridge_env.alice.as_address(),
        &bridge_env.yaro_token,
        &gen_nonce(&bridge_env.env),
    );
}

#[test]
fn withdraw_gas_tokens() {
    let bridge_env = BridgeEnv::default();
    let BridgeEnv {
        env,
        ref alice,
        ref yaro_token,
        ref messenger,
        ref admin,
        ref native_token,
        ..
    } = bridge_env;

    bridge_env.messenger.hash_and_send_message(
        alice,
        100_000,
        &alice.as_address(),
        yaro_token,
        &gen_nonce(&env),
    );

    let messenger_balance = native_token.balance_of(&messenger.id);

    messenger
        .client
        .withdraw_gas_tokens(admin, &messenger_balance);

    let messenger_balance = native_token.balance_of(&messenger.id);

    assert_eq!(messenger_balance, 0);
}
