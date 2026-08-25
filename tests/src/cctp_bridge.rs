use soroban_sdk::testutils::{Address as _, BytesN as _, Events as _, MuxedAddress as _};
use soroban_sdk::{
    contract, contractimpl,
    xdr::{ContractEventBody, ToXdr},
    Address, Bytes, BytesN, Env, MuxedAddress, TryFromVal, TryIntoVal, Val,
};

use crate::contracts::{cctp_bridge, gas_oracle};
use crate::utils::get_latest_event;

const CHAIN_ID: u32 = 42;
const DOMAIN: u32 = 7;
const GAS_USAGE: u128 = 50_000;
const GAS_PRICE: u128 = 200_000_000;
const LOCAL_PRICE: u128 = 100_000_000;
const FEE_CONVERSION_FACTOR: u128 = 1_000_000_000;

#[contract]
pub struct TokenMessengerMock;

#[contractimpl]
impl TokenMessengerMock {
    pub fn set_local_token(env: Env, token: Option<Address>) {
        env.storage()
            .instance()
            .set(&soroban_sdk::symbol_short!("local"), &token);
    }

    pub fn get_local_token(
        env: Env,
        _remote_domain: u32,
        _remote_token: BytesN<32>,
    ) -> Option<Address> {
        env.storage()
            .instance()
            .get(&soroban_sdk::symbol_short!("local"))
            .flatten()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn deposit_for_burn(
        env: Env,
        caller: Address,
        amount: i128,
        destination_domain: u32,
        mint_recipient: BytesN<32>,
        burn_token: Address,
        destination_caller: BytesN<32>,
        max_fee: i128,
        min_finality_threshold: u32,
    ) {
        env.storage().instance().set(
            &soroban_sdk::symbol_short!("pburn"),
            &(
                caller,
                amount,
                destination_domain,
                mint_recipient,
                burn_token,
                destination_caller,
                max_fee,
                min_finality_threshold,
            ),
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn deposit_for_burn_with_hook(
        env: Env,
        caller: Address,
        amount: i128,
        destination_domain: u32,
        mint_recipient: BytesN<32>,
        burn_token: Address,
        destination_caller: BytesN<32>,
        max_fee: i128,
        min_finality_threshold: u32,
        hook_data: Bytes,
    ) {
        env.storage().instance().set(
            &soroban_sdk::symbol_short!("burn"),
            &(
                caller,
                amount,
                destination_domain,
                mint_recipient,
                burn_token,
                destination_caller,
                max_fee,
                min_finality_threshold,
                hook_data,
            ),
        );
    }

    pub fn last_plain_burn(
        env: Env,
    ) -> (
        Address,
        i128,
        u32,
        BytesN<32>,
        Address,
        BytesN<32>,
        i128,
        u32,
    ) {
        env.storage()
            .instance()
            .get(&soroban_sdk::symbol_short!("pburn"))
            .unwrap()
    }

    pub fn last_burn(
        env: Env,
    ) -> (
        Address,
        i128,
        u32,
        BytesN<32>,
        Address,
        BytesN<32>,
        i128,
        u32,
        Bytes,
    ) {
        env.storage()
            .instance()
            .get(&soroban_sdk::symbol_short!("burn"))
            .unwrap()
    }
}

#[contract]
pub struct MessageTransmitterMock;

#[contractimpl]
impl MessageTransmitterMock {
    pub fn initialize(env: Env, token: Address) {
        env.storage()
            .instance()
            .set(&soroban_sdk::symbol_short!("token"), &token);
        env.storage()
            .instance()
            .set(&soroban_sdk::symbol_short!("mint"), &true);
    }

    pub fn set_mint_enabled(env: Env, enabled: bool) {
        env.storage()
            .instance()
            .set(&soroban_sdk::symbol_short!("mint"), &enabled);
    }

    pub fn receive_message(env: Env, caller: Address, message: Bytes, attestation: Bytes) -> bool {
        let mint_enabled: bool = env
            .storage()
            .instance()
            .get(&soroban_sdk::symbol_short!("mint"))
            .unwrap();
        if mint_enabled {
            let amount = read_u128_be_from_u256(&message, 216);
            let fee = read_u128_be_from_u256(&message, 312);
            let minted_amount = i128::try_from((amount - fee) * 10).unwrap();
            let token: Address = env
                .storage()
                .instance()
                .get(&soroban_sdk::symbol_short!("token"))
                .unwrap();
            soroban_sdk::token::StellarAssetClient::new(&env, &token).mint(&caller, &minted_amount);
        }
        env.storage().instance().set(
            &soroban_sdk::symbol_short!("recv"),
            &(caller, message, attestation),
        );
        true
    }
}

struct Fixture {
    env: Env,
    admin: Address,
    bridge_id: Address,
    bridge: cctp_bridge::Client<'static>,
    usdc: soroban_sdk::token::Client<'static>,
    usdc_admin: soroban_sdk::token::StellarAssetClient<'static>,
    native: soroban_sdk::token::Client<'static>,
    native_admin: soroban_sdk::token::StellarAssetClient<'static>,
    token_messenger: TokenMessengerMockClient<'static>,
    token_messenger_id: Address,
    message_transmitter: MessageTransmitterMockClient<'static>,
    gas_oracle: gas_oracle::Client<'static>,
    other_bridge: BytesN<32>,
    recipient_account: Address,
}

fn fixture() -> Fixture {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();
    env.budget().reset_limits(u64::MAX, u64::MAX);

    let admin = Address::generate(&env);
    let usdc_asset = env.register_stellar_asset_contract_v2(admin.clone());
    let native_asset = env.register_stellar_asset_contract_v2(admin.clone());
    let usdc_id = usdc_asset.address();
    let native_id = native_asset.address();
    let recipient_account = native_asset.issuer().address();

    let gas_oracle_id = env.register_contract_wasm(None, gas_oracle::WASM);
    let gas_oracle = gas_oracle::Client::new(&env, &gas_oracle_id);
    gas_oracle.initialize(&admin);
    gas_oracle.set_price(&shared::consts::CHAIN_ID, &Some(LOCAL_PRICE), &Some(1));
    gas_oracle.set_price(&CHAIN_ID, &Some(LOCAL_PRICE), &Some(GAS_PRICE));

    let token_messenger_id = env.register_contract(None, TokenMessengerMock);
    let message_transmitter_id = env.register_contract(None, MessageTransmitterMock);
    let token_messenger = TokenMessengerMockClient::new(&env, &token_messenger_id);
    token_messenger.set_local_token(&Some(usdc_id.clone()));
    let message_transmitter = MessageTransmitterMockClient::new(&env, &message_transmitter_id);
    message_transmitter.initialize(&usdc_id);
    let bridge_id = env.register_contract_wasm(None, cctp_bridge::WASM);
    let bridge = cctp_bridge::Client::new(&env, &bridge_id);

    bridge.initialize(
        &admin,
        &usdc_id,
        &token_messenger_id,
        &message_transmitter_id,
        &gas_oracle_id,
        &native_id,
        &1,
        &100_000,
        &25,
        &FEE_CONVERSION_FACTOR,
    );

    let other_bridge = BytesN::random(&env);
    bridge.register_chain_bridge(&CHAIN_ID, &GAS_USAGE, &DOMAIN, &other_bridge);

    Fixture {
        env: env.clone(),
        admin,
        bridge_id,
        bridge,
        usdc: soroban_sdk::token::Client::new(&env, &usdc_id),
        usdc_admin: soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id),
        native: soroban_sdk::token::Client::new(&env, &native_id),
        native_admin: soroban_sdk::token::StellarAssetClient::new(&env, &native_id),
        token_messenger,
        token_messenger_id,
        message_transmitter,
        gas_oracle,
        other_bridge,
        recipient_account,
    }
}

#[test]
fn initialize_stores_config() {
    let f = fixture();

    assert_eq!(f.bridge.admin(), f.admin);
    assert_eq!(f.bridge.min_finality_threshold(), 1);
    assert_eq!(f.bridge.max_fee_share(), 100_000);
    assert_eq!(f.bridge.admin_fee_share_bp(), 25);
    assert_eq!(
        f.bridge.bridging_fee_conversion_factor(),
        FEE_CONVERSION_FACTOR
    );
}

#[test]
fn admin_registers_and_updates_chain_bridge() {
    let f = fixture();
    let updated_other_bridge = BytesN::random(&f.env);

    let chain_bridge = f.bridge.get_chain_bridge(&CHAIN_ID);
    assert_eq!(chain_bridge.chain_id, CHAIN_ID);
    assert_eq!(chain_bridge.gas_usage, GAS_USAGE);
    assert_eq!(chain_bridge.domain, DOMAIN);
    assert_eq!(chain_bridge.other_bridge, f.other_bridge);
    assert_eq!(f.bridge.get_domain_by_chain_id(&CHAIN_ID), DOMAIN);

    f.bridge.update_chain_bridge(
        &CHAIN_ID,
        &(GAS_USAGE + 1),
        &(DOMAIN + 1),
        &updated_other_bridge,
    );

    let chain_bridge = f.bridge.get_chain_bridge(&CHAIN_ID);
    assert_eq!(chain_bridge.gas_usage, GAS_USAGE + 1);
    assert_eq!(chain_bridge.domain, DOMAIN + 1);
    assert_eq!(chain_bridge.other_bridge, updated_other_bridge);

    let old_domain_amount = 1_000_000;
    let old_domain_message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &address_to_bytes32(&f.env, &f.bridge_id),
        &BytesN::random(&f.env),
        &address_to_bytes32(&f.env, &f.bridge_id),
        old_domain_amount,
        0,
        &f.other_bridge,
        &common_hook_data(
            &f.env,
            b"GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5",
        ),
    );
    f.usdc_admin.mint(&f.bridge_id, &10_000_000);
    let result = f.bridge.try_receive_tokens(
        &Address::generate(&f.env),
        &BytesN::random(&f.env),
        &old_domain_message,
        &Bytes::new(&f.env),
        &0,
    );
    assert!(result.is_err());
}

#[test]
fn admin_cannot_register_or_update_duplicate_cctp_domains() {
    let f = fixture();
    let other_bridge = BytesN::random(&f.env);

    let duplicate_register =
        f.bridge
            .try_register_chain_bridge(&(CHAIN_ID + 1), &GAS_USAGE, &DOMAIN, &other_bridge);
    assert!(duplicate_register.is_err());

    f.bridge
        .register_chain_bridge(&(CHAIN_ID + 1), &GAS_USAGE, &(DOMAIN + 1), &other_bridge);
    let duplicate_update =
        f.bridge
            .try_update_chain_bridge(&(CHAIN_ID + 1), &GAS_USAGE, &DOMAIN, &other_bridge);
    assert!(duplicate_update.is_err());
}

#[test]
fn bridge_collects_fees_and_calls_circle_with_remote_destination_caller() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let recipient = BytesN::random(&f.env);
    let amount = 10_000_000i128;
    let fee_token_amount = 1_000_000u128;
    let gas_amount = 9_000_000u128;

