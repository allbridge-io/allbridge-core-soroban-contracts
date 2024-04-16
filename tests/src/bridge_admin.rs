use shared::consts::{CHAIN_PRECISION, ORACLE_PRECISION};
use soroban_sdk::testutils::{Address as _, BytesN as _, MockAuth, MockAuthInvoke};
use soroban_sdk::{Address, BytesN, IntoVal};
use crate::contracts::bridge;

use crate::utils::consts::GOERLI_CHAIN_ID;
use crate::utils::{contract_id, BridgeEnv, Pool};

#[test]
fn add_pool() {
    let BridgeEnv {
        ref env,
        bridge,
        native_token,
        admin,
        ..
    } = BridgeEnv::default();

    let init_bridge_config = bridge.client.get_config();

    let pool = Pool::create(env, &admin, &bridge.id, 20, &native_token.id, 30, 0, 1);

    bridge.client.add_pool(&pool.id, &native_token.id);

    let bridge_config = bridge.client.get_config();

    assert_eq!(
        bridge_config.pools.len(),
        init_bridge_config.pools.len() + 1
    );

    let decimals = native_token.client.decimals();

    let bridging_fee_conversion_factor = 10u128.pow(ORACLE_PRECISION - decimals + CHAIN_PRECISION);
    let from_gas_oracle_factor = 10u128.pow(ORACLE_PRECISION - decimals);

    let pool_id_on_contract = bridge_config
        .pools
        .get(contract_id(&native_token.id))
        .unwrap();

    let from_gas_oracle_factor_on_contract = bridge_config
        .from_gas_oracle_factor
        .get(native_token.id.clone())
        .unwrap();

    let bridging_fee_conversion_factor_on_contract = bridge_config
        .bridging_fee_conversion_factor
        .get(native_token.id)
        .unwrap();

    assert_eq!(pool_id_on_contract, pool.id);
    assert_eq!(from_gas_oracle_factor_on_contract, from_gas_oracle_factor);
    assert_eq!(
        bridging_fee_conversion_factor_on_contract,
        bridging_fee_conversion_factor
    );
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_gas_oracle_no_auth() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();
    env.mock_auths(&[]);
    bridge.set_gas_oracle(&Address::generate(&env));
}

#[test]
fn set_gas_oracle() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();

    let new_gas_oracle = Address::generate(&env);
    bridge.client.set_gas_oracle(&new_gas_oracle);

    let get_gas_oracle = bridge.client.get_gas_oracle();
    assert_eq!(get_gas_oracle, new_gas_oracle);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_rebalancer_no_auth() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();
    env.mock_auths(&[]);
    bridge.set_rebalancer(&Address::generate(&env));
}

#[test]
fn set_rebalancer() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();

    let rebalancer = Address::generate(&env);
    bridge.client.set_rebalancer(&rebalancer);

    assert_eq!(bridge.client.get_config().rebalancer, rebalancer);
}

