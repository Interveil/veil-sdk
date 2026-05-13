use interveil_sdk::{Chain, Intent};

fn reference_intent() -> Intent {
    Intent {
        version: 1,
        chain: Chain::Solana,
        action: "transfer_sol".to_string(),
        payload: serde_json::json!({"lamports": 100, "to": "11111111111111111111111111111111"}),
        nonce: 42,
        expires_at: 999,
    }
}

fn hash_hex(bytes: &[u8]) -> String {
    let hash = blake3::hash(bytes);
    hash.as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[test]
fn same_intent_produces_same_hash() {
    let intent_a = reference_intent();
    let intent_b = reference_intent();

    let bytes_a = intent_a.to_bytes().unwrap();
    let bytes_b = intent_b.to_bytes().unwrap();

    assert_eq!(
        bytes_a, bytes_b,
        "identical intents must serialize to identical bytes"
    );

    let hash_a = blake3::hash(&bytes_a);
    let hash_b = blake3::hash(&bytes_b);

    assert_eq!(
        hash_a.as_bytes(),
        hash_b.as_bytes(),
        "identical intents must produce identical canonical hashes"
    );
}

#[test]
fn different_nonce_produces_different_hash() {
    let intent_a = reference_intent();

    let mut intent_b = reference_intent();
    intent_b.nonce = 43;

    let bytes_a = intent_a.to_bytes().unwrap();
    let bytes_b = intent_b.to_bytes().unwrap();

    assert_ne!(
        bytes_a, bytes_b,
        "different nonce must produce different serialized output"
    );

    let hash_a = hash_hex(&bytes_a);
    let hash_b = hash_hex(&bytes_b);

    assert_ne!(
        hash_a, hash_b,
        "different nonce must produce different canonical hash"
    );
}

#[test]
fn different_expires_at_produces_different_hash() {
    let intent_a = reference_intent();

    let mut intent_b = reference_intent();
    intent_b.expires_at = 1000;

    let bytes_a = intent_a.to_bytes().unwrap();
    let bytes_b = intent_b.to_bytes().unwrap();

    assert_ne!(
        bytes_a, bytes_b,
        "different expires_at must produce different serialized output"
    );

    let hash_a = hash_hex(&bytes_a);
    let hash_b = hash_hex(&bytes_b);

    assert_ne!(
        hash_a, hash_b,
        "different expires_at must produce different canonical hash"
    );
}

#[test]
fn different_payload_amount_produces_different_hash() {
    let intent_a = reference_intent();

    let mut intent_b = reference_intent();
    intent_b.payload = serde_json::json!({"lamports": 200, "to": "11111111111111111111111111111111"});

    let bytes_a = intent_a.to_bytes().unwrap();
    let bytes_b = intent_b.to_bytes().unwrap();

    assert_ne!(
        bytes_a, bytes_b,
        "different payload amount must produce different serialized output"
    );

    let hash_a = hash_hex(&bytes_a);
    let hash_b = hash_hex(&bytes_b);

    assert_ne!(
        hash_a, hash_b,
        "different payload amount must produce different canonical hash"
    );
}
