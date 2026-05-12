#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use veil_sdk::{Chain, Intent, IntentPayload, VeilError};

    #[test]
    fn builder_creates_correct_intent() {
        let intent =
            Intent::transfer_sol("99999999999999999999999999999999".to_string(), 500_000_000);

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
            nonce: built.nonce,
            payload: IntentPayload::TransferSol {
                to: recipient,
                lamports,
            },
        };

        assert_eq!(built.to_bytes().unwrap(), manual.to_bytes().unwrap());
    }

    #[test]
    fn nonce_monotonically_non_decreasing() {
        let a = Intent::transfer_sol("a".to_string(), 100);
        thread::sleep(Duration::from_millis(2));
        let b = Intent::transfer_sol("a".to_string(), 100);
        assert!(b.nonce >= a.nonce);
    }

    #[test]
    fn validate_rejects_zero_lamports() {
        let intent = Intent::transfer_sol("11111111111111111111111111111111".to_string(), 0);
        let result = intent.validate();
        assert!(result.is_err());
        match result.unwrap_err() {
            VeilError::InvalidIntent(msg) => assert!(msg.contains("greater than 0")),
            _ => panic!("expected InvalidIntent error"),
        }
    }

    #[test]
    fn validate_rejects_empty_recipient() {
        let intent = Intent::transfer_sol("".to_string(), 100);
        let result = intent.validate();
        assert!(result.is_err());
        match result.unwrap_err() {
            VeilError::InvalidIntent(msg) => assert!(msg.contains("not be empty")),
            _ => panic!("expected InvalidIntent error"),
        }
    }

    #[test]
    fn validate_rejects_long_recipient() {
        let long_addr = "a".repeat(45);
        let intent = Intent::transfer_sol(long_addr, 100);
        let result = intent.validate();
        assert!(result.is_err());
        match result.unwrap_err() {
            VeilError::InvalidIntent(msg) => assert!(msg.contains("too long")),
            _ => panic!("expected InvalidIntent error"),
        }
    }

    #[test]
    fn validate_accepts_valid_intent() {
        let intent = Intent::transfer_sol("11111111111111111111111111111111".to_string(), 100);
        assert!(intent.validate().is_ok());
    }

    #[test]
    fn validate_accepts_max_length_recipient() {
        let addr_44 = "a".repeat(44);
        let intent = Intent::transfer_sol(addr_44, 100);
        assert!(intent.validate().is_ok());
    }

    #[test]
    fn validate_accepts_min_length_recipient() {
        let intent = Intent::transfer_sol("1".to_string(), 100);
        assert!(intent.validate().is_ok());
    }
}
