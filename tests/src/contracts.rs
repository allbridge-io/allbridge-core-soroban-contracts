pub mod gas_oracle {
    soroban_sdk::contractimport!(file = "../target/wasm32v1-none/release/gas_oracle.wasm");
}

pub mod messenger {
    soroban_sdk::contractimport!(file = "../target/wasm32v1-none/release/messenger.wasm");
}

pub mod pool {
    soroban_sdk::contractimport!(file = "../target/wasm32v1-none/release/pool.wasm");
}

pub mod bridge {
    soroban_sdk::contractimport!(file = "../target/wasm32v1-none/release/bridge.wasm");
}

pub mod auto_deposit_factory {
    soroban_sdk::contractimport!(
        file = "../target/wasm32v1-none/release/auto_deposit_factory.wasm"
    );
}

pub mod auto_deposit_wallet {
    soroban_sdk::contractimport!(file = "../target/wasm32v1-none/release/auto_deposit_wallet.wasm");
}
