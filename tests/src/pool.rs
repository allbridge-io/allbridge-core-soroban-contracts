use crate::{
    contracts::pool::{Deposit, Withdraw},
    utils::{
        assert_rel_eq, float_to_uint_sp, format_diff, get_latest_event_unchecked_deprecated,
        BridgeEnv, BridgeEnvConfig, Pool,
    },
};

#[test]
fn deposit() {
    let BridgeEnv {
        env,
        yaro_pool,
        alice,
        yaro_token,
        ..
    } = BridgeEnv::default();

    let deposit_amount = 100.0;

    let balance_before = yaro_token.balance_of(&alice.as_address());
    yaro_pool.deposit(&alice, deposit_amount).unwrap();
    let deposit_event = get_latest_event_unchecked_deprecated::<Deposit>(&env);
    let balance_after = yaro_token.balance_of(&alice.as_address());

    assert_eq!(
        yaro_pool.user_deposit(&alice).lp_amount,
        float_to_uint_sp(deposit_amount)
    );
    assert_eq!(
        balance_before - balance_after,
        yaro_token.float_to_uint(deposit_amount)
    );

    assert_eq!(
        deposit_event,
        Deposit {
            user: alice.as_address(),
            amount: float_to_uint_sp(deposit_amount)
        }
    );
}

#[test]
fn withdraw() {
    let BridgeEnv {
        env,
        yaro_pool,
        alice,
        yaro_token,
        ..
    } = BridgeEnv::default();

    let withdraw_amount = 100.0;
    yaro_pool.deposit(&alice, withdraw_amount).unwrap();

    let balance_before = yaro_token.balance_of(&alice.as_address());
    yaro_pool.withdraw(&alice, withdraw_amount).unwrap();
    let withdraw_event = get_latest_event_unchecked_deprecated::<Withdraw>(&env);
    let balance_after = yaro_token.balance_of(&alice.as_address());

    assert_eq!(yaro_pool.user_deposit(&alice).lp_amount, 0);
    assert_eq!(
        balance_after - balance_before,
        yaro_token.float_to_uint(withdraw_amount)
    );

    assert_eq!(
        withdraw_event,
        Withdraw {
            user: alice.as_address(),
            amount: float_to_uint_sp(withdraw_amount)
        }
    );
}

#[test]
fn zero_diff() {
    let bridge_env = BridgeEnv::create(
        BridgeEnvConfig::default()
            .with_yaro_admin_deposit(1_000_000_000.0)
            .with_yusd_admin_deposit(1_000_000_000.0),
    );
    let BridgeEnv { ref yaro_pool, .. } = bridge_env;

    let total_lp_amount_before = yaro_pool.total_lp_amount();
    yaro_pool.client.adjust_total_lp_amount();
    let total_lp_amount_after = yaro_pool.total_lp_amount();

    println!(
        "Total lp amount change: {}",
        &format_diff(total_lp_amount_before, total_lp_amount_after)
    );

    assert_rel_eq(total_lp_amount_before, total_lp_amount_after, 5);
    assert_eq!(yaro_pool.d(), total_lp_amount_after);
}

#[test]
fn success() {
    let bridge_env = BridgeEnv::create(
        BridgeEnvConfig::default()
            .with_yaro_admin_deposit(1_000_000_000.0)
            .with_yusd_admin_deposit(1_000_000_000.0),
    );
    let BridgeEnv {
        ref yaro_pool,
        ref bob,
        ref yusd_pool,
        ref admin,
        ..
    } = bridge_env;

    let init_owner_lp_amount = yaro_pool.user_deposit_by_id(admin);

    let vusd_amount = yaro_pool.swap_to_v_usd(bob, 50_000_000.0);
    yusd_pool
        .client
        .swap_from_v_usd(&bob.as_address(), &vusd_amount, &0, &false);

    yaro_pool.deposit(bob, 50_000_000.0).unwrap();
    yaro_pool
        .withdraw_raw(bob, yaro_pool.user_deposit(bob).lp_amount)
        .unwrap();

    let total_lp_amount_before = yaro_pool.total_lp_amount();
    assert!(total_lp_amount_before < yaro_pool.d());

    yaro_pool.client.adjust_total_lp_amount();

    assert_eq!(yaro_pool.total_lp_amount(), yaro_pool.d());
    assert_eq!(
        yaro_pool.user_deposit_by_id(admin).lp_amount - init_owner_lp_amount.lp_amount,
        yaro_pool.d() - total_lp_amount_before
    );
}

#[test]
#[should_panic = "Contract(InvalidArg)"]
fn init_invalid_a() {
    let bridge_env = BridgeEnv::default();

    Pool::create(
        &bridge_env.env,
        &bridge_env.admin,
        &bridge_env.bridge.id,
        65,
        &bridge_env.yaro_token.id,
        10,
        10_000,
        100,
    );
}

#[test]
#[should_panic = "Contract(InvalidArg)"]
fn init_invalid_admin_fee_share() {
    let bridge_env = BridgeEnv::default();

    Pool::create(
        &bridge_env.env,
        &bridge_env.admin,
        &bridge_env.bridge.id,
        10,
        &bridge_env.yaro_token.id,
        10,
        10_000,
        10_000,
    );
}

#[test]
#[should_panic = "Contract(InvalidArg)"]
fn init_invalid_fee_share() {
    let bridge_env = BridgeEnv::default();

    Pool::create(
        &bridge_env.env,
        &bridge_env.admin,
        &bridge_env.bridge.id,
        10,
        &bridge_env.yaro_token.id,
        10_000,
        10_000,
        10,
    );
}
