use interveil_sdk::{Chain, Intent};

#[test]
fn intent_to_bytes_works() {
    let intent = Intent {
        version: 1,
        chain: Chain::Solana,
        action: "transfer_sol".to_string(),
        payload: serde_json::json!({
            "to": "11111111111111111111111111111111",
            "lamports": 1_000_000_000
        }),
        nonce: 42,
        expires_at: 999,
    };

    let bytes = intent.to_bytes().unwrap();
    assert!(!bytes.is_empty());
}

#[test]
fn deterministic_serialization() {
    let intent = Intent {
        version: 1,
        chain: Chain::Solana,
        action: "transfer_sol".to_string(),
        payload: serde_json::json!({
            "to": "11111111111111111111111111111111",
            "lamports": 1_000_000_000
        }),
        nonce: 42,
        expires_at: 999,
    };

    let bytes1 = intent.to_bytes().unwrap();
    let bytes2 = intent.to_bytes().unwrap();
    assert_eq!(bytes1, bytes2);
}

#[test]
fn different_nonce_different_bytes() {
    let intent1 = Intent {
        version: 1,
        chain: Chain::Solana,
        action: "transfer_sol".to_string(),
        payload: serde_json::json!({
            "to": "11111111111111111111111111111111",
            "lamports": 1_000_000_000
        }),
        nonce: 42,
        expires_at: 999,
    };

    let mut intent2 = intent1.clone();
    intent2.nonce = 999;
    assert_ne!(intent1.to_bytes().unwrap(), intent2.to_bytes().unwrap());
}
