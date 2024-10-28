// // Import necessary traits and types from your generated canister code
// use crate::canister::token::Token; // Adjust this path based on your project structure
// use ic_cdk::api::call::CallResult; // Import for CallResult handling
//                                    // use ic_cdk::export::Principal; // Import Principal type
// use candid::Principal;
// use candid::{utils::u64_from_nat, Nat}; // Ensure you import the needed functions and types
//
// // Define the structure for CollectionMetadata
// pub struct CollectionMetadata {
//     pub weight: f64,
//     pub drive_type: String,
//     pub purchase_price: u64,
//     pub token: Principal,
//     pub documents: Vec<(String, String)>, // Assuming this is a tuple of (text, text)
//     pub supply_cap: u64,
//     pub displays: String,
//     pub seating: String,
//     pub cargo: f64,
//     pub logo: String,
//     pub name: String,
//     pub overall_height: f64,
//     pub description: String,
//     pub overall_width: f64,
//     pub track_front: f64,
//     pub collection_owner: Principal,
//     pub asset_canister: Principal,
//     pub ground_clearance: f64,
//     pub key_features: Vec<String>,
//     pub range_per_charge: f64,
//     pub track_rear: f64,
//     pub acceleration: String,
//     pub charging_speed: String,
//     pub wheels: f64,
//     pub brochure_url: String,
//     pub index: Principal,
//     pub price: u64,
//     pub battery: String,
//     pub overall_length: f64,
//     pub total_supply: u64,
//     pub symbol: String,
//     pub treasury: Principal,
//     pub images: Vec<String>,
// }
//
// // Define a type alias for convenience
// pub type CollectionMetadataResult = Result<CollectionMetadata, String>;
//
// // Example function to fetch collection metadata
// pub async fn fetch_collection_metadata(token_canister: &Token<'_>) -> CollectionMetadataResult {
//     // Call the `get_metadata` method on the token canister
//     match token_canister.get_metadata().await {
//         Ok(metadata) => {
//             // Map the returned record to the CollectionMetadata struct
//             let collection_metadata = CollectionMetadata {
//                 weight: metadata.weight,
//                 drive_type: metadata.drive_type,
//                 purchase_price: metadata.purchase_price,
//                 token: metadata.token,
//                 documents: metadata.documents,
//                 supply_cap: metadata.supply_cap,
//                 displays: metadata.displays,
//                 seating: metadata.seating,
//                 cargo: metadata.cargo,
//                 logo: metadata.logo,
//                 name: metadata.name,
//                 overall_height: metadata.overall_height,
//                 description: metadata.description,
//                 overall_width: metadata.overall_width,
//                 track_front: metadata.track_front,
//                 collection_owner: metadata.collection_owner,
//                 asset_canister: metadata.asset_canister,
//                 ground_clearance: metadata.ground_clearance,
//                 key_features: metadata.key_features,
//                 range_per_charge: metadata.range_per_charge,
//                 track_rear: metadata.track_rear,
//                 acceleration: metadata.acceleration,
//                 charging_speed: metadata.charging_speed,
//                 wheels: metadata.wheels,
//                 brochure_url: metadata.brochure_url,
//                 index: metadata.index,
//                 price: metadata.price,
//                 battery: metadata.battery,
//                 overall_length: metadata.overall_length,
//                 total_supply: metadata.total_supply,
//                 symbol: metadata.symbol,
//                 treasury: metadata.treasury,
//                 images: metadata.images,
//             };
//
//             Ok(collection_metadata)
//         }
//         Err(err) => {
//             // Handle the error, returning an error message
//             Err(format!("Failed to fetch metadata: {:?}", err))
//         }
//     }
// }