    f.usdc_admin.mint(&sender, &amount);
    f.native_admin.mint(&sender, &(gas_amount as i128));

    f.bridge.bridge(
        &sender,
        &(amount as u128),
        &recipient,
        &CHAIN_ID,
        &gas_amount,
        &fee_token_amount,
    );

    let admin_fee = (amount as u128 - fee_token_amount) * 25 / 10_000;
    let net_burn = amount as u128 - fee_token_amount - admin_fee;
    assert_eq!(f.usdc.balance(&f.bridge_id), amount);
    assert_eq!(f.native.balance(&f.bridge_id), gas_amount as i128);

    let (
        _,
        circle_amount,
        domain,
        mint_recipient,
        burn_token,
        destination_caller,
        max_fee,
        threshold,
    ) = f.token_messenger.last_plain_burn();
    assert_eq!(circle_amount, net_burn as i128);
    assert_eq!(domain, DOMAIN);
    assert_eq!(mint_recipient, recipient);
    assert_eq!(burn_token, f.usdc.address);
    assert_eq!(destination_caller, f.other_bridge);
    assert_eq!(max_fee, (net_burn * 100_000 / 1_000_000_000 + 1) as i128);
    assert_eq!(threshold, 1);
}

#[test]
fn bridge_charges_minimum_admin_fee_when_share_rounds_to_zero() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let recipient = BytesN::random(&f.env);
    let amount = 101i128;
    let gas_amount = 9_000_000u128;

    f.usdc_admin.mint(&sender, &amount);
    f.native_admin.mint(&sender, &(gas_amount as i128));

    f.bridge
        .bridge(&sender, &(amount as u128), &recipient, &CHAIN_ID, &gas_amount, &0);

    let event = get_latest_event::<cctp_bridge::TokensSent>(&f.env).unwrap();
    assert_eq!(event.admin_fee, 1);
    assert_eq!(event.amount, 100);

    let (_, circle_amount, _, _, _, _, _, _) = f.token_messenger.last_plain_burn();
    assert_eq!(circle_amount, 100);
}

