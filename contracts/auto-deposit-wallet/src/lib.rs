#![no_std]

mod contract;
mod events;
mod methods;
mod storage;

mod bridge {
    soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/bridge.wasm");
}

mod messenger {
    soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/messenger.wasm");
}

mod gas_oracle {
    soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/gas_oracle.wasm");
}

pub use crate::contract::AutoDepositWalletContract;
