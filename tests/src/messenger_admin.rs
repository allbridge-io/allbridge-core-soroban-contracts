use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    Address, BytesN,
};
use crate::contracts::messenger;

use crate::utils::{consts::GOERLI_CHAIN_ID, unwrap_call_result};
use crate::utils::{desoroban_result, BridgeEnv};

#[test]
fn set_other_chain_ids() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    let other_chains_id = BytesN::random(&env);
    messenger.client.set_other_chain_ids(&other_chains_id);

    assert_eq!(other_chains_id, messenger.other_chain_ids());
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_other_chain_ids_no_auth() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    env.mock_auths(&[]);

    let call_result = desoroban_result(
        messenger
            .client
            .try_set_other_chain_ids(&BytesN::random(&env)),
    );
    unwrap_call_result(&env, call_result);
}

#[test]
fn set_gas_oracle() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    let gas_oracle = Address::generate(&env);
    messenger.client.set_gas_oracle(&gas_oracle);

    assert_eq!(gas_oracle, messenger.gas_oracle());
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_gas_oracle_no_auth() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        messenger
            .client
            .try_set_gas_oracle(&Address::generate(&env)),
    );

    unwrap_call_result(&env, call_result);
}

#[test]
fn set_gas_admin() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    let admin = Address::generate(&env);
    messenger.client.set_admin(&admin);

    assert_eq!(admin, messenger.admin());
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_gas_admin_no_auth() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    let call_result = desoroban_result(messenger.client.try_set_admin(&Address::generate(&env)));

    unwrap_call_result(&env, call_result);
}

#[test]
fn set_primary_validator() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    let validator_address = BytesN::random(&env);
    messenger.client.set_primary_validator(&validator_address);

    assert_eq!(validator_address, messenger.primary_validator_key());
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_primary_validator_no_auth() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        messenger
            .client
            .try_set_primary_validator(&BytesN::random(&env)),
    );

    unwrap_call_result(&env, call_result);
}

#[test]
fn add_secondary_validator() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    let validator_address = BytesN::random(&env);
    messenger.client.add_secondary_validator(&validator_address);

    assert!(messenger.has_secondary_validator_key(&validator_address));
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn add_secondary_validator_no_auth() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        messenger
            .client
            .try_add_secondary_validator(&BytesN::random(&env)),
    );

    unwrap_call_result(&env, call_result);
}

#[test]
fn remove_secondary_validator() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    let validator_address = BytesN::random(&env);
    messenger.client.add_secondary_validator(&validator_address);
    assert!(messenger.has_secondary_validator_key(&validator_address));

    messenger
        .client
        .remove_secondary_validator(&validator_address);
    assert!(!messenger.has_secondary_validator_key(&validator_address));
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn remove_secondary_validator_no_auth() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    let validator_address = BytesN::random(&env);
    messenger.client.add_secondary_validator(&validator_address);
    assert!(messenger.has_secondary_validator_key(&validator_address));

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        messenger
            .client
            .try_remove_secondary_validator(&validator_address),
    );

    unwrap_call_result(&env, call_result);
}

#[test]
fn set_gas_usage() {
    let BridgeEnv { messenger, .. } = BridgeEnv::default();

    messenger.client.set_gas_usage(&GOERLI_CHAIN_ID, &100_000);

    assert_eq!(100_000, messenger.client.get_gas_usage(&GOERLI_CHAIN_ID));
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_gas_usage_no_auth() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        messenger
            .client
            .try_set_gas_usage(&GOERLI_CHAIN_ID, &100_000),
    );
    unwrap_call_result(&env, call_result);
}

#[test]
fn upgrade() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();
    let hash =  env.deployer().upload_contract_wasm(messenger::WASM);
    messenger.upgrade(&hash)
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn upgrade_no_auth() {
    let BridgeEnv { env, messenger, .. } = BridgeEnv::default();
    env.mock_auths(&[]);
    let hash =  env.deployer().upload_contract_wasm(messenger::WASM);
    messenger.upgrade(&hash)
}
