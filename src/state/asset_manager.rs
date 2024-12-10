// asset_manager.rs

use candid::Principal;
use candid::{CandidType, Deserialize};
use candid::{Decode, Encode};
use ic_agent::Agent;
use serde::Serialize;

pub struct AssetManager<'a> {
    upload_canister_id: Principal,
    // asset_canister_id: Principal,
    agent: &'a Agent,
}

#[derive(Serialize, Deserialize, CandidType)]
struct StoreResponse {
    success: bool,
    key: Option<String>,
    error: Option<String>,
}

impl<'a> AssetManager<'a> {
    pub fn new(
        upload_canister_id: Principal,
        // asset_canister_id: Principal,
        agent: &'a Agent,
    ) -> Self {
        Self {
            upload_canister_id,
            // asset_canister_id,
            agent,
        }
    }

    /// Uploads a file to the upload canister and returns its URL.
    pub async fn store(&self, file_data: Vec<u8>, file_name: String) -> Result<String, String> {
        // Encode the arguments using Candid
        let args =
            Encode!(&file_name, &file_data).map_err(|e| format!("Candid encode error: {}", e))?;

        // Make an update call to the 'store' method on the upload canister
        let response = self
            .agent
            .update(&self.upload_canister_id, "store")
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| format!("Agent update call failed: {}", e))?;

        // Decode the response to extract the URL
        let store_response: StoreResponse = Decode!(response.as_slice(), StoreResponse)
            .map_err(|e| format!("Candid decode error: {}", e))?;

        if store_response.success {
            if let Some(key) = store_response.key {
                Ok(key)
            } else {
                Err("Store succeeded but no key returned.".to_string())
            }
        } else {
            if let Some(error) = store_response.error {
                Err(error)
            } else {
                Err("Store failed without error message.".to_string())
            }
        }
    }
    pub async fn delete(&self, url: String) -> Result<(), String> {
        // Encode the URL using Candid
        let args = Encode!(&url).map_err(|e| format!("Candid encode error: {}", e))?;

        // Make an update call to the 'delete' method on the asset canister
        self.agent
            .update(&self.upload_canister_id, "delete")
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| format!("Agent update call failed: {}", e))?;

        Ok(())
    }
}
