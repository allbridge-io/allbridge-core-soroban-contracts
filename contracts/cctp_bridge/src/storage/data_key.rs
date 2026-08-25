use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Config,
    ChainBridge(u32),
    Domain(u32),
}
