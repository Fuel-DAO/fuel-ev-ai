use crate::{canister::token::CollectionMetaData, state::canisters::Canisters};
use candid::Principal;
use ic_agent::AgentError;
use leptos::expect_context;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
// Structure to hold both canister IDs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionId {
    pub asset_canister: Principal,
    pub token_canister: Principal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionData {
    pub id: CollectionId,
    pub name: String,
    pub status: String,
    pub metadata: Option<CollectionMetaData>,
}

// Modify the fetch_collections_data function to accept a reference to Canisters
pub async fn fetch_collections_data(canisters: &Canisters) -> Result<Vec<CollectionData>, String> {
    // Get provision canister actor
    let provision_canister = canisters.provision_canister().await;

    // Fetch collections from provision canister
    let collections_list = provision_canister
        .list_collections()
        .await
        .map_err(|e| format!("Failed to fetch collections: {:?}", e))?;

    // Fetch metadata for each collection using the token canister
    let mut collections = vec![];
    for collection in collections_list {
        let collection_id = CollectionId {
            asset_canister: collection.asset_canister,
            token_canister: collection.token_canister,
        };

        let collection_meta_data =
            get_collection_metadata_from_token_canister(canisters, collection.token_canister).await;

        match collection_meta_data {
            Ok(metadata) => {
                collections.push(CollectionData {
                    id: collection_id.clone(),
                    name: metadata.name.clone(),
                    status: "Available".to_string(), // Adjust as needed based on actual status
                    metadata: Some(metadata),
                });
            }
            Err(e) => {
                log::error!(
                    "Failed to fetch metadata for collection {:?}: {}",
                    collection_id,
                    e
                );
                // Optionally, you can push a CollectionData with partial information
            }
        }
    }

    Ok(collections)
}

// Modify get_collection_metadata_from_token_canister similarly
pub async fn get_collection_metadata_from_token_canister(
    canisters: &Canisters,
    token_canister_id: Principal,
) -> Result<CollectionMetaData, String> {
    let token_canister = canisters.token_canister(token_canister_id).await;

    token_canister
        .get_metadata()
        .await
        .map_err(|e| e.to_string())
}
