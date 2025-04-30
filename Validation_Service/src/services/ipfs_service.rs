use anyhow::Result;
use dotenv::dotenv;
use reqwest::Client;
use std::env;

pub async fn read_from_ipfs(ipfs_hash: &str) -> Result<Vec<u8>> {
    dotenv().ok(); // Load .env if available

    // Get the IPFS gateway host from env or use default
    let base_url = env::var("IPFS_HOST").unwrap_or_else(|_| "https://ipfs.io/ipfs/".to_string());

    let url = format!("{}/{}", base_url.trim_end_matches('/'), ipfs_hash);
    let client = Client::new();

    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch from IPFS: {}", response.status()));
    }

    let bytes = response.bytes().await?.to_vec();
    Ok(bytes)
}
pub async fn download_proof_file(ipfs_hash: &str) -> Result<()> {
    let bytes = read_from_ipfs(ipfs_hash).await?;
    std::fs::write("example-json.presentation.tlsn", bytes)?;
    Ok(())
}
