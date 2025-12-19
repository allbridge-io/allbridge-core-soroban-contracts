#![no_std]

mod contract;
mod events;
mod methods;
mod storage;

mod gas_oracle {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/gas_oracle.wasm"
    );
}

pub use crate::contract::Messenger;
