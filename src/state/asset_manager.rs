// asset_manager.rs

use candid::Principal;
use candid::{CandidType, Deserialize};
use candid::{Decode, Encode};
use ic_agent::Agent;
use serde::Serialize;

use crate::canister::asset_proxy::{AssetProxy, Result_, };


pub struct AssetManager<'a> {
   pub asset_proxy_canister: AssetProxy<'a>,
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

#[derive(CandidType, Deserialize, serde::Serialize, Debug, Clone)]
pub struct StoreArg {
  pub key: String,
  pub content: Vec<u8>,
  pub sha256: Option<Vec<u8>>,
  pub content_type: String,
  pub content_encoding: String,
}

impl<'a> AssetManager<'a> {
    pub fn new(
        upload_canister_id: Principal,
        // asset_canister_id: Principal,
        agent: &'a Agent,
    ) -> Self {
        Self {
            upload_canister_id,
            asset_proxy_canister: AssetProxy(upload_canister_id, &agent),
            // asset_canister_id,
            agent,
        }
    }

    /// Uploads a file to the upload canister and returns its URL.
    pub async fn store(&self, arg0: StoreArg) -> Result<Result_, ic_agent::AgentError> {
        let args = Encode!(&arg0)?;
        let bytes = self.agent.update(&self.upload_canister_id, "store").with_arg(args).call_and_wait().await?;
        Ok(Decode!(&bytes, Result_)?)
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