#[test]
fn set_messenger() {
    let BridgeEnv {
        env,
        bridge,
        ref admin,
        ..
    } = BridgeEnv::default();
    env.mock_auths(&[]);

    let messenger = Address::generate(&env);

    env.mock_auths(&[MockAuth {
        address: admin,
        invoke: &MockAuthInvoke {
            contract: &bridge.id,
            fn_name: "set_messenger",
            args: (&messenger,).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    bridge.client.set_messenger(&messenger);

    assert_eq!(bridge.client.get_config().messenger, messenger);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_messenger_no_auth() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();
    env.mock_auths(&[]);

    bridge.set_messenger(&Address::generate(&env));
}

#[test]
fn set_admin() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();

    let admin = Address::generate(&env);
    bridge.set_admin(&admin);

    assert_eq!(bridge.client.get_admin(), admin);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_admin_no_auth() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();
    env.mock_auths(&[]);
    bridge.set_admin(&Address::generate(&env));
}

#[test]
fn set_stop_authority() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();

    let stop_authority = Address::generate(&env);
    bridge.client.set_stop_authority(&stop_authority);

    assert_eq!(bridge.client.get_stop_authority(), stop_authority);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_stop_authority_no_auth() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();
    env.mock_auths(&[]);
    bridge.set_stop_authority(&Address::generate(&env));
}

#[test]
#[should_panic = "Contract(SwapProhibited)"]
fn sucessful_stop_swap_and_then_swap_and_bridge() {
    let bridge_env = BridgeEnv::default();
    let stop_authority = Address::generate(&bridge_env.env);

    bridge_env.bridge.client.set_stop_authority(&stop_authority);
    bridge_env.bridge.client.stop_swap();

    let bridge_config = bridge_env.bridge.client.get_config();
    assert!(!bridge_config.can_swap);

    bridge_env.do_swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        10.0,
        30_00.0,
        0.0,
        3.0,
        3.0,
        None,
    );
}

#[test]
#[should_panic = "Contract(SwapProhibited)"]
fn sucessful_stop_swap_and_then_receive_tokens() {
    let bridge_env = BridgeEnv::default();
    let BridgeEnv { ref bridge, .. } = bridge_env;

    let stop_authority = Address::generate(&bridge_env.env);
    bridge.client.set_stop_authority(&stop_authority);

    bridge.client.stop_swap();

    let bridge_config = bridge.client.get_config();
    assert!(!bridge_config.can_swap);

    bridge_env.do_receive_tokens(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        10.0,
        0.0,
        1.5,
        None,
    );
}

#[test]
#[should_panic = "Contract(SwapProhibited)"]
fn sucessful_stop_swap_and_then_swap() {
    let bridge_env = BridgeEnv::default();
    let BridgeEnv { ref bridge, .. } = bridge_env;

    let stop_authority = Address::generate(&bridge_env.env);
    bridge.client.set_stop_authority(&stop_authority);

    bridge.client.stop_swap();

    let bridge_config = bridge.client.get_config();
    assert!(!bridge_config.can_swap);

    bridge_env.do_swap(
        &bridge_env.alice,
        &bridge_env.alice,
        &bridge_env.yaro_token,
        &bridge_env.yusd_token,
        10.0,
        1.0,
        None,
        None,
    );
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn stop_swap_no_auth() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();

    let stop_authority = Address::generate(&env);
    bridge.client.set_stop_authority(&stop_authority);

    env.mock_auths(&[]);
    bridge.stop_swap();
}

#[test]
fn sucessful_swap_restart() {
    let bridge_env = BridgeEnv::default();
    let BridgeEnv {
        ref env,
        ref bridge,
        ..
    } = bridge_env;

    let stop_authority = Address::generate(env);
    bridge.client.set_stop_authority(&stop_authority);

    bridge.client.stop_swap();

    let bridge_config = bridge.client.get_config();
    assert!(!bridge_config.can_swap);

    bridge.client.start_swap();

    let bridge_config = bridge.client.get_config();
    assert!(bridge_config.can_swap);

    bridge_env.do_swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        10.0,
        30_00.0,
        0.0,
        3.0,
        3.0,
        None,
    );

    bridge_env.do_receive_tokens(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        10.0,
        0.0,
        1.5,
        None,
    );

    bridge_env.do_swap(
        &bridge_env.alice,
        &bridge_env.alice,
        &bridge_env.yaro_token,
        &bridge_env.yusd_token,
        10.0,
        1.0,
        None,
        None,
    );
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn swap_restart_no_auth() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();

    let stop_authority = Address::generate(&env);
    bridge.client.set_stop_authority(&stop_authority);
    bridge.client.stop_swap();

    let bridge_config = bridge.client.get_config();
    assert!(!bridge_config.can_swap);

    env.mock_auths(&[]);
    bridge.start_swap();
}

