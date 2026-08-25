use shared::Error;
use soroban_sdk::{xdr::ToXdr, Address, BytesN, Env};

const SC_VAL_TYPE_ADDRESS: u32 = 18;
const SC_ADDRESS_TYPE_ACCOUNT: u32 = 0;
const SC_ADDRESS_TYPE_CONTRACT: u32 = 1;
const PUBLIC_KEY_TYPE_ED25519: u32 = 0;
const SC_VAL_ADDRESS_CONTRACT_LEN: u32 = 4 + 4 + 32;
const SC_VAL_ADDRESS_ACCOUNT_LEN: u32 = 4 + 4 + 4 + 32;

pub fn address_to_bytes32(env: &Env, address: &Address) -> Result<BytesN<32>, Error> {
    let xdr_bytes = address.clone().to_xdr(env);
    let len = xdr_bytes.len();
    if len < 8 || read_u32_be(&xdr_bytes, 0) != SC_VAL_TYPE_ADDRESS {
        return Err(Error::InvalidArg);
    }

    match read_u32_be(&xdr_bytes, 4) {
        SC_ADDRESS_TYPE_CONTRACT => {
            if len != SC_VAL_ADDRESS_CONTRACT_LEN {
                return Err(Error::InvalidArg);
            }
            Ok(read_bytes32(env, &xdr_bytes, 8))
        }
        SC_ADDRESS_TYPE_ACCOUNT => {
            if len != SC_VAL_ADDRESS_ACCOUNT_LEN
                || read_u32_be(&xdr_bytes, 8) != PUBLIC_KEY_TYPE_ED25519
            {
                return Err(Error::InvalidArg);
            }
            Ok(read_bytes32(env, &xdr_bytes, 12))
        }
        _ => Err(Error::InvalidArg),
    }
}

fn read_u32_be(b: &soroban_sdk::Bytes, offset: u32) -> u32 {
    let b0 = b.get(offset).unwrap_or(0) as u32;
    let b1 = b.get(offset + 1).unwrap_or(0) as u32;
    let b2 = b.get(offset + 2).unwrap_or(0) as u32;
    let b3 = b.get(offset + 3).unwrap_or(0) as u32;
    (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
}

fn read_bytes32(env: &Env, b: &soroban_sdk::Bytes, offset: u32) -> BytesN<32> {
    let mut buf = [0u8; 32];
    for i in 0..32u32 {
        buf[i as usize] = b.get(offset + i).unwrap_or(0);
    }
    BytesN::from_array(env, &buf)
}
