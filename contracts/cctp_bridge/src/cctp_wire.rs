use shared::Error;
use soroban_sdk::{Bytes, BytesN, Env};

pub const CCTP_V2_HEADER_VERSION: u32 = 1;
pub const BURN_MESSAGE_V2_VERSION: u32 = 1;

const OFF_HEADER_VERSION: u32 = 0;
const OFF_SOURCE_DOMAIN: u32 = 4;
const OFF_RECIPIENT: u32 = 76;
const OFF_DESTINATION_CALLER: u32 = 108;
const HEADER_LEN: u32 = 148;

const OFF_BODY_VERSION: u32 = HEADER_LEN;
const OFF_BODY_MINT_RECIPIENT: u32 = HEADER_LEN + 36;
const MIN_MESSAGE_LEN: u32 = HEADER_LEN + 228;

#[derive(Clone)]
pub struct CctpV2Decoded {
    pub source_domain: u32,
    pub recipient: BytesN<32>,
    pub destination_caller: BytesN<32>,
    pub burn_token: BytesN<32>,
    pub mint_recipient: BytesN<32>,
    pub hook_data: Bytes,
}

pub fn parse_cctp_v2(env: &Env, message: &Bytes) -> Result<CctpV2Decoded, Error> {
    if message.len() < MIN_MESSAGE_LEN {
        return Err(Error::InvalidArg);
    }
    if read_u32_be(message, OFF_HEADER_VERSION) != CCTP_V2_HEADER_VERSION {
        return Err(Error::InvalidArg);
    }
    if read_u32_be(message, OFF_BODY_VERSION) != BURN_MESSAGE_V2_VERSION {
        return Err(Error::InvalidArg);
    }

    let hook_data_offset = HEADER_LEN + 228;
    let hook_data = message.slice(hook_data_offset..message.len());

    Ok(CctpV2Decoded {
        source_domain: read_u32_be(message, OFF_SOURCE_DOMAIN),
        recipient: read_bytes32(env, message, OFF_RECIPIENT),
        destination_caller: read_bytes32(env, message, OFF_DESTINATION_CALLER),
        burn_token: read_bytes32(env, message, OFF_BODY_VERSION + 4),
        mint_recipient: read_bytes32(env, message, OFF_BODY_MINT_RECIPIENT),
        hook_data,
    })
}

fn read_u32_be(b: &Bytes, offset: u32) -> u32 {
    let b0 = b.get(offset).unwrap_or(0) as u32;
    let b1 = b.get(offset + 1).unwrap_or(0) as u32;
    let b2 = b.get(offset + 2).unwrap_or(0) as u32;
    let b3 = b.get(offset + 3).unwrap_or(0) as u32;
    (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
}

fn read_bytes32(env: &Env, b: &Bytes, offset: u32) -> BytesN<32> {
    let mut buf = [0u8; 32];
    for i in 0..32u32 {
        buf[i as usize] = b.get(offset + i).unwrap_or(0);
    }
    BytesN::from_array(env, &buf)
}
