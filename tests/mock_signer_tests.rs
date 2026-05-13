use interveil_sdk::{Chain, Intent, IntentSigner, VeilError};

/// Deterministic mock signer for testing.
/// Derives fake signatures from input bytes using blake3.
struct MockSigner {
    pub_key: String,
}

impl MockSigner {
    fn new(pub_key: &str) -> Self {
        Self {
            pub_key: pub_key.to_string(),
        }
    }
}

impl IntentSigner for MockSigner {
    fn public_key(&self) -> String {
        self.pub_key.clone()
    }

    fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, VeilError> {
        Ok(blake3::hash(message).as_bytes().to_vec())
    }
}

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

#[test]
fn sdk_signs_intent_via_mock_signer_trait() {
    let signer = MockSigner::new("mock_pubkey");
    let intent = reference_intent();

    let signed = intent.sign(&signer).unwrap();

    assert_eq!(signed.signer, "mock_pubkey");
    assert_eq!(signed.signature.len(), 32); // blake3 output
    assert_eq!(signed.intent, intent);
}

#[test]
fn same_intent_produces_same_signature_with_mock_signer() {
    let signer = MockSigner::new("mock_pubkey");
    let intent = reference_intent();

    let signed1 = intent.sign(&signer).unwrap();
    let signed2 = intent.sign(&signer).unwrap();

    assert_eq!(signed1.signature, signed2.signature);
}

#[test]
fn different_nonce_produces_different_signature_with_mock_signer() {
    let signer = MockSigner::new("mock_pubkey");

    let intent_a = reference_intent();

    let mut intent_b = reference_intent();
    intent_b.nonce = 43;

    let signed_a = intent_a.sign(&signer).unwrap();
    let signed_b = intent_b.sign(&signer).unwrap();

    assert_ne!(signed_a.signature, signed_b.signature);
}
