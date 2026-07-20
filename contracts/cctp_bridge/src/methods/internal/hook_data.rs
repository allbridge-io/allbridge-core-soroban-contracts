use shared::Error;
use soroban_sdk::{Bytes, Env, MuxedAddress};

const RECIPIENT_LEN_BYTES: u32 = 4;

pub(crate) fn parse_common_hook_data(
    _env: &Env,
    hook_data: &Bytes,
) -> Result<MuxedAddress, Error> {
    if hook_data.len() < RECIPIENT_LEN_BYTES {
        return Err(Error::InvalidArg);
    }

    let recipient_len = read_u32_be(hook_data, 0);
    let expected_len = RECIPIENT_LEN_BYTES
        .checked_add(recipient_len)
        .ok_or(Error::InvalidArg)?;
    if hook_data.len() != expected_len {
        return Err(Error::InvalidArg);
    }

    let recipient_bytes = hook_data.slice(RECIPIENT_LEN_BYTES..expected_len);
    Ok(MuxedAddress::from_string_bytes(&recipient_bytes))
}

fn read_u32_be(bytes: &Bytes, offset: u32) -> u32 {
    let b0 = bytes.get(offset).unwrap_or(0) as u32;
    let b1 = bytes.get(offset + 1).unwrap_or(0) as u32;
    let b2 = bytes.get(offset + 2).unwrap_or(0) as u32;
    let b3 = bytes.get(offset + 3).unwrap_or(0) as u32;
    (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
}
