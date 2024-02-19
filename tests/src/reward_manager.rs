use crate::utils::{uint_to_float_sp, BalancesSnapshot, BridgeEnv, BridgeEnvConfig};

pub fn swap_a_to_b(bridge_env: &BridgeEnv, swap_amount: f64) {
    bridge_env.do_swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        swap_amount,
        3_000.0,
        0.0,
        3.0,
        3.0,
        None,
    );

    bridge_env.do_receive_tokens(
        &bridge_env.bob,
        &bridge_env.yusd_token,
        swap_amount,
        0.0,
        15.0,
        None,
    );
}

pub fn swap_b_to_a(bridge_env: &BridgeEnv, swap_amount: f64) {
    bridge_env.do_swap_and_bridge(
        &bridge_env.bob,
        &bridge_env.yusd_token,
        swap_amount,
        3_000.0,
        0.0,
        3.0,
        3.0,
        None,
    );

    bridge_env.do_receive_tokens(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        swap_amount,
        0.0,
        1.5,
        None,
    );
}

#[test]
fn common_flow() {
    let bridge_env = BridgeEnv::create(
        BridgeEnvConfig::default()
            .with_yaro_fee_share(1.0)
            .with_yusd_fee_share(10.0)
            .with_yaro_admin_deposit(0.0)
            .with_yusd_admin_deposit(0.0),
    );

    let deposit_amount = 2_000.0;
    let swap_amount = 100.0;

    bridge_env.do_deposit(deposit_amount, &bridge_env.alice, &bridge_env.yaro_pool);
    bridge_env.do_deposit(deposit_amount, &bridge_env.bob, &bridge_env.yusd_pool);

    swap_a_to_b(&bridge_env, swap_amount);
    swap_b_to_a(&bridge_env, swap_amount);
    // check alice reward

    let alice_reward = 2.002_399_9;
    let balances_before = BalancesSnapshot::take(&bridge_env);

    bridge_env
        .yaro_pool
        .claim_rewards(&bridge_env.alice)
        .unwrap();

    let alice_deposit_after = bridge_env.yaro_pool.user_deposit(&bridge_env.alice);
    let balances_after = BalancesSnapshot::take(&bridge_env);

    assert_eq!(uint_to_float_sp(bridge_env.yaro_pool.d()), 2_000.0);
    assert_eq!(uint_to_float_sp(alice_deposit_after.lp_amount), 2_000.0);
    assert_eq!(
        bridge_env
            .yaro_token
            .int_to_float(alice_deposit_after.reward_debt),
        alice_reward
    );

    assert_eq!(
        bridge_env
            .yaro_token
            .int_to_float(balances_before.pool_yaro_balance - balances_after.pool_yaro_balance),
        alice_reward
    );

    assert_eq!(
        bridge_env
            .yaro_token
            .int_to_float(balances_after.alice_yaro_balance - balances_before.alice_yaro_balance),
        alice_reward
    );

    // check bob reward

    let bob_reward = 19.975_499_9;

    let balances_before = BalancesSnapshot::take(&bridge_env);

    bridge_env.yusd_pool.claim_rewards(&bridge_env.bob).unwrap();

    let bob_deposit_after = bridge_env.yusd_pool.user_deposit(&bridge_env.bob);
    let balances_after = BalancesSnapshot::take(&bridge_env);

    assert_eq!(uint_to_float_sp(bridge_env.yusd_pool.d()), 2_000.0);
    assert_eq!(uint_to_float_sp(bob_deposit_after.lp_amount), 2_000.0);
    assert_eq!(
        bridge_env
            .yusd_token
            .int_to_float(bob_deposit_after.reward_debt),
        bob_reward
    );

    assert_eq!(
        bridge_env
            .yusd_token
            .int_to_float(balances_before.pool_yusd_balance - balances_after.pool_yusd_balance),
        bob_reward
    );

    assert_eq!(
        bridge_env
            .yusd_token
            .int_to_float(balances_after.bob_yusd_balance - balances_before.bob_yusd_balance),
        bob_reward
    );

    swap_a_to_b(&bridge_env, swap_amount);
    swap_b_to_a(&bridge_env, swap_amount);

    // alice withdraw

    bridge_env
        .yaro_pool
        .withdraw(&bridge_env.alice, 1995.0)
        .unwrap();

    let alice_deposit = bridge_env.yaro_pool.user_deposit(&bridge_env.alice);

    assert_eq!(uint_to_float_sp(bridge_env.yaro_pool.d()), 5.0);
    assert_eq!(uint_to_float_sp(alice_deposit.lp_amount), 5.0);
    assert_eq!(
        bridge_env
            .yaro_token
            .int_to_float(alice_deposit.reward_debt),
        0.010_011_8
    );

    // bob withdraw

    bridge_env
        .yusd_pool
        .withdraw(&bridge_env.bob, 1980.0)
        .unwrap();

    let bob_deposit = bridge_env.yusd_pool.user_deposit(&bridge_env.bob);

    assert_eq!(uint_to_float_sp(bridge_env.yusd_pool.d()), 20.01);
    assert_eq!(uint_to_float_sp(bob_deposit.lp_amount), 20.0);
    assert_eq!(
        bridge_env.yaro_token.int_to_float(bob_deposit.reward_debt),
        0.399_460_9
    );
}
