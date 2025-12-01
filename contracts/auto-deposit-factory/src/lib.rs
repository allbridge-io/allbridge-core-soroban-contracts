#![no_std]

mod contract;
mod events;
mod methods;
mod storage;

mod gas_oracle {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/gas_oracle.wasm"
    );
}

mod wallet {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/auto_deposit_wallet.wasm"
    );
}

pub use crate::contract::AutoDepositFactoryContract;
