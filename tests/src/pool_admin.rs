use soroban_sdk::{testutils::Address as _, Address};
use crate::contracts::pool;

use crate::utils::{desoroban_result, unwrap_call_result, BridgeEnv, BridgeEnvConfig};

#[test]
#[should_panic = "Contract(Forbidden)"]
fn stop_deposit() {
    let BridgeEnv {
        env,
        yaro_pool,
        alice,
        ..
    } = BridgeEnv::default();

    yaro_pool.client.stop_deposit();
    assert!(!yaro_pool.can_deposit());
    let call_result = yaro_pool.deposit(&alice, 1_000.0);

    unwrap_call_result(&env, call_result);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn stop_deposit_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    unwrap_call_result(&env, desoroban_result(yaro_pool.client.try_stop_deposit()));
}

#[test]
fn start_deposit() {
    let BridgeEnv {
        yaro_pool, alice, ..
    } = BridgeEnv::default();

    yaro_pool.client.stop_deposit();
    yaro_pool.client.start_deposit();
    assert!(yaro_pool.can_deposit());

    yaro_pool.deposit(&alice, 1_000.0).unwrap();
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn start_deposit_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    yaro_pool.client.stop_deposit();

    env.mock_auths(&[]);
    unwrap_call_result(&env, desoroban_result(yaro_pool.client.try_start_deposit()));
}

#[test]
#[should_panic = "Contract(Forbidden)"]
fn stop_withdraw() {
    let BridgeEnv {
        env,
        yaro_pool,
        alice,
        ..
    } = BridgeEnv::default();

    yaro_pool.deposit(&alice, 1_000.0).unwrap();

    yaro_pool.client.stop_withdraw();
    assert!(!yaro_pool.can_withdraw());

    let call_result = yaro_pool.withdraw(&alice, 1_000.0);

    unwrap_call_result(&env, call_result);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn stop_withdraw_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    unwrap_call_result(&env, desoroban_result(yaro_pool.client.try_stop_withdraw()));
}

#[test]
fn start_withdraw() {
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default();

    yaro_pool.client.stop_withdraw();
    yaro_pool.client.start_withdraw();
    assert!(yaro_pool.can_withdraw());
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn start_withdraw_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    yaro_pool.client.stop_withdraw();

    env.mock_auths(&[]);
    unwrap_call_result(
        &env,
        desoroban_result(yaro_pool.client.try_start_withdraw()),
    );
}

#[test]
fn set_admin() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    let admin = Address::generate(&env);
    yaro_pool.client.set_admin(&admin);

    assert_eq!(yaro_pool.admin(), admin);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_admin_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    unwrap_call_result(
        &env,
        desoroban_result(yaro_pool.client.try_set_admin(&Address::generate(&env))),
    );
}

#[test]
fn set_stop_authority() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    let stop_authority = Address::generate(&env);
    yaro_pool.client.set_stop_authority(&stop_authority);

    assert_eq!(yaro_pool.stop_authority(), stop_authority);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_stop_authority_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    unwrap_call_result(
        &env,
        desoroban_result(
            yaro_pool
                .client
                .try_set_stop_authority(&Address::generate(&env)),
        ),
    );
}

#[test]
fn set_bridge() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    let bridge = Address::generate(&env);
    yaro_pool.client.set_bridge(&bridge);

    assert_eq!(yaro_pool.bridge(), bridge);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_bridge_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    unwrap_call_result(
        &env,
        desoroban_result(yaro_pool.client.try_set_bridge(&Address::generate(&env))),
    );
}

#[test]
fn set_fee_share() {
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default();

    yaro_pool.client.set_fee_share(&1000);
    assert_eq!(yaro_pool.fee_share_bp(), 1000);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_fee_share_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    unwrap_call_result(
        &env,
        desoroban_result(yaro_pool.client.try_set_fee_share(&1_000)),
    );
}

#[test]
#[should_panic = "Contract(InvalidArg)"]
fn set_admin_fee_share_invalid() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    let call_result = desoroban_result(yaro_pool.client.try_set_admin_fee_share(&10_000));

    unwrap_call_result(&env, call_result);
}

#[test]
#[should_panic = "Contract(InvalidArg)"]
fn set_fee_share_invalid() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    let call_result = desoroban_result(yaro_pool.client.try_set_fee_share(&10_000));

    unwrap_call_result(&env, call_result);
}

#[test]
fn set_balance_ratio_min_bp() {
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default();

    yaro_pool.client.set_balance_ratio_min_bp(&100);
    assert_eq!(yaro_pool.balance_ratio_min_bp(), 100);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn balance_ratio_min_bp_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    unwrap_call_result(
        &env,
        desoroban_result(yaro_pool.client.try_set_balance_ratio_min_bp(&100)),
    );
}

#[test]
fn admin_fee_share_bp() {
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default();

    yaro_pool.client.set_admin_fee_share(&100);
    assert_eq!(yaro_pool.admin_fee_share_bp(), 100);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn admin_fee_share_bp_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();

    env.mock_auths(&[]);
    unwrap_call_result(
        &env,
        desoroban_result(yaro_pool.client.try_set_admin_fee_share(&100)),
    );
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn adjust_total_lp_amount_no_auth() {
    let bridge_env = BridgeEnv::create(BridgeEnvConfig {
        yaro_admin_deposit: 1_000_000_000.0,
        yusd_admin_deposit: 1_000_000_000.0,
        ..Default::default()
    });

    bridge_env.env.mock_auths(&[]);

    unwrap_call_result(
        &bridge_env.env,
        desoroban_result(bridge_env.yaro_pool.client.try_adjust_total_lp_amount()),
    );
}

#[test]
fn upgrade() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();
    let hash = env.deployer().upload_contract_wasm(pool::WASM);
    unwrap_call_result(
        &env,
        yaro_pool.upgrade(&hash),
    )
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn upgrade_no_auth() {
    let BridgeEnv { env, yaro_pool, .. } = BridgeEnv::default();
    env.mock_auths(&[]);
    let hash = env.deployer().upload_contract_wasm(pool::WASM);
    unwrap_call_result(
        &env,
        yaro_pool.upgrade(&hash),
    )
}