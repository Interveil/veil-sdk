use ed25519_dalek::{Signature, Signer as EdSigner, SigningKey, VerifyingKey};
use interveil_sdk::{Client, Intent, IntentSigner, VeilError};
use rand::rngs::OsRng;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

impl IntentSigner for TestSigner {
    fn public_key(&self) -> String {
        bytes_to_hex(&self.verifying_key.to_bytes())
    }

    fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, VeilError> {
        let sig: Signature = self.signing_key.sign(message);
        Ok(sig.to_bytes().to_vec())
    }
}

/// Mock server that captures the POST body and sends a response.
fn start_capturing_mock_node(
    response_body: &str,
    status_code: u16,
) -> (String, mpsc::Receiver<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let body = response_body.to_string();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buf = [0u8; 8192];
        let n = stream.read(&mut buf).unwrap();
        let received = String::from_utf8_lossy(&buf[..n]).to_string();

        // Extract body after \r\n\r\n
        let body_start = received.find("\r\n\r\n").map(|i| i + 4).unwrap_or(0);
        let post_body = received[body_start..].to_string();
        let _ = tx.send(post_body);

        let response = format!(
            "HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            status_code,
            body.len(),
            body
        );
        let _ = stream.write_all(response.as_bytes());
    });

    (format!("http://127.0.0.1:{}", port), rx)
}

fn start_mock_node(response_body: &str, status_code: u16) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let body = response_body.to_string();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buf = [0u8; 4096];
        let _ = stream.read(&mut buf);

        let response = format!(
            "HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            status_code,
            body.len(),
            body
        );
        let _ = stream.write_all(response.as_bytes());
    });

    format!("http://127.0.0.1:{}", port)
}

#[test]
fn submit_returns_tx_hash() {
    let mock_response = r#"{"tx_hash":"5UfDuX7WdrZ1WPPRpgP6x5aCqJiXxXkR9YnGxWPPRpgP6x5aCqJiXxXkR9YnGx","status":"submitted"}"#;
    let base_url = start_mock_node(mock_response, 200);
    thread::sleep(Duration::from_millis(50));

    let client = Client::new(&base_url);
    let signer = TestSigner::new();
    let intent = Intent::transfer_sol(
        "11111111111111111111111111111111".to_string(),
        1_000_000_000,
    );
    let signed = intent.sign(&signer).unwrap();
    let response = client.submit(&signed).unwrap();

    assert_eq!(
        response.tx_hash,
        "5UfDuX7WdrZ1WPPRpgP6x5aCqJiXxXkR9YnGxWPPRpgP6x5aCqJiXxXkR9YnGx"
    );
    assert_eq!(response.status, "submitted");
}

#[test]
fn submit_sends_correct_json_structure() {
    let base_url = start_mock_node(r#"{"tx_hash":"abc","status":"submitted"}"#, 200);
    thread::sleep(Duration::from_millis(50));

    let client = Client::new(&base_url);
    let signer = TestSigner::new();
    let intent = Intent::transfer_sol("11111111111111111111111111111111".to_string(), 500_000_000);
    let signed = intent.sign(&signer).unwrap();

    let response = client.submit(&signed).unwrap();
    assert_eq!(response.tx_hash, "abc");
}

#[test]
fn submit_sends_exact_json_structure() {
    let mock_response = r#"{"tx_hash":"abc","status":"submitted"}"#;
    let (base_url, rx) = start_capturing_mock_node(mock_response, 200);
    thread::sleep(Duration::from_millis(50));

    let client = Client::new(&base_url);
    let signer = TestSigner::new();
    let intent = Intent::transfer_sol("11111111111111111111111111111111".to_string(), 500_000_000);
    let signed = intent.sign(&signer).unwrap();

    let _ = client.submit(&signed).unwrap();
    let post_body = rx.recv_timeout(Duration::from_secs(2)).unwrap();

    // Parse the exact JSON sent
    let parsed: serde_json::Value = serde_json::from_str(&post_body).unwrap();

    // Must have exactly 3 keys
    assert_eq!(parsed.as_object().unwrap().len(), 3);
    assert!(parsed.get("intent").is_some(), "missing intent field");
    assert!(parsed.get("signer").is_some(), "missing signer field");
    assert!(parsed.get("signature").is_some(), "missing signature field");

    // Check encoding formats
    let signer = parsed["signer"].as_str().unwrap();
    let sig = parsed["signature"].as_str().unwrap();
    let intent_b64 = parsed["intent"].as_str().unwrap();

    // signer: 32 bytes hex = 64 chars
    assert_eq!(signer.len(), 64);
    assert!(signer.chars().all(|c| c.is_ascii_hexdigit()));

    // signature: 64 bytes = 128 hex chars
    assert_eq!(sig.len(), 128);
    assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));

    // intent: valid base64 (non-empty, only base64 chars)
    assert!(!intent_b64.is_empty());
    assert!(
        intent_b64
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
    );
}

#[test]
fn submit_returns_error_on_500() {
    let base_url = start_mock_node(r#"{"error":"internal failure"}"#, 500);
    thread::sleep(Duration::from_millis(50));

    let client = Client::new(&base_url);
    let signer = TestSigner::new();
    let intent = Intent::transfer_sol(
        "11111111111111111111111111111111".to_string(),
        1_000_000_000,
    );
    let signed = intent.sign(&signer).unwrap();

    let result = client.submit(&signed);
    assert!(result.is_err());
    match result.unwrap_err() {
        VeilError::Http(msg) => assert!(msg.contains("500")),
        _ => panic!("expected Http error"),
    }
}

#[test]
fn submit_returns_error_on_invalid_json_response() {
    let base_url = start_mock_node(r#"this is not json"#, 200);
    thread::sleep(Duration::from_millis(50));

    let client = Client::new(&base_url);
    let signer = TestSigner::new();
    let intent = Intent::transfer_sol(
        "11111111111111111111111111111111".to_string(),
        1_000_000_000,
    );
    let signed = intent.sign(&signer).unwrap();

    let result = client.submit(&signed);
    assert!(result.is_err());
}

#[test]
fn submit_returns_error_on_missing_tx_hash() {
    let base_url = start_mock_node(r#"{"status":"submitted"}"#, 200);
    thread::sleep(Duration::from_millis(50));

    let client = Client::new(&base_url);
    let signer = TestSigner::new();
    let intent = Intent::transfer_sol(
        "11111111111111111111111111111111".to_string(),
        1_000_000_000,
    );
    let signed = intent.sign(&signer).unwrap();

    let result = client.submit(&signed);
    assert!(result.is_err());
    match result.unwrap_err() {
        VeilError::Http(msg) => assert!(msg.contains("missing tx_hash")),
        _ => panic!("expected Http error about missing tx_hash"),
    }
}

#[test]
fn new_trims_trailing_slash() {
    let client = Client::new("http://localhost:3030/");
    assert_eq!(client.base_url(), "http://localhost:3030");
}

#[test]
fn new_keeps_url_without_slash() {
    let client = Client::new("http://localhost:3030");
    assert_eq!(client.base_url(), "http://localhost:3030");
}
