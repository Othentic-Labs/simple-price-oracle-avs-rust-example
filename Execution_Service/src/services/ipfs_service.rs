use anyhow::Result;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use reqwest::{Client, multipart};
use dotenv::dotenv;
use std::env;

pub async fn upload_to_pinata(file_path: &str) -> Result<String> {
    dotenv().ok(); // load .env if present

    let api_key = env::var("PINATA_API_KEY")?;
    let secret_key = env::var("PINATA_SECRET_API_KEY")?;

    let client = Client::new();

    // Read file contents
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Create multipart form
    let file_part = multipart::Part::bytes(buffer)
        .file_name(Path::new(file_path).file_name().unwrap().to_string_lossy().into_owned())
        .mime_str("application/octet-stream")?;

    let form = multipart::Form::new().part("file", file_part);

    // Send request
    let response = client
        .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
        .header("pinata_api_key", api_key)
        .header("pinata_secret_api_key", secret_key)
        .multipart(form)
        .send()
        .await?;

    // Parse response
    let json: serde_json::Value = response.json().await?;
    let hash = json["IpfsHash"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse IPFS hash"))?;

    Ok(hash.to_string())
}
