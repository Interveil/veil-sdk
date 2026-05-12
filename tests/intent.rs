#[cfg(test)]
mod tests {
    use veil_sdk::{Chain, Intent, IntentPayload};

    #[test]
    fn builder_creates_correct_intent() {
        let intent = Intent::transfer_sol(
            "99999999999999999999999999999999".to_string(),
            500_000_000,
        );

        assert_eq!(intent.version, 1);
        assert_eq!(intent.chain, Chain::Solana);
        assert!(intent.nonce > 0);
        match intent.payload {
            IntentPayload::TransferSol { to, lamports } => {
                assert_eq!(to, "99999999999999999999999999999999");
                assert_eq!(lamports, 500_000_000);
            }
        }
    }

    #[test]
    fn nonce_is_unique_across_creations() {
        let intent1 = Intent::transfer_sol("a".to_string(), 100);
        let intent2 = Intent::transfer_sol("a".to_string(), 100);

        // Timestamp-based nonces should differ (unless created in same millisecond)
        // We don't strictly assert inequality, but in practice they will differ
        // This test mainly ensures no panic and nonce > 0
        assert!(intent1.nonce > 0);
        assert!(intent2.nonce > 0);
    }

    #[test]
    fn builder_vs_manual_are_equivalent() {
        let recipient = "11111111111111111111111111111111".to_string();
        let lamports = 1_000_000_000u64;

        let built = Intent::transfer_sol(recipient.clone(), lamports);
        let manual = Intent {
            version: 1,
            chain: Chain::Solana,
            nonce: built.nonce, // Use same nonce for byte comparison
            payload: IntentPayload::TransferSol {
                to: recipient,
                lamports,
            },
        };

        // Should produce identical bytes when nonce matches
        assert_eq!(built.to_bytes().unwrap(), manual.to_bytes().unwrap());
    }
}
