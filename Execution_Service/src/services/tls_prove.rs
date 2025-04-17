use tlsn_prover::{Prover, ProverConfig};
use tlsn_core::transcript::Idx;
use tlsn_common::config::ProtocolConfig;
use anyhow::Result;
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;
use async_tungstenite::tokio::connect_async;
use async_tungstenite::WebSocketStream;
use tokio_tungstenite::WebSocketStream as TokioWebSocketStream;
use tokio::net::TcpStream as TokioTcpStream;
use futures_util::StreamExt;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use notary_client::{Accepted, NotarizationRequest, NotaryClient};
use std::env;
// use tls_server_fixture::SERVER_DOMAIN;

#[tokio::main]
async fn main() -> Result<()> {
    println!("[1] Starting TLSNotary attestation...");

    let notary_host: String = env::var("NOTARY_HOST").unwrap_or("127.0.0.1".into());
    let notary_port: u16 = env::var("NOTARY_PORT")
        .map(|port| port.parse().expect("port should be valid integer"))
        .unwrap_or(7047);

    println!("[2] Building notary client...");
    let notary_client = NotaryClient::builder()
        .host(notary_host)
        .port(notary_port)
        .enable_tls(false)
        .build()
        .unwrap();

    println!("[3] Creating notarization request...");
    let notarization_request = NotarizationRequest::builder()
        .max_sent_data(1024)
        .max_recv_data( 1024)
        .build()?;

    println!("[4] Requesting notarization...");
    let Accepted {
        io: notary_connection,
        id: _session_id,
        ..
    } = notary_client
        .request_notarization(notarization_request)
        .await
        .expect("Could not connect to notary. Make sure it is running.");

    println!("[5] Creating protocol config...");
    let protocol_config = ProtocolConfig::builder()
        .max_sent_data(1024)
        .max_recv_data( 1024)
        .build()
        .expect("Failed to build protocol config");

    println!("[6] Creating prover config...");
    let config = ProverConfig::builder()
        .server_name("example.com")
        .protocol_config(protocol_config)
        .build()
        .expect("Failed to build prover config");

    println!("[7] Creating prover instance...");
    let prover = Prover::new(config);

    println!("[8] Setting up prover with notary...");
    let prover = prover.setup(notary_connection.compat()).await?;

    println!("[9] Connecting to target server...");
    let target_socket = TcpStream::connect("example.com:443").await?;
    println!("[10] Connected to target server, setting up TLS...");
    let (mpc_tls_connection, prover_fut) = prover.connect(target_socket.compat()).await?;

    println!("[11] Waiting for connection to close...");
    let prover = prover_fut.await?;

    println!("[12] Starting proof...");
    let mut prover = prover.start_prove();

    println!("[13] Getting transcript...");
    let transcript = prover.transcript();

    let sent_len = transcript.sent().len();
    let recv_len = transcript.received().len();
    
    let sent_idx = Idx::new(0..sent_len);
    let recv_idx = Idx::new(0..recv_len);
    
    println!("[14] Proving transcript...");
    prover.prove_transcript(sent_idx, recv_idx).await?;

    println!("[15] Finalizing proof...");
    prover.finalize().await?;

    println!("✅ Attestation complete");
    Ok(())
}
