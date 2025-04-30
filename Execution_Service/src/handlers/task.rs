use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::services::dal_service; // Import from services/price.rs
use crate::services::oracle_service;  // Import from services/task.rs
use crate::services::notary_service;
use crate::services::ipfs_service;

#[derive(Deserialize)]
pub struct ExecuteTaskPayload {
    pub taskDefinitionId: Option<i32>, // optional in case it's not included in the request body
}

#[derive(Serialize)]
struct CustomResponse {
    status: String,
    data: HashMap<String, serde_json::Value>,
}

pub async fn execute_task(payload: web::Json<ExecuteTaskPayload>) -> impl Responder {
    println!("Executing Task");

    // Default taskDefinitionId to 0 if not provided
    let task_definition_id = payload.taskDefinitionId.unwrap_or(0);
    println!("task_definition_id: {}", task_definition_id);

    notary_service::run().await;

    // // Upload the generated proof file to IPFS
    // let ipfs_hash = match ipfs_service::upload_proof_file().await {
    //     Ok(hash) => hash,
    //     Err(err) => {
    //         eprintln!("Error uploading to IPFS: {}", err);
    //         return HttpResponse::ServiceUnavailable().json("IPFS upload error");
    //     }
    // };
    // dal_service::send_task(ipfs_hash.clone(), task_definition_id).await;

    match ipfs_service::upload_to_pinata("example-json.presentation.tlsn").await {
        Ok(ipfs_hash) => {
            dal_service::send_task(ipfs_hash.clone(), task_definition_id).await;
            HttpResponse::Ok().json("Task executed successfully".to_string())
        }
        Err(err) => {
            eprintln!("Error uploading to IPFS: {}", err);
            HttpResponse::ServiceUnavailable().json("IPFS upload error")
        }
    }
}
