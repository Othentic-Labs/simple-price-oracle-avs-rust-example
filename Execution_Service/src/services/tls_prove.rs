use tlsn_prover::{Prover, ProverConfig};
use tlsn_core::transcript::Idx;
use tlsn_common::config::ProtocolConfig;
use tlsn_examples::{get_crypto_provider_with_server_fixture, ExampleType};
use anyhow::Result;
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;
use futures_util::{AsyncWriteExt, AsyncReadExt};
use notary_client::{Accepted, NotarizationRequest, NotaryClient};
use std::env;
use tokio::time::{timeout, Duration};

use bincode::{DefaultOptions, Options};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use hyper_util::rt::TokioIo;
use hyper::{body::Bytes, Request, StatusCode};
use http_body_util::Empty;
use hyper::Uri;
use clap::Parser;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";

#[derive(Parser)]
struct Args {
    /// What data to notarize
    #[clap(default_value_t = ExampleType::Json, ignore_case = true)]
    example_type: ExampleType,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("[1] Starting Prover...");
    let args = Args::parse();

    let notary_host = env::var("NOTARY_HOST").unwrap_or("127.0.0.1".into());
    let notary_port: u16 = env::var("NOTARY_PORT")
        .map(|port| port.parse().unwrap())
        .unwrap_or(4001);

    let server_host = env::var("SERVER_HOST").unwrap_or("127.0.0.1".into());
    let server_port: u16 = env::var("SERVER_PORT")
        .map(|port| port.parse().unwrap())
        .unwrap_or(4000);

    let notary_client = NotaryClient::builder()
        .host(notary_host)
        .port(notary_port)
        .enable_tls(false)
        .build()?;

    let notarization_request = NotarizationRequest::builder()
        .max_sent_data(4096)
        .max_recv_data(4096)
        .build()?;

    let Accepted { io: notary_connection, .. } = notary_client
        .request_notarization(notarization_request)
        .await?;

    let protocol_config = ProtocolConfig::builder()
        .max_sent_data(1024)
        .max_recv_data(1024)
        .build()?;

    let prover_config = ProverConfig::builder()
        .server_name("test-server.io") // MUST match server fixture!
        .protocol_config(protocol_config)
        .crypto_provider(get_crypto_provider_with_server_fixture()) // VERY important
        .build()?;

    let prover = Prover::new(prover_config)
        .setup(notary_connection.compat())
        .await?;

    println!("[2] Connecting to fixture server...");
    let server_socket = TcpStream::connect((server_host.as_str(), server_port)).await?;

    println!("[3] Setting up MPC TLS connection...");
    let (mpc_tls_connection, prover_fut) = prover.connect(server_socket.compat()).await?;

    // Wrap the TLS stream to expose AsyncRead
    // let mut io = TokioIo::new(mpc_tls_connection);

    let mpc_tls_connection = TokioIo::new(mpc_tls_connection.compat());

    // Perform HTTP1 handshake
    let (mut request_sender, connection) =
        hyper::client::conn::http1::handshake(mpc_tls_connection).await?;
    // Spawn the connection driver (hyper manages read/write in background)
    tokio::spawn(connection);
    println!("✅ Hyper HTTP connection driver spawned");


    let (uri, extra_headers) = match args.example_type {
        ExampleType::Json => ("/formats/json", vec![]),
        ExampleType::Html => ("/formats/html", vec![]),
        ExampleType::Authenticated => ("/protected", vec![("Authorization", "random_auth_token")]),
    };
    let uri = Uri::builder()
    .path_and_query("/formats/json")
    .build()?;

    // Build the HTTP request
    let request_builder = Request::builder()
    .uri(uri)
    .header("Host", "test-server.io")
    .header("Accept", "*/*")
    // Using "identity" instructs the Server not to use compression for its HTTP response.
    // TLSNotary tooling does not support compression.
    .header("Accept-Encoding", "identity")
    .header("Connection", "close")
    .header("User-Agent", USER_AGENT);
    let mut request_builder = request_builder;

for (key, value) in extra_headers {
    request_builder = request_builder.header(key, value);
}
let request = request_builder.body(Empty::<Bytes>::new())?;
println!("✅ HTTP request built: {:?}", request);

println!("[4] Sending HTTP request...");
    // Send the request
    let response = request_sender.send_request(request).await.unwrap_or_else(|e| {
        println!("❌ HTTP request failed: {:?}", e);
        panic!("Exiting because send_request failed");
    });
    // let response = request_sender.send_request(request).await?;
    
    println!("✅ Got HTTP response: {}", response.status());
    




    // let mut stream = mpc_tls_connection; // This is TokioIo wrapped

    // println!("[4] Sending raw HTTP request...");
    // stream.write_all(b"GET /formats/json HTTP/1.1\r\nHost: test-server.io\r\nAccept-Encoding: identity\r\nConnection: close\r\nUser-Agent: RustClient\r\n\r\n").await?;
    // stream.flush().await?;
    
    // println!("✅ HTTP request sent! Now reading response...");
    
    // let mut buf = vec![0u8; 4096];
    // let read_result = timeout(Duration::from_secs(5), stream.read(&mut buf)).await;

    // match read_result {
    //     Ok(Ok(n)) => {
    //         println!("✅ Received {} bytes from server:", n);
    //         println!("{}", String::from_utf8_lossy(&buf[..n]));
    //     }
    //     Ok(Err(e)) => {
    //         println!("❌ Read failed: {:?}", e);
    //     }
    //     Err(_) => {
    //         println!("⏰ Timeout: no data received in 5 seconds");
    //     }
    // }
    // println!("{}", String::from_utf8_lossy(&buf[..n]));











    println!("[5] Finalizing TLS connection...");
    let prover = prover_fut.await?;

    println!("[6] Starting notarization...");
    let mut prover = prover.start_prove();

    let transcript = prover.transcript();
    let sent_len = transcript.sent().len();
    let recv_len = transcript.received().len();
    let sent_idx = Idx::new(0..sent_len);
    let recv_idx = Idx::new(0..recv_len);

    prover.prove_transcript(sent_idx, recv_idx).await?;

    println!("[7] Finalizing attestation...");
    let attestation = prover.finalize().await?;

    tokio::fs::write("attestation.tlsn", DefaultOptions::new().serialize(&attestation)?).await?;

    println!("✅ Attestation saved successfully!");

    Ok(())
}
