use reqwest::Error;
use std::str::FromStr;
use crate::services::{ipfs_service, oracle_service};
use tlsn_examples::ExampleType;

pub async fn validate(proof_of_task: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Convert the proofOfTask string into a float
    let task_result = match f64::from_str(proof_of_task) {
        Ok(val) => val,
        Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid proofOfTask value"))),
    };

    ipfs_service::download_proof_file(proof_of_task).await;
    println!("Validating presentation");
    let example_type = ExampleType::Json;

    match crate::services::verify::verify_presentation(&example_type).await {
        Ok(is_valid) => {
            println!("Presentation validated: {}", is_valid);
            Ok(is_valid)
        }
        Err(e) => {
            println!("❌ Error during presentation verification: {}", e);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Presentation verification failed: {}", e))))
        }
    }
    
}