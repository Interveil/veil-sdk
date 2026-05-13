use crate::error::VeilError;
use crate::intent::intent::Intent;
use crate::signer::Signer;

/// A signed intent ready for submission to the node.
#[derive(Debug, Clone)]
pub struct SignedIntent {
    pub intent: Intent,
    pub pubkey: Vec<u8>,
    pub signature: Vec<u8>,
}

impl SignedIntent {
    /// Serialize to JSON for HTTP submission to node.
    /// Format:
    /// {
    ///   "intent": "<base64 encoded intent bytes>",
    ///   "pubkey": "<hex encoded public key>",
    ///   "signature": "<hex encoded signature>"
    /// }
    pub fn to_json(&self) -> Result<String, VeilError> {
        let intent_b64 = base64_encode(&self.intent.to_bytes()?);
        let pubkey_hex = hex_encode(&self.pubkey);
        let sig_hex = hex_encode(&self.signature);

        // Manual JSON construction to avoid serde_json dependency in main code
        Ok(format!(
            r#"{{"intent":"{}","pubkey":"{}","signature":"{}"}}"#,
            intent_b64, pubkey_hex, sig_hex
        ))
    }
}

impl Intent {
    /// Sign this intent with the given signer.
    ///
    /// Flow:
    ///   intent → to_bytes() → blake3 hash (32 bytes) → signer.sign(hash) → SignedIntent
    ///
    /// We hash before signing because:
    ///   1. Fixed 32-byte input regardless of intent size
    ///   2. Deterministic
    ///   3. Standard practice across major chains
    pub fn sign(&self, signer: &dyn Signer) -> Result<SignedIntent, VeilError> {
        let intent_bytes = self.to_bytes()?;
        let hash = blake3::hash(&intent_bytes);
        let signature = signer.sign(hash.as_bytes())?;
        let pubkey = signer.public_key();

        Ok(SignedIntent {
            intent: self.clone(),
            pubkey,
            signature,
        })
    }
}

// --- Encoding helpers (no external dependency) ---

fn base64_encode(data: &[u8]) -> String {
    // Minimal base64 implementation
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}
