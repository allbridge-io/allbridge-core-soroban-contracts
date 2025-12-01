use rand::prelude::*;
use std::{
    any::type_name,
    cmp::Ordering,
    fmt::{Debug, Display},
};

use color_print::cformat;
use ethers_core::types::Signature;
use ethers_signers::LocalWallet;
use soroban_sdk::{
    testutils::Events,
    xdr::{ScError, ScVal},
    Address, BytesN, ConversionError, Env, Error as SorobanError, FromVal, InvokeError, Symbol,
    TryFromVal, Val, U256,
};

use soroban_sdk::xdr::ScAddress;

use super::consts::SP;

pub const SYSTEM_PRECISION: u32 = 3;

pub type CallResult<T = ()> = Result<T, SorobanError>;
pub type SorobanCallResult<T, E = ConversionError> =
    Result<Result<T, E>, Result<SorobanError, InvokeError>>;

pub fn error_code_to_error(v: u32) -> shared::Error {
    // don't try this at home
    unsafe { std::mem::transmute(v) }
}

pub fn unwrap_call_result<T>(env: &Env, call_result: CallResult<T>) -> T {
    let Err(error) = call_result else {
        return call_result.unwrap();
    };

    let val = ScVal::from_val(env, error.as_val());
    let sc_error = ScError::try_from(val).expect("Expect ScError");

    match sc_error {
        ScError::Contract(contract_error) => {
            panic!("Contract({:?})", error_code_to_error(contract_error))
        }
        _ => panic!("{:?}", sc_error),
    }
}

pub fn desoroban_result<T, E: Debug>(soroban_result: SorobanCallResult<T, E>) -> CallResult<T> {
    soroban_result.map(Result::unwrap).map_err(Result::unwrap)
}

pub fn gen_nonce(env: &Env) -> U256 {
    let mut rng = rand::thread_rng();
    U256::from_u32(env, rng.gen())
}

pub fn float_to_uint(amount: f64, decimals: u32) -> u128 {
    (amount * 10.0f64.powi(decimals as i32)) as u128
}

pub fn uint_to_float(amount: u128, decimals: u32) -> f64 {
    (amount as f64) / 10.0f64.powi(decimals as i32)
}

pub fn int_to_float(amount: i128, decimals: i32) -> f64 {
    (amount as f64) / 10.0f64.powi(decimals)
}

pub fn float_to_uint_sp(amount: f64) -> u128 {
    float_to_uint(amount, SP)
}

pub fn uint_to_float_sp(amount: u128) -> f64 {
    uint_to_float(amount, SP)
}

pub fn public_key_to_bytes(env: &Env, public_key: &str) -> BytesN<65> {
    let bytes = hex::decode(public_key).unwrap();
    let bytes = arrayref::array_ref![bytes, 0, 65];

    BytesN::<65>::from_array(env, bytes)
}

pub fn signature_to_bytes(env: &Env, signature: &Signature) -> BytesN<64> {
    let bytes = hex::decode(signature.to_string()).unwrap();
    let bytes = arrayref::array_ref![bytes, 0, 64];

    BytesN::<64>::from_array(env, bytes)
}

pub fn message_hash_vec_to_byte(env: &Env, message_hash: &[u8]) -> BytesN<32> {
    let message = arrayref::array_ref![message_hash, 0, 32];

    BytesN::<32>::from_array(env, message)
}

pub fn vec_to_bytes<const N: usize>(env: &Env, bytes: Vec<u8>) -> BytesN<N> {
    let mut slice: [u8; N] = [0; N];
    slice.copy_from_slice(bytes.as_slice());

    BytesN::from_array(env, &slice)
}

pub fn message_hash_to_byte(env: &Env, message_hash: &str) -> BytesN<32> {
    let message = hex::decode(message_hash).unwrap();
    let message = arrayref::array_ref![message, 0, 32];

    BytesN::<32>::from_array(env, message)
}

pub fn get_recover_id(signature: &Signature) -> u32 {
    signature.recovery_id().unwrap().to_byte() as u32
}

pub fn get_non_compress_public_key(wallet: &LocalWallet) -> String {
    hex::encode(wallet.signer().verifying_key().to_encoded_point(false))
}

pub fn get_private_key(wallet: &LocalWallet) -> String {
    let bytes = wallet.signer().to_bytes();

    hex::encode(bytes.as_slice())
}

pub fn format_diff<T: PartialOrd + Display>(start: T, to: T) -> String {
    match to.partial_cmp(&start).unwrap() {
        Ordering::Equal => cformat!("<dim>{start} => {to}</dim>"),
        Ordering::Greater => cformat!("<bright-green>{start} => {to}</bright-green>"),
        Ordering::Less => cformat!("<bright-red>{start} => {to}</bright-red>"),
    }
}

fn type_name_of_event<T: FromVal<Env, Val> + ?Sized>() -> String {
    static SPLITTERS: &[char] = &['(', ')', '[', ']', '<', '>', '{', '}', ' ', ',', '='];
    type_name::<T>()
        .split_inclusive(SPLITTERS)
        .flat_map(|component| component.rsplit("::").next())
        .collect()
}

pub fn get_latest_event<T: FromVal<Env, Val>>(env: &Env) -> Option<T> {
    env.events()
        .all()
        .iter()
        .rev()
        .find_map(|(_, topic, event_data)| {
            Symbol::try_from_val(env, &topic.last().unwrap())
                .map(|symbol| {
                    symbol
                        .to_string()
                        .eq(&type_name_of_event::<T>())
                        .then(|| T::from_val(env, &event_data))
                })
                .ok()
                .flatten()
        })
}

pub fn assert_rel_eq(a: u128, b: u128, d: u128) {
    assert!(
        a.abs_diff(b) <= d,
        "a: {}, b: {}, d: {}, diff: {}",
        a,
        b,
        d,
        a.abs_diff(b)
    );
}

pub fn contract_id(address: &Address) -> BytesN<32> {
    let sc_address: ScAddress = address.try_into().unwrap();
    if let ScAddress::Contract(c) = sc_address {
        BytesN::from_array(address.env(), c.0.as_ref())
    } else {
        panic!("address is not a contract {:?}", address);
    }
}

pub fn get_percentage(v: f64, percentage: f64) -> f64 {
    assert!((0.0..=100.0).contains(&percentage));

    v * (percentage / 100.0)
}

pub fn percentage_to_bp(percentage: f64) -> u128 {
    assert!((0.0..=100.0).contains(&percentage));

    (percentage * 100.0) as u128
}
