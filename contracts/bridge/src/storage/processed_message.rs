use crate::storage::data_key::DataKey;
use shared::consts::DAY_IN_LEDGERS;
use soroban_sdk::{BytesN, Env};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessedMessage;

impl ProcessedMessage {
    const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
    const LIFETIME_THRESHOLD: u32 = Self::BUMP_AMOUNT - DAY_IN_LEDGERS;

    pub fn bump(env: &Env, key: &DataKey) {
        env.storage()
            .persistent()
            .bump(key, Self::LIFETIME_THRESHOLD, Self::BUMP_AMOUNT);
    }

    #[inline]
    pub fn set_processed(env: &Env, message: BytesN<32>) {
        let key = DataKey::ReceivedMessage(message);
        env.storage().persistent().set(&key, &true);
        Self::bump(env, &key);
    }

    #[inline]
    pub fn is_processed(env: &Env, message: BytesN<32>) -> bool {
        let key = DataKey::ReceivedMessage(message);
        if let Some(result) = env.storage().persistent().get(&key) {
            Self::bump(env, &key);
            result
        } else {
            false
        }
    }
}
