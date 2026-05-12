#[cfg(test)]
mod tests {
    use veil_sdk::VeilError;

    #[test]
    fn error_display_works() {
        let err = VeilError::Serialization("bincode failed".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("bincode failed"));
        assert!(msg.contains("serialization failed"));
    }

    #[test]
    fn error_display_signing() {
        let err = VeilError::Signing("key rejected".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("key rejected"));
        assert!(msg.contains("signing failed"));
    }

    #[test]
    fn error_display_http() {
        let err = VeilError::Http("connection refused".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("connection refused"));
        assert!(msg.contains("http request failed"));
    }

    #[test]
    fn error_display_invalid_intent() {
        let err = VeilError::InvalidIntent("zero lamports".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("zero lamports"));
        assert!(msg.contains("invalid intent"));
    }
}