#[test]
fn bridge_adds_cctp_wire_dust_to_admin_fee() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let recipient = BytesN::random(&f.env);
    let amount = 10_000_001i128;
    let fee_token_amount = 1_000_000u128;
    let gas_amount = 9_000_000u128;

    f.usdc_admin.mint(&sender, &amount);
    f.native_admin.mint(&sender, &(gas_amount as i128));

    f.bridge.bridge(
        &sender,
        &(amount as u128),
        &recipient,
        &CHAIN_ID,
        &gas_amount,
        &fee_token_amount,
    );

    let bridge_amount = amount as u128 - fee_token_amount;
    let configured_admin_fee = bridge_amount * 25 / 10_000;
    let unrounded_net_burn = bridge_amount - configured_admin_fee;
    let dust = unrounded_net_burn % 10;
    let expected_net_burn = unrounded_net_burn - dust;
    let expected_admin_fee = configured_admin_fee + dust;

    let event = get_latest_event::<cctp_bridge::TokensSent>(&f.env).unwrap();
    assert_eq!(event.amount, expected_net_burn);
    assert_eq!(event.admin_fee, expected_admin_fee);

    let (_, circle_amount, _, _, _, _, _, _) = f.token_messenger.last_plain_burn();
    assert_eq!(circle_amount, expected_net_burn as i128);
    assert_eq!(expected_net_burn % 10, 0);
}

