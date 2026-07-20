use shared::{require, Error};

use crate::methods::MAX_FEE_SHARE_DENOMINATOR;

const ADMIN_FEE_DENOMINATOR: u128 = 10_000;

pub(crate) fn validate_max_fee_share(value: i128) -> Result<(), Error> {
    require!(
        (0..=MAX_FEE_SHARE_DENOMINATOR).contains(&value),
        Error::InvalidArg
    );
    Ok(())
}

pub(crate) fn validate_admin_fee_share(value: u64) -> Result<(), Error> {
    require!(value <= ADMIN_FEE_DENOMINATOR as u64, Error::InvalidArg);
    Ok(())
}

pub(crate) fn admin_fee_denominator() -> u128 {
    ADMIN_FEE_DENOMINATOR
}
