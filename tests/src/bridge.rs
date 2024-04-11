use shared::consts::CHAIN_ID;

use soroban_sdk::Address;
use soroban_sdk::{testutils::Address as _, testutils::BytesN as _, BytesN};

use crate::utils::{consts::GOERLI_CHAIN_ID, contract_id};
use crate::utils::{float_to_uint_sp, gen_nonce, BridgeEnv, BridgeEnvConfig, ExpectedPoolDiff};

#[test]
fn swap_and_bridge() {
    let bridge_env = BridgeEnv::default();

    bridge_env.do_swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        3_000.0,
        0.0,
        3.0,
        3.0,
        Some(ExpectedPoolDiff {
            v_usd_diff: 999.513,
            token_balance_diff: 1_000.0,
        }),
    );
}

#[test]
fn swap_and_bridge_fee_share_gt_zero() {
    let bridge_env = BridgeEnv::create(BridgeEnvConfig::default().with_yaro_fee_share(5.0));

    bridge_env.do_swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        3_000.0,
        0.0,
        3.0,
        3.0,
        Some(ExpectedPoolDiff {
            v_usd_diff: 949.56,
            token_balance_diff: 950.0,
        }),
    );
}

#[test]
fn swap_and_bridge_near_zero() {
    let bridge_env = BridgeEnv::default();

    bridge_env.do_swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        0.0001,
        3_000.0,
        0.0,
        3.0,
        3.0,
        Some(ExpectedPoolDiff {
            v_usd_diff: 0.0,
            token_balance_diff: 0.0,
        }),
    );
}

#[test]
fn swap_and_bridge_near_zero_unbalanced_pool() {
    let bridge_env = BridgeEnv::default();

    bridge_env.do_receive_tokens(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        200_000.0,
        0.0,
        200_000.0,
        Some(ExpectedPoolDiff {
            v_usd_diff: 200_000.0,
            token_balance_diff: 49_917.401,
        }),
    );

    bridge_env.do_swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        0.001,
        3_000.0,
        0.0,
        3.0,
        3.0,
        Some(ExpectedPoolDiff {
            v_usd_diff: 0.549,
            token_balance_diff: 0.001,
        }),
    );
}

#[test]
fn swap_and_bridge_fee_fully_in_token() {
    let bridge_env = BridgeEnv::default();

    bridge_env.do_swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        0.0,
        5.0,
        3.0,
        3.0,
        Some(ExpectedPoolDiff {
            v_usd_diff: 994.518,
            token_balance_diff: 995.0,
        }),
    );
}

#[test]
fn swap_and_bridge_fee_partially_in_token() {
    let bridge_env = BridgeEnv::default();

    bridge_env.do_swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        100.0,
        5.0,
        0.1,
        3.0,
        3.0,
        Some(ExpectedPoolDiff {
            v_usd_diff: 99.896,
            token_balance_diff: 99.9,
        }),
    );
}

#[test]
#[should_panic = "Contract(BridgeToTheZeroAddress)"]
fn swap_and_bridge_bridge_to_the_zero_address() {
    let bridge_env = BridgeEnv::default();

    bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        5_000.0,
        0.0,
        GOERLI_CHAIN_ID,
        &BytesN::<32>::from_array(&bridge_env.env, &[0; 32]),
        &bridge_env.goerli_token,
        &gen_nonce(&bridge_env.env),
    );
}

#[test]
#[should_panic = "Contract(InvalidOtherChainId)"]
fn swap_and_bridge_invalid_other_chain_id() {
    let bridge_env = BridgeEnv::default();

    bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        5_000.0,
        0.0,
        CHAIN_ID,
        &BytesN::random(&bridge_env.env),
        &bridge_env.goerli_token,
        &gen_nonce(&bridge_env.env),
    );
}

#[test]
#[should_panic = "Contract(UnknownAnotherChain)"]
fn swap_and_bridge_unknown_chain() {
    let bridge_env = BridgeEnv::default();

    bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        5_000.0,
        0.0,
        10,
        &BytesN::random(&bridge_env.env),
        &bridge_env.goerli_token,
        &gen_nonce(&bridge_env.env),
    );
}

#[test]
#[should_panic = "Contract(UnknownAnotherToken)"]
fn swap_and_bridge_unknown_token() {
    let bridge_env = BridgeEnv::default();

    bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        5_000.0,
        0.0,
        GOERLI_CHAIN_ID,
        &BytesN::random(&bridge_env.env),
        &BytesN::random(&bridge_env.env),
        &gen_nonce(&bridge_env.env),
    );
}

#[test]
#[should_panic = "Contract(AmountTooLowForFee)"]
fn swap_and_bridge_amount_too_low_for_fee() {
    let bridge_env = BridgeEnv::default();

    bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        0.0,
        0.0,
        GOERLI_CHAIN_ID,
        &BytesN::random(&bridge_env.env),
        &bridge_env.goerli_token,
        &gen_nonce(&bridge_env.env),
    );
}

#[test]
pub fn receive_tokens() {
    let bridge_env = BridgeEnv::default();

    bridge_env.do_receive_tokens(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        100.0,
        0.0,
        1.5,
        Some(ExpectedPoolDiff {
            v_usd_diff: 100.0,
            token_balance_diff: 99.996,
        }),
    );
}