#[test]
fn receive_tokens_forwards_to_g_address_from_hook_data() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let recipient = f.recipient_account.clone();
    let recipient_strkey = MuxedAddress::from(&recipient).to_strkey();
    let amount = 1_000_000;
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let burn_token = BytesN::random(&f.env);
    let hook_data = common_hook_data_from_bytes(&f.env, &recipient_strkey.to_bytes());
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &bridge_bytes,
        &burn_token,
        &bridge_bytes,
        amount,
        1_000,
        &f.other_bridge,
        &hook_data,
    );
    let attestation = Bytes::new(&f.env);
    let message_id = BytesN::random(&f.env);

    f.usdc_admin.trust(&recipient);
    f.bridge
        .receive_tokens(&sender, &message_id, &message, &attestation, &0);

    assert_eq!(f.usdc.balance(&recipient), 9_990_000);
}

#[test]
fn receive_tokens_uses_explicit_message_id() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let recipient = f.recipient_account.clone();
    let recipient_strkey = MuxedAddress::from(&recipient).to_strkey();
    let amount = 1_000_000;
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let burn_token = BytesN::random(&f.env);
    let hook_data = common_hook_data_from_bytes(&f.env, &recipient_strkey.to_bytes());
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &bridge_bytes,
        &burn_token,
        &bridge_bytes,
        amount,
        1_000,
        &f.other_bridge,
        &hook_data,
    );
    let message_id = BytesN::random(&f.env);

    f.usdc_admin.trust(&recipient);
    f.bridge
        .receive_tokens(&sender, &message_id, &message, &Bytes::new(&f.env), &0);

    let all_events = f.env.events().all();
    let event: cctp_bridge::TokensReceived = all_events
        .events()
        .iter()
        .rev()
        .find_map(|event| {
            let ContractEventBody::V0(body) = &event.body;
            let data: Val = body.data.try_into_val(&f.env).ok()?;
            cctp_bridge::TokensReceived::try_from_val(&f.env, &data).ok()
        })
        .unwrap();
    assert_eq!(event.message_id, message_id);
    assert_eq!(event.amount, 9_990_000);
    assert_eq!(event.recipient, recipient);
    assert_eq!(event.recipient_muxed_id, None);
    assert_eq!(event.source_chain_id, CHAIN_ID);
    assert_eq!(event.extra_gas_value, 0);
    assert_eq!(f.usdc.balance(&recipient), 9_990_000);
}

