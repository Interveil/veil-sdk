use crate::error::VeilError;
use crate::intent::sign::SignedIntent;

/// Response from the node after submitting a signed intent.
#[derive(Debug, Clone)]
pub struct SubmitResponse {
    pub tx_hash: String,
    pub status: String,
}

/// HTTP client for the Interveil execution node.
///
/// Usage:
///   let client = Client::new("http://localhost:3030");
///   let response = client.submit(&signed_intent)?;
///   println!("{}", response.tx_hash);
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
}

impl Client {
    /// Create a new client pointing to an Interveil node.
    ///
    /// `base_url` should NOT have a trailing slash.
    /// Example: "http://localhost:3030"
    pub fn new(base_url: &str) -> Self {
        let url = base_url.trim_end_matches('/');
        Self {
            base_url: url.to_string(),
        }
    }

    /// Submit a signed intent to the node for execution on Solana.
    ///
    /// POST {base_url}/intents
    /// Content-Type: application/json
    /// Body: output of signed_intent.to_json()
    ///
    /// Returns SubmitResponse with tx_hash on success.
    /// Returns VeilError::Http on failure.
    pub fn submit(&self, signed_intent: &SignedIntent) -> Result<SubmitResponse, VeilError> {
        let url = format!("{}/intents", self.base_url);
        let body = signed_intent.to_json()?;

        let response = reqwest::blocking::Client::new()
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .map_err(|e| VeilError::Http(format!("request failed: {}", e)))?;

        let status = response.status();
        let response_text = response
            .text()
            .map_err(|e| VeilError::Http(format!("failed to read response: {}", e)))?;

        if !status.is_success() {
            return Err(VeilError::Http(format!(
                "node returned status {}: {}",
                status, response_text
            )));
        }

        // Parse response JSON
        // Expected: {"tx_hash":"...","status":"..."}
        let parsed: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| VeilError::Http(format!("invalid JSON response: {}", e)))?;

        let tx_hash = parsed["tx_hash"]
            .as_str()
            .ok_or_else(|| VeilError::Http("missing tx_hash in response".to_string()))?
            .to_string();

        let status_str = parsed["status"]
            .as_str()
            .ok_or_else(|| VeilError::Http("missing status in response".to_string()))?
            .to_string();

        Ok(SubmitResponse {
            tx_hash,
            status: status_str,
        })
    }

    /// Return the base URL this client is configured with.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
