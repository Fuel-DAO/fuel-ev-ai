use crate::canister::provision::{self, Result1, Result2};
use crate::state::canisters::Canisters;
use candid::Nat;
use candid::{CandidType, Principal};
use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};
// Import the new types
use ic_agent::AgentError;

/// Structure to hold both canister IDs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionId {
    pub asset_canister: Principal,
    pub token_canister: Principal,
}

/// Structure to represent collection data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionData {
    // pub id: CollectionId,
    pub collection_id: u64, // Changed from u32 to Nat

    pub name: String,
    pub status: String,
    pub metadata: Option<provision::CollectionRequest>,
}

pub async fn fetch_pending_requests_data(
    canisters: &Canisters,
) -> Result<Vec<CollectionData>, String> {
    let provision_canister = canisters.provision_canister().await;

    // Fetch pending requests
    let pending_request_ids = provision_canister
        .get_pending_requests()
        .await
        .map_err(|e| format!("Failed to fetch pending request IDs: {:?}", e))?;
    log!("Pending request IDs: {:?}", pending_request_ids);

    let mut collections = Vec::new();

    for request_id in pending_request_ids {
        // Use `get_request_info_by_id` to fetch request info for each ID
        match get_request_info_by_id(canisters, request_id.clone()).await {
            Ok(collection_data) => {
                log!("Fetched collection data: {:?}", collection_data);
                collections.push(collection_data);
            }
            Err(e) => {
                log!(
                    "Failed to fetch collection data for request_id {}: {:?}",
                    request_id,
                    e
                );
            }
        }
    }

    log!("Final collections: {:?}", collections);
    Ok(collections)
}
pub async fn get_request_info_by_id(
    canisters: &Canisters,
    collection_id: u64,
) -> Result<CollectionData, String> {
    let provision_canister = canisters.provision_canister().await;

    // Fetch request info for the given collection_id
    match provision_canister
        .get_request_info(collection_id.clone())
        .await
    {
        Ok(Some(request_info)) => {
            log!(
                "Request info for collection_id {:?}: {:?}",
                collection_id,
                request_info
            );

            // let collection_id_struct = CollectionId {
            //     asset_canister: request_info
            //         .asset_canister
            //         .unwrap_or_else(Principal::anonymous),
            //     token_canister: request_info
            //         .token_canister
            //         .unwrap_or_else(Principal::anonymous),
            // };

            let metadata = request_info
                .clone()
               /*  .map(|meta: Metadata| Metadata {
                    weight: meta.weight,
                    drive_type: meta.drive_type,
                    purchase_price: meta.purchase_price,
                    token: meta.token,
                    documents: meta.documents,
                    supply_cap: meta.supply_cap,
                    displays: meta.displays,
                    seating: meta.seating,
                    cargo: meta.cargo,
                    logo: meta.logo,
                    name: meta.name.clone(),
                    overall_height: meta.overall_height,
                    description: meta.description,
                    overall_width: meta.overall_width,
                    track_front: meta.track_front,
                    ground_clearance: meta.ground_clearance,
                    key_features: meta.key_features,
                    range_per_charge: meta.range_per_charge,
                    track_rear: meta.track_rear,
                    acceleration: meta.acceleration,
                    charging_speed: meta.charging_speed,
                    wheels: meta.wheels,
                    brochure_url: meta.brochure_url,
                    index: meta.index,
                    price: meta.price,
                    battery: meta.battery,
                    overall_length: meta.overall_length,
                    symbol: meta.symbol,
                    treasury: meta.treasury,
                    images: meta.images,
                }) */;

            Ok(CollectionData {
                collection_id, // Use the input collection_id
                name: request_info.name
                    ,
                status: "Pending"
                .to_string(),
                metadata:  Some(metadata),
            })
        }
        Ok(None) => {
            let error_msg = format!(
                "No request info found for collection_id: {:?}",
                collection_id
            );
            log!("{}", error_msg);
            Err(error_msg)
        }
        Err(e) => {
            let error_msg = format!(
                "Failed to fetch request info for collection_id {}: {:?}",
                collection_id, e
            );
            log!("{}", error_msg);
            Err(error_msg)
        }
    }
}

pub async fn approve_request(
    canisters: &Canisters,
    collection_id: u64,
) -> Result<(u64, Principal, Principal), String> {
    let provision_canister = canisters.provision_canister().await;

    // Call the approve_request method on the canister
    let response_result: Result<Result1, AgentError> = provision_canister
        .approve_request(collection_id.clone())
        .await;

    // Handle the response
    match response_result {
        Ok(response) => match response {
            Result1::Ok( provision::ListCollection{
                id,
                token_canister,
                asset_canister,
            } )=> {
                log!(
                    "Request approved: id={}, token_canister={}, asset_canister={}",
                    id,
                    token_canister,
                    asset_canister
                );
                Ok((id, token_canister, asset_canister))
            }
            Result1::Err(err_msg) => {
                let error_msg = format!("approve_request failed: {}", err_msg);
                log!("{}", error_msg);
                Err(error_msg)
            }
        },
        Err(agent_error) => {
            let error_msg = format!("Agent error during approve_request: {:?}", agent_error);
            log!("{}", error_msg);
            Err(error_msg)
        }
    }
}
pub async fn reject_request(canisters: &Canisters, collection_id: u64) -> Result<bool, String> {
    let provision_canister = canisters.provision_canister().await;

    // Call the reject_request method on the canister
    let response:Result2 = provision_canister
        .reject_request(collection_id.clone())
        .await
        .map_err(|e| format!("Failed to call reject_request: {:?}", e))?;

    match response {
        Result2::Ok(success) => {
            if success {
                log!(
                    "Request rejected successfully for collection_id: {}",
                    collection_id
                );
                Ok(true)
            } else {
                let error_msg = "reject_request returned Ok(false)".to_string();
                log!("{}", error_msg);
                Err(error_msg)
            }
        }
        Result2::Err(err_msg) => {
            let error_msg = format!("reject_request failed: {}", err_msg);
            log!("{}", error_msg);
            Err(error_msg)
        }
    }
}
