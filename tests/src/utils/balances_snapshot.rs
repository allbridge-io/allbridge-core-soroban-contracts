use crate::{contracts::pool::Pool as PoolInfo, utils::format_diff};
use color_print::cformat;
use core::panic;
use std::{cmp::Ordering, ops::Index};

use super::{int_to_float, BridgeEnv};

pub fn format_diff_with_float_diff(a: u128, b: u128, decimals: u32) -> (String, String) {
    let float_diff = int_to_float(b as i128 - a as i128, decimals as i32);

    let float_diff = match b.partial_cmp(&a).unwrap() {
        Ordering::Equal => String::new(),
        Ordering::Greater => cformat!("<bright-green>+{float_diff}</bright-green>"),
        Ordering::Less => cformat!("<bright-red>{float_diff}</bright-red>"),
    };

    (format_diff(a, b), float_diff)
}

#[derive(Debug, Clone)]
pub struct BalancesSnapshot {
    pub yaro_pool_info: PoolInfo,
    pub yusd_pool_info: PoolInfo,

    pub alice_yaro_balance: u128,
    pub alice_yusd_balance: u128,
    pub alice_native_balance: u128,

    pub bob_yaro_balance: u128,
    pub bob_yusd_balance: u128,
    pub bob_native_balance: u128,

    pub bridge_yaro_balance: u128,
    pub bridge_yusd_balance: u128,
    pub bridge_native_balance: u128,

    pub pool_yaro_balance: u128,
    pub pool_yusd_balance: u128,

    pub messenger_native_balance: u128,
}

impl Index<String> for BalancesSnapshot {
    type Output = u128;

    fn index(&self, string: String) -> &Self::Output {
        self.index(string.as_str())
    }
}

impl Index<&str> for BalancesSnapshot {
    type Output = u128;

    fn index(&self, string: &str) -> &Self::Output {
        match string {
            "alice_yaro_balance" => &self.alice_yaro_balance,
            "alice_yusd_balance" => &self.alice_yusd_balance,
            "alice_native_balance" => &self.alice_native_balance,
            "bob_yaro_balance" => &self.bob_yaro_balance,
            "bob_yusd_balance" => &self.bob_yusd_balance,
            "bob_native_balance" => &self.bob_native_balance,
            "bridge_yaro_balance" => &self.bridge_yaro_balance,
            "bridge_yusd_balance" => &self.bridge_yusd_balance,
            "bridge_native_balance" => &self.bridge_native_balance,
            "pool_yaro_balance" => &self.pool_yaro_balance,
            "pool_yusd_balance" => &self.pool_yusd_balance,
            "messenger_native_balance" => &self.messenger_native_balance,

            "pool_yaro_d" => &self.yaro_pool_info.d,
            "pool_yusd_d" => &self.yusd_pool_info.d,
            "pool_yaro_reserves" => &self.yaro_pool_info.reserves,
            "pool_yusd_reserves" => &self.yusd_pool_info.reserves,
            "pool_yaro_v_usd_balance" => &self.yaro_pool_info.v_usd_balance,
            "pool_yusd_v_usd_balance" => &self.yusd_pool_info.v_usd_balance,
            "pool_yaro_token_balance" => &self.yaro_pool_info.token_balance,
            "pool_yusd_token_balance" => &self.yusd_pool_info.token_balance,

            _ => panic!("BalancesSnapshot: unknown field: {}", string),
        }
    }
}

impl BalancesSnapshot {
    pub fn get_pool_info_by_tag(&self, tag: &str) -> PoolInfo {
        match tag {
            "yaro" => self.yaro_pool_info.clone(),
            "yusd" => self.yusd_pool_info.clone(),
            _ => panic!("Unexpected tag"),
        }
    }

