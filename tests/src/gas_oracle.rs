use shared::consts::CHAIN_ID;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};
use crate::contracts::gas_oracle;

use crate::utils::{GasOracle};

struct GasOracleEnv {
    pub admin: Address,
    pub gas_oracle: GasOracle,
    pub env: Env,
}

impl GasOracleEnv {
    pub fn setup() -> GasOracleEnv {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let gas_oracle = GasOracle::create(&env, &admin);

        GasOracleEnv {
            admin,
            gas_oracle,
            env,
        }
    }
}

#[test]
fn test_initialize() {
    GasOracleEnv::setup();
}

#[test]
#[should_panic = "Contract(Initialized)"]
fn test_initialize_already_initialized() {
    let gas_oracle_env = GasOracleEnv::setup();

    gas_oracle_env.gas_oracle.initialize(&gas_oracle_env.admin);
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn set_admin_no_auth() {
    let gas_oracle_env = GasOracleEnv::setup();

    gas_oracle_env.env.mock_auths(&[]);
    gas_oracle_env
        .gas_oracle
        .set_admin(&Address::generate(&gas_oracle_env.env));
}

#[test]
#[should_panic = "Contract(NoGasDataForChain)"]
fn test_initialize_no_gas_price_data() {
    let gas_oracle_env = GasOracleEnv::setup();

    gas_oracle_env.gas_oracle.get_gas_price(1);
}

#[test]
fn test_set_price() {
    let gas_oracle_env = GasOracleEnv::setup();

    let chain_id = 1;

    gas_oracle_env
        .gas_oracle
        .set_price(chain_id, Some(150), None);
    let new_gas_price = gas_oracle_env.gas_oracle.get_gas_price(chain_id);

    assert_eq!(new_gas_price.price, 150);
    assert_eq!(new_gas_price.gas_price, 0);

    gas_oracle_env
        .gas_oracle
        .set_price(chain_id, None, Some(100));
    let new_gas_price = gas_oracle_env.gas_oracle.get_gas_price(chain_id);

    assert_eq!(new_gas_price.price, 150);
    assert_eq!(new_gas_price.gas_price, 100);

    gas_oracle_env
        .gas_oracle
        .set_price(chain_id, Some(250), Some(150));
    let new_gas_price = gas_oracle_env.gas_oracle.get_gas_price(chain_id);

    assert_eq!(new_gas_price.price, 250);
    assert_eq!(new_gas_price.gas_price, 150);
}

#[test]
fn test_set_new_admin() {
    let gas_oracle_env = GasOracleEnv::setup();
    let new_admin = Address::generate(&gas_oracle_env.env);

    gas_oracle_env.gas_oracle.set_admin(&new_admin);
    assert_eq!(new_admin, gas_oracle_env.gas_oracle.get_admin());
}

#[test]
fn test_get_gas_cost_in_native_token() {
    let gas_oracle_env = GasOracleEnv::setup();

    let other_gas_price = 200_000_000;
    let other_price = 1_000_000_000;
    let this_price = 20_000_000;
    let gas_amount = 300_000_000;

    let expected_cost = 30_000_000;

    gas_oracle_env
        .gas_oracle
        .set_price(2, Some(other_price), Some(other_gas_price));
    gas_oracle_env
        .gas_oracle
        .set_price(CHAIN_ID, Some(this_price), Some(40));

    let cost = gas_oracle_env
        .gas_oracle
        .get_gas_cost_in_native_token(2, gas_amount);

    assert_eq!(expected_cost, cost);
}

#[test]
fn upgrade() {
    let gas_oracle_env = GasOracleEnv::setup();
    let hash =  gas_oracle_env.env.deployer().upload_contract_wasm(gas_oracle::WASM);
    gas_oracle_env.gas_oracle.upgrade(&hash)
}

#[test]
#[should_panic = "Context(InvalidAction)"]
fn upgrade_no_auth() {
    let gas_oracle_env = GasOracleEnv::setup();
    gas_oracle_env.env.mock_auths(&[]);
    let hash =  gas_oracle_env.env.deployer().upload_contract_wasm(gas_oracle::WASM);
    gas_oracle_env.gas_oracle.upgrade(&hash)
}