#[test]
pub fn receive_tokens_fee_share_gt_zero() {
    let bridge_env = BridgeEnv::create(BridgeEnvConfig::default().with_yaro_fee_share(5.0));

    bridge_env.do_receive_tokens(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        100.0,
        0.0,
        100.0,
        Some(ExpectedPoolDiff {
            v_usd_diff: 100.0,
            token_balance_diff: 99.996,
        }),
    );
}

#[test]
pub fn receive_tokens_zero_amount() {
    let bridge_env = BridgeEnv::default();

    bridge_env.do_receive_tokens(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        0.0,
        0.0,
        0.0,
        Some(ExpectedPoolDiff {
            v_usd_diff: 0.0,
            token_balance_diff: 0.0,
        }),
    );
}

#[test]
pub fn receive_tokens_extra_gas() {
    let bridge_env = BridgeEnv::default();

    bridge_env.do_receive_tokens(
        &bridge_env.alice,
        &bridge_env.yusd_token,
        100.0,
        0.0001,
        1.5,
        Some(ExpectedPoolDiff {
            v_usd_diff: 100.0,
            token_balance_diff: 99.996,
        }),
    );
}

#[test]
#[should_panic = "Contract(TokenInsufficientBalance)"]
pub fn receive_tokens_extra_gas_not_enough_native_token_on_bridge() {
    let bridge_env = BridgeEnv::default();
    let nonce = gen_nonce(&bridge_env.env);

    bridge_env.native_token.client.transfer(
        &bridge_env.bridge.id,
        &Address::generate(&bridge_env.env),
        &(bridge_env.native_token.balance_of(&bridge_env.bridge.id) as i128),
    );

    bridge_env.hash_and_receive_message(
        float_to_uint_sp(1_000.0),
        &bridge_env.alice.as_address(),
        &bridge_env.yaro_token,
        &nonce,
    );

    bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        1_000.0,
        &bridge_env.alice,
        GOERLI_CHAIN_ID,
        &bridge_env.yaro_token,
        &nonce,
        0.0,
        &Some(0.0001),
    );
}

#[test]
#[should_panic = "Contract(NoMessage)"]
pub fn receive_tokens_no_message() {
    let bridge_env = BridgeEnv::default();
    bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        1_000.0,
        &bridge_env.alice,
        GOERLI_CHAIN_ID,
        &bridge_env.yaro_token,
        &gen_nonce(&bridge_env.env),
        990.0,
        &Some(0.0),
    );
}

#[test]
#[should_panic = "Contract(SourceNotRegistered)"]
pub fn receive_tokens_source_not_registered() {
    let bridge_env = BridgeEnv::default();
    bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        1_000.0,
        &bridge_env.alice,
        10,
        &bridge_env.yaro_token,
        &gen_nonce(&bridge_env.env),
        990.0,
        &Some(0.0),
    );
}

#[test]
#[should_panic = "Contract(InsufficientReceivedAmount)"]
pub fn receive_tokens_insufficient_received_amount() {
    let bridge_env = BridgeEnv::default();

    let amount = 1_000.0;
    let nonce = gen_nonce(&bridge_env.env);

    bridge_env.hash_and_receive_message(
        float_to_uint_sp(amount),
        &bridge_env.alice.as_address(),
        &bridge_env.yaro_token,
        &nonce,
    );

    bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        amount,
        &bridge_env.alice,
        GOERLI_CHAIN_ID,
        &bridge_env.yaro_token,
        &nonce,
        amount + 10.0,
        &Some(0.0),
    );
}

#[test]
#[should_panic = "Contract(MessageProcessed)"]
pub fn receive_tokens_message_processed() {
    let bridge_env = BridgeEnv::default();

    let nonce = gen_nonce(&bridge_env.env);
    let amount = 1_000.0;
    let amount_sp = float_to_uint_sp(amount);

    bridge_env.hash_and_receive_message(
        float_to_uint_sp(amount),
        &bridge_env.alice.as_address(),
        &bridge_env.yaro_token,
        &nonce,
    );

    bridge_env.bridge.client.receive_tokens(
        &bridge_env.bridge.id,
        &amount_sp,
        &bridge_env.alice.as_address(),
        &GOERLI_CHAIN_ID,
        &contract_id(&bridge_env.yaro_token.id),
        &nonce,
        &0,
        &Some(0u128),
    );

    bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        amount,
        &bridge_env.alice,
        GOERLI_CHAIN_ID,
        &bridge_env.yaro_token,
        &nonce,
        0.0,
        &Some(0.0),
    );
}

#[test]
pub fn swap() {
    let bridge_env = BridgeEnv::default();

    bridge_env.do_swap(
        &bridge_env.alice,
        &bridge_env.alice,
        &bridge_env.yaro_token,
        &bridge_env.yusd_token,
        1_000.0,
        1.0,
        Some(ExpectedPoolDiff {
            v_usd_diff: 999.513,
            token_balance_diff: 1_000.0,
        }),
        Some(ExpectedPoolDiff {
            v_usd_diff: 999.513,
            token_balance_diff: 999.026,
        }),
    );
}
