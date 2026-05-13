use interveil_sdk::{Chain, Intent, IntentSigner, VeilError};

/// Deterministic mock signer for testing.
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
fn signed_intent_contains_signer_and_signature() {
    let signer = MockSigner::new("test_signer_key");
    let intent = reference_intent();

    let signed = intent.sign(&signer).unwrap();

    assert_eq!(signed.signer, "test_signer_key");
    assert!(!signed.signature.is_empty());
    assert_eq!(signed.intent, intent);
}

#[test]
fn sign_intent_signs_canonical_hash_not_raw_serialized_intent() {
    let signer = MockSigner::new("mock_pubkey");
    let intent = reference_intent();

    let raw_bytes = intent.to_bytes().unwrap();
    let canonical_hash = blake3::hash(&raw_bytes);

    // If the signer signed the raw serialized bytes, the signature would be blake3(raw_bytes)
    let signature_if_raw_signed = blake3::hash(&raw_bytes).as_bytes().to_vec();

    // The actual signed intent
    let signed = intent.sign(&signer).unwrap();

    // The canonical hash (what SHOULD be signed) is blake3(raw_bytes)
    // The signer returns blake3(message), so if canonical_hash is passed:
    //   signature = blake3(canonical_hash_bytes) = blake3(blake3(raw_bytes))
    let expected_signature = blake3::hash(canonical_hash.as_bytes()).as_bytes().to_vec();

    // Verify the signature equals the double-hash, not the single-hash
    assert_eq!(
        signed.signature, expected_signature,
        "signature must be derived from canonical hash, not raw bytes"
    );
    assert_ne!(
        signed.signature, signature_if_raw_signed,
        "signature must NOT be derived from raw serialized bytes"
    );
}
