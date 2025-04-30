# 🔐 TLSNotary AVS Example
This project demonstrates how to build a TLSNotary-based attestation flow with an AVS using the Othentic Stack.


## Table of Contents

1. [Overview](#overview)
2. [Architecture](#usage)
3. [Prerequisites](#prerequisites)
4. [Installation](#installation)
5. [Usage](#usage)
6. [Usage with Docker](#usage)

## Overview
This project demonstrates a complete implementation of a TLSNotary-based attestation flow, integrated with an AVS. It includes:

- Notary Server – Verifies TLS sessions and produces notarized transcripts

- Fixture Server – Mock TLS endpoint used by the prover

- Prover (Execution Service) – Establishes a TLS session, creates a .tlsn proof, and uploads it to IPFS

- Verifier (Validation Service) – Downloads the proof and verifies the attestation before approving the task


### Features

- **Containerised deployment:** Simplifies deployment and scaling.

## Architecture

![Price oracle sample](https://github.com/user-attachments/assets/03d544eb-d9c3-44a7-9712-531220c94f7e)

The Performer node executes tasks using the Task Execution Service and sends the results to the p2p network.

Attester Nodes validate task execution through the Validation Service. Based on the Validation Service's response, attesters sign the tasks. In this AVS:

1. The prover creates a notarized TLS transcript (.presentation.tlsn)

2. It uploads the transcript to IPFS (via Pinata)

3. The verifier downloads the file and verifies the signature and integrity

4. Based on this, the AVS either accepts or rejects the task


## Prerequisites

- Rust (v 1.23 )
- Foundry
- [Docker](https://docs.docker.com/engine/install/)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Othentic-Labs/avs-examples.git
   cd avs-examples/simple-price-oracle-avs-rust-example
   ```

2. Install Othentic CLI:

   ```bash
   npm i -g @othentic/othentic-cli
   ```

## Usage

1. Start the Notary Server
```bash
cd Execution_Service/tlsn-src/crates/notary/server
cargo run -r -- --tls-enabled false
```
The server will start listening on `0.0.0.0:7047`.

2. Start the Fixture Server
```bash
cd Execution_Service/tlsn-src/crates/server-fixture/server
PORT=4000 cargo run --release
```
Simulates an HTTPS server used by the prover to establish TLS sessions.

3. Run the Execution Service (Prover)
```bash
cd Execution_Service/
cargo build
# Use this command to run the prover directly
# RUST_LOG=debug SERVER_PORT=4000 cargo run --bin attestation-prove 
cargo run --bin Execution_Service
```
The Execution service will start on port 4003.

4. Trigger task execution with following command

```bash
curl -X POST http://localhost:4003/task/execute -H "Content-Type: application/json" -d "{}"
```

It will:
- Connect to the notary server
- Establish a TLS connection
- Create a notarized TLS transcript file (example-json.presentation.tlsn)
- Upload the file to the IPFS and Return the IPFS hash as Proof of Task

5. Run the Validation Service (Verifier)
```bash
cd Validation_Service/
cargo build
# Use this command to verify directly
# cargo run --bin attestation-verify
cargo run --bin Validation_Service
```

6. Validate the Proof of Task
Replace <proofOfTask> with the actual hash returned from the Execution Service:

```bash
curl -X POST http://localhost:4002/task/validate -H "Content-Type: application/json" -d '{"proofOfTask":"QmaLrbmC5W3Guz22htHZLMD31voWNLJnv7kjirm7Sb8Np9"}'
```


## Usage with Docker

Follow the steps in the official documentation's [Quickstart](https://docs.othentic.xyz/main/avs-framework/quick-start#steps) Guide for setup and deployment.

If you already have all the information required to run the AVS, simply copy the .env file into your project directory and then run:
```bash
docker-compose up --build
```

### Next
Modify the different configurations, tailor the task execution logic as per your use case, and run the AVS.

Happy Building! 🚀

