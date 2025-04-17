# TLSNotary Implementation

This project implements a TLSNotary attestation system using the TLSNotary protocol. It consists of two main components:
1. A notary server that verifies TLS connections
2. A prover that creates attestations for TLS connections

## Prerequisites

- Rust and Cargo installed
- Access to the TLSNotary source code (tlsn-src directory)

## Setup

1. Clone the repository and navigate to the project directory:
```bash
cd Execution_Service
```

2. Build the project:
```bash
cargo build
```

## Usage

### Starting the Notary Server

./generate_notary_key.sh

go to 
cd Execution_Service/tlsn-src/crates/notary/server
cargo run -r -- --tls-enabled false

1. In one terminal, start the notary server:
```bash
cargo run --bin notary-server
```
The server will start listening on `127.0.0.1:4000`.

### Creating TLS Attestations

2. In another terminal, run the prover to create an attestation:
```bash
cargo run --bin tls-prove
```

The prover will:
- Connect to the notary server
- Establish a TLS connection
- Create an attestation of the connection
- Finalize the proof

## Components

- `notary-server`: The verifier component that validates TLS connections
- `tls-prove`: The prover component that creates attestations

## Dependencies

The project uses the following main dependencies:
- tlsn-prover
- tlsn-core
- tlsn-common
- notary-server

All dependencies are sourced from the local tlsn-src directory. 