#[test]
fn receive_tokens_forwards_to_m_address_from_hook_data_and_emits_muxed_id() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let g_account = f.recipient_account.clone();
    let m_address = MuxedAddress::new(&g_account, 123456);
    let m_strkey = m_address.to_strkey();
    let amount = 1_000_000;
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let burn_token = BytesN::random(&f.env);
    let hook_data = common_hook_data_from_bytes(&f.env, &m_strkey.to_bytes());
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &bridge_bytes,
        &burn_token,
        &bridge_bytes,
        amount,
        1_000,
        &f.other_bridge,
        &hook_data,
    );

    f.usdc_admin.trust(&g_account);
    f.bridge.receive_tokens(
        &sender,
        &BytesN::random(&f.env),
        &message,
        &Bytes::new(&f.env),
        &0,
    );

    let all_events = f.env.events().all();
    let event: cctp_bridge::TokensReceived = all_events
        .events()
        .iter()
        .rev()
        .find_map(|event| {
            let ContractEventBody::V0(body) = &event.body;
            let data: Val = body.data.try_into_val(&f.env).ok()?;
            cctp_bridge::TokensReceived::try_from_val(&f.env, &data).ok()
        })
        .unwrap();
    assert_eq!(event.recipient_muxed_id, Some(123456));
    assert_eq!(f.usdc.balance(&g_account), 9_990_000);
}

#[test]
fn receive_tokens_rejects_invalid_hook_len() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let amount = 1_000_000;
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let burn_token = BytesN::random(&f.env);
    let mut hook_data = Bytes::new(&f.env);
    hook_data.extend_from_array(&100u32.to_be_bytes());
    hook_data.extend_from_slice(b"GA3D");
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &bridge_bytes,
        &burn_token,
        &bridge_bytes,
        amount,
        1_000,
        &f.other_bridge,
        &hook_data,
    );

    let result = f.bridge.try_receive_tokens(
        &sender,
        &BytesN::random(&f.env),
        &message,
        &Bytes::new(&f.env),
        &0,
    );

    assert!(result.is_err());
}

