use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use veil_sdk::{Chain, Intent, IntentPayload, Signer as VeilSigner, VeilError};

/// Test signer using ed25519-dalek — simulates what wallet-sdk would do
struct TestSigner {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl TestSigner {
    fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }
}

impl VeilSigner for TestSigner {
    fn public_key(&self) -> Vec<u8> {
        self.verifying_key.to_bytes().to_vec()
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, VeilError> {
        let sig: Signature = self.signing_key.sign(message);
        Ok(sig.to_bytes().to_vec())
    }
}

#[test]
fn sign_produces_signed_intent() {
    let signer = TestSigner::new();
    let intent = Intent::transfer_sol(
        "11111111111111111111111111111111".to_string(),
        1_000_000_000,
    );

    let signed = intent.sign(&signer).unwrap();

    assert_eq!(signed.pubkey.len(), 32);
    assert_eq!(signed.signature.len(), 64);
}

#[test]
fn sign_pubkey_matches_signer() {
    let signer = TestSigner::new();
    let intent = Intent::transfer_sol(
        "11111111111111111111111111111111".to_string(),
        500_000_000,
    );

    let signed = intent.sign(&signer).unwrap();

    assert_eq!(signed.pubkey, signer.public_key());
}

#[test]
fn signature_is_verifiable() {
    let signer = TestSigner::new();
    let intent = Intent::transfer_sol(
        "22222222222222222222222222222222".to_string(),
        1_000_000_000,
    );

    let signed = intent.sign(&signer).unwrap();

    // Reconstruct the hash and verify with ed25519-dalek
    let intent_bytes = signed.intent.to_bytes().unwrap();
    let hash = blake3::hash(&intent_bytes);
    let sig: Signature = Signature::from_slice(&signed.signature).unwrap();

    signer
        .verifying_key
        .verify(hash.as_bytes(), &sig)
        .expect("signature verification failed");
}

#[test]
fn wrong_signer_fails_verification() {
    let signer_a = TestSigner::new();
    let signer_b = TestSigner::new();
    let intent = Intent::transfer_sol(
        "11111111111111111111111111111111".to_string(),
        1_000_000_000,
    );

    let signed = intent.sign(&signer_a).unwrap();

    // Try to verify with signer B's public key
    let intent_bytes = signed.intent.to_bytes().unwrap();
    let hash = blake3::hash(&intent_bytes);
    let sig: Signature = Signature::from_slice(&signed.signature).unwrap();

    assert!(
        signer_b.verifying_key.verify(hash.as_bytes(), &sig).is_err(),
        "signature should NOT verify with wrong key"
    );
}

#[test]
fn to_json_has_correct_structure() {
    let signer = TestSigner::new();
    let intent = Intent::transfer_sol(
        "11111111111111111111111111111111".to_string(),
        1_000_000_000,
    );

    let signed = intent.sign(&signer).unwrap();
    let json = signed.to_json().unwrap();

    // Must be valid JSON with 3 fields
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.get("intent").is_some());
    assert!(parsed.get("pubkey").is_some());
    assert!(parsed.get("signature").is_some());
    assert_eq!(parsed.as_object().unwrap().len(), 3);

    // pubkey and signature should be hex strings
    let pubkey_str = parsed["pubkey"].as_str().unwrap();
    let sig_str = parsed["signature"].as_str().unwrap();
    assert_eq!(pubkey_str.len(), 64); // 32 bytes = 64 hex chars
    assert_eq!(sig_str.len(), 128); // 64 bytes = 128 hex chars

    // intent should be non-empty base64
    let intent_b64 = parsed["intent"].as_str().unwrap();
    assert!(!intent_b64.is_empty());
}

#[test]
fn same_intent_same_hash() {
    let signer = TestSigner::new();
    let intent = Intent {
        version: 1,
        chain: Chain::Solana,
        nonce: 100,
        payload: IntentPayload::TransferSol {
            to: "11111111111111111111111111111111".to_string(),
            lamports: 1_000_000_000,
        },
    };

    let signed1 = intent.sign(&signer).unwrap();
    let signed2 = intent.sign(&signer).unwrap();

    // Same intent + same signer = same signature
    assert_eq!(signed1.signature, signed2.signature);
}
