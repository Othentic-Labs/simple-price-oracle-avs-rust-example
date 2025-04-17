use notary_server::{Settings, CliFields, NotaryServerProperties, NotaryServerError};
use tlsn_common::config::ProtocolConfig;
use std::path::PathBuf;
use anyhow::{Result, Context};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 Starting TLSNotary notary server...");

    // Create CLI fields with default config file
    let cli_fields = CliFields {
        config_file: "config/default.yaml".to_string(),
        port: Some(4000),
        tls_enabled: Some(false),
        log_level: Some("info".to_string()),
    };

    // Create settings from CLI fields
    let settings = Settings::new(&cli_fields)
        .map_err(|e| anyhow::anyhow!("Failed to create settings: {}", e))?;
    
    // Run the server with the settings
    notary_server::run_server(&settings.config)
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
} 