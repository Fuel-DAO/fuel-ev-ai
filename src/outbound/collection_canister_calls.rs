use crate::canister::token;
use crate::canister::token::SaleStatus;
use crate::state::canisters::Canisters;

use candid::Nat;
use candid::Principal;
use serde::{Deserialize, Serialize};
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
    pub metadata: Option<token::GetMetadataRet>,
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
        let sale_status =
            canisters.token_canister(collection.token_canister).await.get_sale_status().await.ok();

        let status = sale_status.map(|f| match f {
            SaleStatus::Live => "Open",
            _ => "Closed",
        } ).unwrap_or("Open").to_string() ;

        match collection_meta_data {
            Ok(metadata)  => {
                match metadata {
                    token::Result4::Ok(metadata) => {
                        collections.push(CollectionData {
                            id: collection_id.clone(),
                            name: metadata.name.clone(),
                            status: status, // Adjust as needed based on actual status
                            metadata: Some(metadata),
                        });
                    },
                    token::Result4::Err(e) =>  {
                        log::error!(
                            "Failed to fetch metadata for collection {:?}: {}",
                            collection_id,
                            e
                        );
                    },
                }

                
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
) -> Result<token::Result4, String>{
    let token_canister = canisters.token_canister(token_canister_id,).await;

    token_canister
        .get_metadata()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_total_booked_tokens(
    canisters: &Canisters,
    token_canister_id: Principal,
) -> Result<Nat, String> {
    let token_canister = canisters.token_canister(token_canister_id, ).await;

    token_canister
        .get_total_booked_tokens()
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_annonymous_principal(
    canisters: &Canisters,
    token_canister_id: Principal,
    principal: Principal
) -> Result<(), String> {
    let token_canister = canisters.token_canister(token_canister_id, ).await;

    let res = token_canister
        .update_annonymous_investor(principal)
        .await
        .map_err(|e| e.to_string())?;

    match res {
        token::Result5::Ok => Ok(()),
        token::Result5::Err(e) => Err(e),
    }

}

pub async fn get_sale_status(
    canisters: &Canisters,
    token_canister_id: Principal,
) -> Result<SaleStatus, String> {
    let token_canister = canisters.token_canister(token_canister_id,).await;

    token_canister
        .get_sale_status()
        .await
        .map_err(|e| e.to_string())
}