#[test]
fn receive_tokens_rejects_invalid_strkey() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let amount = 1_000_000;
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let burn_token = BytesN::random(&f.env);
    let hook_data = common_hook_data(&f.env, b"not-a-stellar-strkey");
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &bridge_bytes,
        &burn_token,
        &bridge_bytes,
        amount,
        1_000,
        &f.other_bridge,
        &hook_data,
    );

    let result = f.bridge.try_receive_tokens(
        &sender,
        &BytesN::random(&f.env),
        &message,
        &Bytes::new(&f.env),
        &0,
    );

    assert!(result.is_err());
}

#[test]
fn receive_tokens_accepts_any_source_sender_from_registered_domain() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let recipient = f.recipient_account.clone();
    let amount = 1_000_000;
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let burn_token = BytesN::random(&f.env);
    let recipient_strkey = MuxedAddress::from(&recipient).to_strkey();
    let hook_data = common_hook_data_from_bytes(&f.env, &recipient_strkey.to_bytes());
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &bridge_bytes,
        &burn_token,
        &bridge_bytes,
        amount,
        1_000,
        &BytesN::random(&f.env),
        &hook_data,
    );

    f.usdc_admin.trust(&recipient);
    f.bridge.receive_tokens(
        &sender,
        &BytesN::random(&f.env),
        &message,
        &Bytes::new(&f.env),
        &0,
    );

    assert_eq!(f.usdc.balance(&recipient), 9_990_000);
}

#[test]
fn receive_tokens_rejects_mint_recipient_that_is_not_bridge() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let recipient = Address::generate(&f.env);
    let amount = 1_000_000;
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let recipient_bytes = address_to_bytes32(&f.env, &recipient);
    let burn_token = BytesN::random(&f.env);
    let hook_data = common_hook_data(
        &f.env,
        b"GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5",
    );
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &bridge_bytes,
        &burn_token,
        &recipient_bytes,
        amount,
        0,
        &f.other_bridge,
        &hook_data,
    );

    let result = f.bridge.try_receive_tokens(
        &sender,
        &BytesN::random(&f.env),
        &message,
        &Bytes::new(&f.env),
        &0,
    );

    assert!(result.is_err());
}

#[test]
fn receive_tokens_rejects_destination_caller_that_is_not_bridge() {
    let f = fixture();
    let sender = Address::generate(&f.env);
    let amount = 1_000_000;
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let wrong_caller = BytesN::random(&f.env);
    let burn_token = BytesN::random(&f.env);
    let hook_data = common_hook_data(
        &f.env,
        b"GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5",
    );
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &wrong_caller,
        &burn_token,
        &bridge_bytes,
        amount,
        0,
        &f.other_bridge,
        &hook_data,
    );

    let result = f.bridge.try_receive_tokens(
        &sender,
        &BytesN::random(&f.env),
        &message,
        &Bytes::new(&f.env),
        &0,
    );

    assert!(result.is_err());
}

#[test]
fn receive_tokens_rejects_recipient_that_is_not_token_messenger() {
    let f = fixture();
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let hook_data = common_hook_data(
        &f.env,
        b"GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5",
    );
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &Address::generate(&f.env),
        &bridge_bytes,
        &BytesN::random(&f.env),
        &bridge_bytes,
        1_000_000,
        1_000,
        &f.other_bridge,
        &hook_data,
    );

    let result = f.bridge.try_receive_tokens(
        &Address::generate(&f.env),
        &BytesN::random(&f.env),
        &message,
        &Bytes::new(&f.env),
        &0,
    );

    assert!(result.is_err());
}

#[test]
fn receive_tokens_rejects_burn_token_without_local_usdc_route() {
    let f = fixture();
    f.token_messenger.set_local_token(&None);
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let hook_data = common_hook_data(
        &f.env,
        b"GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5",
    );
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &bridge_bytes,
        &BytesN::random(&f.env),
        &bridge_bytes,
        1_000_000,
        1_000,
        &f.other_bridge,
        &hook_data,
    );

    let result = f.bridge.try_receive_tokens(
        &Address::generate(&f.env),
        &BytesN::random(&f.env),
        &message,
        &Bytes::new(&f.env),
        &0,
    );

    assert!(result.is_err());
}

