use crate::state::canisters::Canisters;
use candid::Principal;

pub async fn is_admin(canisters: &Canisters, principal: Option<Principal>) -> Result<bool, String> {
    let provision_canister = canisters.provision_canister().await;
    provision_canister
        .is_admin(principal)
        .await
        .map_err(|e| format!("Failed to check admin status: {:?}", e))
}