    pub fn take(bridge_env: &BridgeEnv) -> BalancesSnapshot {
        let alice_address = bridge_env.alice.as_address();
        let bob_address = bridge_env.bob.as_address();

        let alice_yaro_balance = bridge_env.yaro_token.balance_of(&alice_address);
        let alice_yusd_balance = bridge_env.yusd_token.balance_of(&alice_address);
        let alice_native_balance = bridge_env.native_token.balance_of(&alice_address);

        let bob_yaro_balance = bridge_env.yaro_token.balance_of(&bob_address);
        let bob_yusd_balance = bridge_env.yusd_token.balance_of(&bob_address);
        let bob_native_balance = bridge_env.native_token.balance_of(&bob_address);

        let bridge_yaro_balance = bridge_env.yaro_token.balance_of(&bridge_env.bridge.id);
        let bridge_yusd_balance = bridge_env.yusd_token.balance_of(&bridge_env.bridge.id);
        let bridge_native_balance = bridge_env.native_token.balance_of(&bridge_env.bridge.id);

        let pool_yaro_balance = bridge_env.yaro_token.balance_of(&bridge_env.yaro_pool.id);
        let pool_yusd_balance = bridge_env.yusd_token.balance_of(&bridge_env.yusd_pool.id);

        let messenger_native_balance = bridge_env.native_token.balance_of(&bridge_env.messenger.id);

        BalancesSnapshot {
            yaro_pool_info: bridge_env.yaro_pool.client.get_pool(),
            yusd_pool_info: bridge_env.yusd_pool.client.get_pool(),
            alice_yaro_balance,
            bridge_yaro_balance,
            alice_native_balance,
            pool_yaro_balance,
            bridge_native_balance,
            messenger_native_balance,
            alice_yusd_balance,
            pool_yusd_balance,
            bridge_yusd_balance,
            bob_yaro_balance,
            bob_yusd_balance,
            bob_native_balance,
        }
    }

    #[allow(dead_code)]
    pub fn print_change_with(&self, other: &BalancesSnapshot, title: Option<&str>) {
        let title = title.unwrap_or("Diff");
        println!("----------------------| {title} |----------------------");

        let balances = vec![
            // Alice
            ("Alice native balance", "alice_native_balance", Some(7)),
            ("Alice yaro balance", "alice_yusd_balance", Some(7)),
            ("Alice yusd balance", "alice_yaro_balance", Some(7)),
            // Bob
            ("Bob native balance", "bob_native_balance", Some(7)),
            ("Bob yaro balance", "bob_yaro_balance", Some(7)),
            ("Bob yusd balance", "bob_yusd_balance", Some(7)),
            // Bridge
            ("Bridge native balance", "bridge_native_balance", Some(7)),
            ("Bridge yaro balance", "bridge_yaro_balance", Some(7)),
            ("Bridge yusd balance", "bridge_yusd_balance", Some(7)),
            // Messenger
            (
                "Messenger native balance",
                "messenger_native_balance",
                Some(7),
            ),
            // Pools
            ("Pool yaro balance", "pool_yaro_balance", Some(7)),
            ("Pool yusd balance", "pool_yusd_balance", Some(7)),
            ("Pool yaro d", "pool_yaro_d", Some(3)),
            ("Pool yusd d", "pool_yusd_d", Some(3)),
            ("Pool yaro reserves", "pool_yaro_reserves", Some(3)),
            ("Pool yusd reserves", "pool_yusd_reserves", Some(3)),
            ("Pool yaro vUSD", "pool_yaro_v_usd_balance", Some(3)),
            ("Pool yusd vUSD", "pool_yusd_v_usd_balance", Some(3)),
        ];

        for (title, value_key, use_float_diff) in balances {
            let (before, after) = (self[value_key], other[value_key]);

            match use_float_diff {
                Some(decimals) => {
                    let (balance_diff, diff) = format_diff_with_float_diff(before, after, decimals);
                    if diff.is_empty() {
                        println!("{}: {}", title, &balance_diff);
                    } else {
                        println!("{}: {} ({})", title, &balance_diff, &diff);
                    }
                }
                None => println!("{}: {}", title, format_diff(before, after)),
            }
        }
    }
}
