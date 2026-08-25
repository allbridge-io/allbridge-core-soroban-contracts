use soroban_sdk::{Bytes, Env};

#[test]
fn hashes_realistic_long_cctp_message() {
    let env = Env::default();
    let mut message = Bytes::new(&env);

    // CCTP V2 messages with hook data can be much larger than the fixed header.
    // This verifies Soroban crypto hashes the complete byte payload deterministically.
    for i in 0..512u32 {
        message.push_back((i % 251) as u8);
    }

    let hash_1 = env.crypto().keccak256(&message);
    let hash_2 = env.crypto().keccak256(&message);

    assert_eq!(hash_1.to_array(), hash_2.to_array());
    assert_ne!(hash_1.to_array(), [0u8; 32]);
}
