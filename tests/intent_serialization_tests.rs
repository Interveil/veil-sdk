use interveil_sdk::{Chain, Intent};

const GOLDEN_HEX: &str = "01000000000c000000000000007472616e736665725f736f6c020000000000000008000000000000006c616d706f72747364000000000000000200000000000000746f200000000000000031313131313131313131313131313131313131313131313131313131313131312a00000000000000e703000000000000";

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

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[test]
fn serialization_is_deterministic_for_same_transfer_sol_intent() {
    let intent = reference_intent();
    let first_bytes = intent.to_bytes().unwrap();

    for _ in 0..1000 {
        let bytes = intent.to_bytes().unwrap();
        assert_eq!(bytes, first_bytes, "serialization is not deterministic");
    }
}

#[test]
fn canonical_serialization_matches_golden_output() {
    let intent = reference_intent();
    let bytes = intent.to_bytes().unwrap();
    let hex_output = bytes_to_hex(&bytes);

    assert_eq!(
        hex_output.len(),
        GOLDEN_HEX.len(),
        "golden hex length mismatch"
    );
    assert_eq!(
        hex_output, GOLDEN_HEX,
        "canonical serialization does not match golden output"
    );
}