#[test]
fn receive_tokens_does_not_spend_existing_usdc_when_no_tokens_are_minted() {
    let f = fixture();
    let recipient = f.recipient_account.clone();
    let bridge_bytes = address_to_bytes32(&f.env, &f.bridge_id);
    let recipient_strkey = MuxedAddress::from(&recipient).to_strkey();
    let hook_data = common_hook_data_from_bytes(&f.env, &recipient_strkey.to_bytes());
    let message = inbound_cctp_message(
        &f.env,
        DOMAIN,
        &f.token_messenger_id,
        &bridge_bytes,
        &BytesN::random(&f.env),
        &bridge_bytes,
        1_000_000,
        1_000,
        &f.other_bridge,
        &hook_data,
    );
    f.usdc_admin.trust(&recipient);
    f.usdc_admin.mint(&f.bridge_id, &9_990_000);
    f.message_transmitter.set_mint_enabled(&false);

    let result = f.bridge.try_receive_tokens(
        &Address::generate(&f.env),
        &BytesN::random(&f.env),
        &message,
        &Bytes::new(&f.env),
        &0,
    );

    assert!(result.is_err());
    assert_eq!(f.usdc.balance(&recipient), 0);
    assert_eq!(f.usdc.balance(&f.bridge_id), 9_990_000);
}

fn address_to_bytes32(env: &Env, address: &Address) -> BytesN<32> {
    let xdr = address.to_xdr(env);
    let mut out = [0u8; 32];
    let start = xdr.len() - 32;
    for i in 0..32u32 {
        out[i as usize] = xdr.get(start + i).unwrap();
    }
    BytesN::from_array(env, &out)
}

fn common_hook_data(env: &Env, recipient: &[u8]) -> Bytes {
    let mut data = Bytes::new(env);
    data.extend_from_array(&(recipient.len() as u32).to_be_bytes());
    data.extend_from_slice(recipient);
    data
}

fn common_hook_data_from_bytes(env: &Env, recipient: &Bytes) -> Bytes {
    let mut data = Bytes::new(env);
    data.extend_from_array(&recipient.len().to_be_bytes());
    data.append(recipient);
    data
}

fn write_u32_be(buf: &mut [u8], offset: usize, value: u32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
}

fn write_u256_from_u128(buf: &mut [u8], offset: usize, value: u128) {
    buf[offset + 16..offset + 32].copy_from_slice(&value.to_be_bytes());
}

fn inbound_cctp_message(
    env: &Env,
    source_domain: u32,
    recipient: &Address,
    destination_caller: &BytesN<32>,
    burn_token: &BytesN<32>,
    mint_recipient: &BytesN<32>,
    amount: u128,
    fee_executed: u128,
    source_sender: &BytesN<32>,
    hook_data: &Bytes,
) -> Bytes {
    let mut buf = [0u8; 376];
    write_u32_be(&mut buf, 0, 1);
    write_u32_be(&mut buf, 4, source_domain);
    buf[76..108].copy_from_slice(&address_to_bytes32(env, recipient).to_array());
    buf[108..140].copy_from_slice(&destination_caller.to_array());
    write_u32_be(&mut buf, 148, 1);
    buf[152..184].copy_from_slice(&burn_token.to_array());
    buf[184..216].copy_from_slice(&mint_recipient.to_array());
    write_u256_from_u128(&mut buf, 216, amount);
    buf[248..280].copy_from_slice(&source_sender.to_array());
    write_u256_from_u128(&mut buf, 312, fee_executed);
    let mut message = Bytes::from_array(env, &buf);
    message.append(hook_data);
    message
}

fn read_u128_be_from_u256(bytes: &Bytes, offset: u32) -> u128 {
    let mut value = 0u128;
    for index in 16..32u32 {
        value = (value << 8) | bytes.get(offset + index).unwrap() as u128;
    }
    value
}
