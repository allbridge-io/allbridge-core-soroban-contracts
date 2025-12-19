use crate::contracts::auto_deposit_factory;
use shared::consts::{CHAIN_PRECISION, ORACLE_PRECISION};
use soroban_sdk::{testutils::Address as _, Address};

use crate::utils::{consts::GOERLI_CHAIN_ID, unwrap_call_result};
use crate::utils::{desoroban_result, BridgeEnv, Token};

#[test]
fn set_gas_oracle() {
    let BridgeEnv {
        env,
        auto_deposit_factory,
        ..
    } = BridgeEnv::default();

    let gas_oracle = Address::generate(&env);
    auto_deposit_factory.client.set_gas_oracle(&gas_oracle);

    assert_eq!(gas_oracle, auto_deposit_factory.gas_oracle());
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_gas_oracle_no_auth() {
    let BridgeEnv {
        env,
        auto_deposit_factory,
        ..
    } = BridgeEnv::default();

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        auto_deposit_factory
            .client
            .try_set_gas_oracle(&Address::generate(&env)),
    );

    unwrap_call_result(&env, call_result);
}

#[test]
fn register_token() {
    let BridgeEnv {
        auto_deposit_factory,
        ref env,
        ref admin,
        ..
    } = BridgeEnv::default();
    let token = Token::create(env, "temp", admin);
    let decimals = token.client.decimals();
    assert_eq!(decimals, 7);

    auto_deposit_factory.client.register_token(&token.id);

    let config = auto_deposit_factory.config();

    let fee_conversion_factor = config
        .fee_conversion_factor
        .get(token.id.clone())
        .unwrap_or(0);
    let expected_fee_conversion_factor = 10u128.pow(ORACLE_PRECISION - decimals + CHAIN_PRECISION);

    assert_eq!(fee_conversion_factor, expected_fee_conversion_factor);
    assert!(config.accepted_tokens.contains_key(token.id.clone()));
}

#[test]
fn unregister_token() {
    let BridgeEnv {
        auto_deposit_factory,
        yusd_token,
        ..
    } = BridgeEnv::default();

    auto_deposit_factory.client.unregister_token(&yusd_token.id);

    let config = auto_deposit_factory.config();
    assert!(!config.accepted_tokens.contains_key(yusd_token.id.clone()));
    assert!(!config
        .fee_conversion_factor
        .contains_key(yusd_token.id.clone()));
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn unregister_token_no_auth() {
    let BridgeEnv {
        auto_deposit_factory,
        yusd_token,
        ref env,
        ..
    } = BridgeEnv::default();

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        auto_deposit_factory
            .client
            .try_unregister_token(&yusd_token.id),
    );
    unwrap_call_result(env, call_result);
}

#[test]
fn set_gas_admin() {
    let BridgeEnv {
        env,
        auto_deposit_factory,
        ..
    } = BridgeEnv::default();

    let admin = Address::generate(&env);
    auto_deposit_factory.client.set_admin(&admin);

    assert_eq!(admin, auto_deposit_factory.admin());
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_gas_admin_no_auth() {
    let BridgeEnv {
        env,
        auto_deposit_factory,
        ..
    } = BridgeEnv::default();

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        auto_deposit_factory
            .client
            .try_set_admin(&Address::generate(&env)),
    );

    unwrap_call_result(&env, call_result);
}

#[test]
fn set_send_tx_cost() {
    let BridgeEnv {
        auto_deposit_factory,
        ..
    } = BridgeEnv::default();

    auto_deposit_factory.client.set_send_tx_cost(&100_000);

    assert_eq!(100_000, auto_deposit_factory.config().send_tx_cost);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_send_tx_cost_no_auth() {
    let BridgeEnv {
        auto_deposit_factory,
        ref env,
        ..
    } = BridgeEnv::default();

    env.mock_auths(&[]);
    let call_result = desoroban_result(auto_deposit_factory.client.try_set_send_tx_cost(&100_000));
    unwrap_call_result(env, call_result);
}

#[test]
fn set_gas_usage() {
    let BridgeEnv {
        auto_deposit_factory,
        ..
    } = BridgeEnv::default();

    auto_deposit_factory
        .client
        .set_gas_usage(&GOERLI_CHAIN_ID, &100_000);

    assert_eq!(
        100_000,
        auto_deposit_factory.client.get_gas_usage(&GOERLI_CHAIN_ID)
    );
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_gas_usage_no_auth() {
    let BridgeEnv {
        env,
        auto_deposit_factory,
        ..
    } = BridgeEnv::default();

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        auto_deposit_factory
            .client
            .try_set_gas_usage(&GOERLI_CHAIN_ID, &100_000),
    );
    unwrap_call_result(&env, call_result);
}

#[test]
fn withdraw_token() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref yusd_token,
        ref admin,
        ..
    } = BridgeEnv::default();
    yusd_token.airdrop(&auto_deposit_factory.id);

    let balance = yusd_token.balance_of(&auto_deposit_factory.id);

    auto_deposit_factory
        .client
        .withdraw(&admin, &yusd_token.id, &balance);

    let balance = yusd_token.balance_of(&auto_deposit_factory.id);

    assert_eq!(balance, 0);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn withdraw_token_no_auth() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref yusd_token,
        ref admin,
        ref env,
        ..
    } = BridgeEnv::default();
    yusd_token.airdrop(&auto_deposit_factory.id);

    let balance = yusd_token.balance_of(&auto_deposit_factory.id);

    env.mock_auths(&[]);
    let call_result = desoroban_result(auto_deposit_factory.client.try_withdraw(
        &admin,
        &yusd_token.id,
        &balance,
    ));
    unwrap_call_result(env, call_result);
}

#[test]
fn withdraw_gas_tokens() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref native_token,
        ref admin,
        ..
    } = BridgeEnv::default();
    native_token.airdrop(&auto_deposit_factory.id);

    let balance = native_token.balance_of(&auto_deposit_factory.id);

    auto_deposit_factory
        .client
        .withdraw_gas_tokens(&admin, &balance);

    let balance = native_token.balance_of(&auto_deposit_factory.id);

    assert_eq!(balance, 0);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn withdraw_gas_tokens_no_auth() {
    let BridgeEnv {
        ref auto_deposit_factory,
        ref native_token,
        ref env,
        ref admin,
        ..
    } = BridgeEnv::default();
    native_token.airdrop(&auto_deposit_factory.id);

    let balance = native_token.balance_of(&auto_deposit_factory.id);

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        auto_deposit_factory
            .client
            .try_withdraw_gas_tokens(&admin, &balance),
    );
    unwrap_call_result(env, call_result);
}

#[test]
fn upgrade() {
    let BridgeEnv {
        env,
        auto_deposit_factory,
        ..
    } = BridgeEnv::default();
    let hash = env
        .deployer()
        .upload_contract_wasm(auto_deposit_factory::WASM);
    auto_deposit_factory.upgrade(&hash);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn upgrade_no_auth() {
    let BridgeEnv {
        env,
        auto_deposit_factory,
        ..
    } = BridgeEnv::default();
    env.mock_auths(&[]);
    let hash = env
        .deployer()
        .upload_contract_wasm(auto_deposit_factory::WASM);
    auto_deposit_factory.upgrade(&hash);
}
