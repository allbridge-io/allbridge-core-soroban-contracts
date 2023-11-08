#![no_std]
mod contract;
mod events;
mod internal;
mod storage;

mod gas_oracle {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/gas_oracle.wasm"
    );
}

pub use crate::contract::Messenger;
