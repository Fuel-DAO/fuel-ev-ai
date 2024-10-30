use candid::Principal;
use ic_agent::AgentError;
use serde::{Deserialize, Serialize};
use leptos::expect_context;
use crate::{canister::token::CollectionMetaData, state::canisters::Canisters};

// Structure to hold both canister IDs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionId {
   pub asset_canister: Principal,
   pub token_canister: Principal,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct CollectionData {
   pub id: CollectionId,
   pub name: String,
   pub status: String,
   pub metadata: Option<CollectionMetaData>,
}
// Modify the fetch_collections_data function to specify that it accepts an authenticated Canisters instance
pub async fn fetch_collections_data() -> Result<Vec<CollectionData>, String> {
    // Get provision canister actor
    let cans: Canisters = expect_context();

    let provision_canister = cans.provision_canister().await;

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

        let collection_meta_data = get_collection_metadata_from_token_canister(collection.token_canister).await;

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
                // Handle metadata fetch failure
                // collections.push(CollectionData {
                //     id: collection_id.clone(),
                //     name: "Unknown".to_string(),
                //     status: "Unavailable".to_string(),
                //     metadata: None,
                // });
            }
        }
    }

    Ok(collections)
}

pub async fn get_collection_metadata_from_token_canister(token_canister_id: Principal) -> Result<CollectionMetaData, String> {
    let cans: Canisters = expect_context();
    let token_canister = cans.token_canister(token_canister_id).await;

         token_canister.get_metadata().await.map_err(|e| e.to_string()) 
}