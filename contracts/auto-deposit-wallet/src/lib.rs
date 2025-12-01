#![no_std]

mod contract;
mod events;
mod methods;
mod storage;

mod bridge {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/bridge.wasm");
}

mod messenger {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/messenger.wasm"
    );
}

mod gas_oracle {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/gas_oracle.wasm"
    );
}

pub use crate::contract::AutoDepositWalletContract;
