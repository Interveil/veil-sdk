#[cfg(test)]
mod tests {
    use interveil_sdk::{Chain, Intent, IntentPayload};

    fn make_intent(nonce: u64, lamports: u64) -> Intent {
        Intent {
            version: 1,
            chain: Chain::Solana,
            nonce,
            payload: IntentPayload::TransferSol {
                to: "11111111111111111111111111111111".to_string(),
                lamports,
            },
        }
    }

    #[test]
    fn deterministic_1000_iterations() {
        let intent = make_intent(42, 1_000_000_000);
        let first_bytes = intent.to_bytes().unwrap();

        for _ in 0..1000 {
            let bytes = intent.to_bytes().unwrap();
            assert_eq!(bytes, first_bytes, "serialization is not deterministic");
        }
    }

    #[test]
    fn different_nonce_different_bytes() {
        let a = make_intent(1, 1_000_000_000);
        let b = make_intent(2, 1_000_000_000);
        assert_ne!(a.to_bytes().unwrap(), b.to_bytes().unwrap());
    }

    #[test]
    fn different_lamports_different_bytes() {
        let a = make_intent(1, 100);
        let b = make_intent(1, 200);
        assert_ne!(a.to_bytes().unwrap(), b.to_bytes().unwrap());
    }

    #[test]
    fn different_recipient_different_bytes() {
        let a = Intent {
            version: 1,
            chain: Chain::Solana,
            nonce: 1,
            payload: IntentPayload::TransferSol {
                to: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                lamports: 100,
            },
        };
        let b = Intent {
            version: 1,
            chain: Chain::Solana,
            nonce: 1,
            payload: IntentPayload::TransferSol {
                to: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
                lamports: 100,
            },
        };
        assert_ne!(a.to_bytes().unwrap(), b.to_bytes().unwrap());
    }

    #[test]
    fn bytes_are_non_empty() {
        let intent = make_intent(1, 1);
        let bytes = intent.to_bytes().unwrap();
        assert!(!bytes.is_empty());
    }
}