#[test]
fn register_bridge() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();

    let other_bridge = BytesN::random(&env);
    let chain_id = 5;

    bridge.register_bridge(chain_id, &other_bridge);

    let another_bridge = bridge.client.get_another_bridge(&chain_id);
    assert_eq!(another_bridge.address, other_bridge);
    assert_eq!(another_bridge.tokens.len(), 0);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn register_bridge_no_auth() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();
    env.mock_auths(&[]);
    bridge.register_bridge(5, &BytesN::random(&env));
}

#[test]
fn change_bridge_address() {
    let BridgeEnv {
        env,
        bridge,
        goerli_bridge,
        ..
    } = BridgeEnv::default();

    assert_eq!(
        bridge.client.get_another_bridge(&GOERLI_CHAIN_ID).address,
        goerli_bridge
    );

    let bridge_address = BytesN::random(&env);

    bridge.register_bridge(GOERLI_CHAIN_ID, &bridge_address);

    assert_eq!(
        bridge.client.get_another_bridge(&GOERLI_CHAIN_ID).address,
        bridge_address
    );
}

#[test]
fn add_bridge_token() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();

    let token = BytesN::random(&env);

    bridge.client.add_bridge_token(&GOERLI_CHAIN_ID, &token);

    let another_bridge = bridge.client.get_another_bridge(&GOERLI_CHAIN_ID);

    assert_eq!(another_bridge.tokens.len(), 2);
    assert!(another_bridge.tokens.get(token).unwrap());
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn add_bridge_token_no_auth() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();
    env.mock_auths(&[]);
    bridge.add_bridge_token(GOERLI_CHAIN_ID, &BytesN::random(&env));
}

#[test]
#[should_panic = "Contract(UnknownAnotherChain)"]
fn add_bridge_for_unregistered_bridge() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();
    bridge.add_bridge_token(10, &BytesN::random(&env));
}

#[test]
fn remove_bridge_token() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();

    let token = BytesN::random(&env);

    bridge.add_bridge_token(GOERLI_CHAIN_ID, &token);

    let another_bridge = bridge.client.get_another_bridge(&GOERLI_CHAIN_ID);

    assert_eq!(another_bridge.tokens.len(), 2);
    assert!(another_bridge.tokens.get(token.clone()).unwrap());

    bridge.client.remove_bridge_token(&GOERLI_CHAIN_ID, &token);

    let another_bridge = bridge.client.get_another_bridge(&GOERLI_CHAIN_ID);

    assert_eq!(another_bridge.tokens.len(), 2);
    assert!(!another_bridge.tokens.get(token).unwrap());
}

#[test]
fn withdraw_gas_tokens() {
    let BridgeEnv {
        env,
        admin,
        native_token,
        bridge,
        ..
    } = BridgeEnv::default();

    let user = Address::generate(&env);
    let gas_amount = 10_000_000u128;
    let half_gas_amount = gas_amount / 2;

    native_token.asset_client.mint(&user, &(gas_amount as i128));
    native_token
        .client
        .transfer(&user, &bridge.id, &(gas_amount as i128));

    let init_admin_token_balance = native_token.balance_of(&admin);
    let init_bridge_token_balance = native_token.balance_of(&bridge.id);

    bridge.client.withdraw_gas_tokens(&admin, &half_gas_amount);

    let admin_token_balance = native_token.balance_of(&admin);
    let bridge_token_balance = native_token.balance_of(&bridge.id);

    assert_eq!(
        admin_token_balance,
        init_admin_token_balance + half_gas_amount
    );
    assert_eq!(
        bridge_token_balance,
        init_bridge_token_balance - half_gas_amount
    );
}

#[test]
fn upgrade() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();
    let hash =  env.deployer().upload_contract_wasm(bridge::WASM);
    bridge.upgrade(&hash)
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn upgrade_no_auth() {
    let BridgeEnv { env, bridge, .. } = BridgeEnv::default();
    env.mock_auths(&[]);
    let hash =  env.deployer().upload_contract_wasm(bridge::WASM);
    bridge.upgrade(&hash)
}