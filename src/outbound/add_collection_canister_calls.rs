use crate::{canister::provision::CollectionRequest, state::canisters::Canisters};
use candid::CandidType;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Document {
    pub title: String,
    pub url: String,
}
pub async fn add_collection(
    canisters: &Canisters,
    collection_data: CollectionRequest,
) -> Result<(), String> {
    let auth_service = canisters.auth_service.borrow();
    if !auth_service.is_authenticated() {
        log!(" User is not authenticated. Please log in first.");
        return Err("User is not authenticated. Please log in first.".to_string());
    } else {
        log!(" User is authenticated.");
    }
    log!("collection_data: {:?}", collection_data);

    let provision_canister = canisters.provision_canister().await;
    match provision_canister
        .add_collection_request(collection_data)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.to_string().contains("TODO needs error info") {
                log!("Error: {:?}", e);
                Err("The provision canister encountered an unhandled error. Please check the canister code.".to_string())
            } else {
                Err(format!("Failed to add collection request: {:?}", e))
            }
        }
    }
}
