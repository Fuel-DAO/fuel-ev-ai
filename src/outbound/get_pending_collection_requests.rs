use crate::{
    canister::provision::{AddCollectionRequestArg, ApprovalStatus, Metadata, RequestInfo},
    state::canisters::Canisters,
};
use candid::Nat;
use candid::{CandidType, Principal};
use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};
// Import the new types

/// Structure to hold both canister IDs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionId {
    pub asset_canister: Principal,
    pub token_canister: Principal,
}

/// Structure to represent collection data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionData {
    pub id: CollectionId,
    pub collection_id: Nat, // Changed from u32 to Nat

    pub name: String,
    pub status: String,
    pub metadata: Option<Metadata>,
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
    collection_id: Nat,
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

            let collection_id_struct = CollectionId {
                asset_canister: request_info
                    .asset_canister
                    .unwrap_or_else(Principal::anonymous),
                token_canister: request_info
                    .token_canister
                    .unwrap_or_else(Principal::anonymous),
            };

            let metadata = request_info
                .metadata
                .clone()
                .map(|meta: Metadata| Metadata {
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
                });

            Ok(CollectionData {
                id: collection_id_struct,
                collection_id, // Use the input collection_id
                name: request_info
                    .metadata
                    .map_or("Unknown".to_string(), |meta| meta.name),
                status: match request_info.approval_status {
                    ApprovalStatus::Approved => "Approved",
                    ApprovalStatus::Rejected => "Rejected",
                    ApprovalStatus::Pending => "Pending",
                }
                .to_string(),
                metadata,
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
