// use candid::Principal;
// use ic_agent::{Agent, HttpAgent, HttpAgentOptions, Identity};
// use ic_utils::interfaces::management_canister::builders::CanisterInstall;
//
// // Import your canister interfaces
// use crate::declarations::provision::{self, _SERVICE as ProvisionService};
// use crate::declarations::token::{self, _SERVICE as TokenService};
//
// // Constants
// const HOST: &str = if cfg!(debug_assertions) {
//     "http://localhost:4943"
// } else {
//     "https://ic0.app"
// };
//
// pub fn agent_options(identity: Option<Box<dyn Identity>>) -> HttpAgentOptions {
//     HttpAgentOptions {
//         identity,
//         ..Default::default()
//     }
// }
//
// pub async fn create_agent(identity: Option<Box<dyn Identity>>) -> Result<Agent, String> {
//     let agent = HttpAgent::builder()
//         .with_url(HOST)
//         .with_options(agent_options(identity))
//         .build()
//         .map_err(|e| format!("Failed to create agent: {}", e))?;
//
//     if cfg!(debug_assertions) {
//         agent
//             .fetch_root_key()
//             .await
//             .map_err(|e| format!("Failed to fetch root key: {}", e))?;
//     }
//
//     Ok(agent)
// }
//
// pub async fn nft_canister(canister_id: Principal, agent: &Agent) -> Result<TokenService, String> {
//     token::create_actor(canister_id, agent)
//         .await
//         .map_err(|e| format!("Failed to create NFT canister actor: {}", e))
// }
//
// pub async fn provision_canister(
//     canister_id: Principal,
//     agent: &Agent,
// ) -> Result<ProvisionService, String> {
//     provision::create_actor(canister_id, agent)
//         .await
//         .map_err(|e| format!("Failed to create provision canister actor: {}", e))
// }
//
// // Asset path function
// pub fn asset_path(canister_id: &str, image_path: &str) -> String {
//     format!("https://{}.icp0.io{}", canister_id, image_path)
// }
