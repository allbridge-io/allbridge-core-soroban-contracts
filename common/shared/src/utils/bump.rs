use soroban_sdk::Env;

use crate::consts::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};

#[inline(always)]
pub fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}
