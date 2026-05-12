# Veil SDK

[![CI](https://github.com/Interveil/veil-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/Interveil/veil-sdk/actions/workflows/ci.yml)

A portable execution language for intent-based Solana transactions. Veil SDK sits between a wallet and a node — it builds and serializes intents. It is **not** a wallet and **not** a node.

## Installation

```toml
[dependencies]
veil-sdk = "0.1"
```

## Quick Start

```rust
use veil_sdk::{Client, Intent, Signer};

// 1. Build an intent in one line
let intent = Intent::transfer_sol(
    "amount".to_string(),
    1_000_000_000, // 1 SOL in lamports
);

// 2. Validate before signing
intent.validate()?;

// 3. Sign with any wallet implementing the Signer trait
let signed = intent.sign(&wallet)?;

// 4. Submit to the Interveil node
let client = Client::new("http://localhost:3030");
let response = client.submit(&signed)?;
println!("tx_hash: {}", response.tx_hash);
```

## API Overview

| Type | Purpose |
|---|---|
| `Intent` | Encodes a transaction (version, chain, nonce, payload) |
| `IntentPayload::TransferSol` | Transfer lamports to a Solana address |
| `Signer` trait | Wallet abstraction — implement `public_key()` and `sign()` |
| `SignedIntent` | Intent + signature, serializable to JSON |
| `Client` | HTTP client for submitting to an Interveil node |
| `SubmitResponse` | Node response with `tx_hash` and `status` |
| `Chain::Solana` | Chain identifier |
| `VeilError` | Unified error type (Serialization, Signing, Http, InvalidIntent) |

## Serialization

- `Intent::to_bytes()` uses bincode for deterministic binary serialization
- `SignedIntent::to_json()` produces `{"intent": "<base64>", "pubkey": "<hex>", "signature": "<hex>"}`
- Signing flow: `intent → to_bytes() → blake3 hash → ed25519 sign`

## Development

```bash
# Build
cargo build

# Run all tests
cargo test

# Lint
cargo clippy --all-targets -- -D warnings

# Format
cargo fmt --all -- --check
```
