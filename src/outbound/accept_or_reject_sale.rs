use crate::{
    canister::token::{self, SaleStatus},
    state::canisters::Canisters,
};
use candid::Principal;
use leptos::logging::log;

// Existing imports...

// Updated accept_sale function
pub async fn accept_sale(
    canisters: &Canisters,
    token_canister_id: Principal,
) -> Result<bool, String> {
    let token_canister = canisters.token_canister(token_canister_id).await;
    // Call the accept_sale method on the canister
    let response_result: Result<token::Result_, String> = token_canister
        .accept_sale()
        .await
        .map_err(|e| format!("Failed to call accept_sale: {}", e));

    // Handle the response
    match response_result {
        Ok(response) => match response {
            token::Result_::Ok(success) => {
                if success {
                    log!("Sale accepted successfully.");
                    Ok(true)
                } else {
                    let error_msg = "accept_sale returned Ok(false)".to_string();
                    log!("{}", error_msg);
                    Err(error_msg)
                }
            }
            token::Result_::Err(err_msg) => {
                let error_msg = format!("accept_sale failed: {}", err_msg);
                log!("{}", error_msg);
                Err(error_msg)
            }
        },
        Err(e) => Err(e),
    }
}

// Updated reject_sale function
pub async fn reject_sale(
    canisters: &Canisters,
    token_canister_id: Principal,
) -> Result<bool, String> {
    let token_canister = canisters.token_canister(token_canister_id).await;
    // Call the reject_sale method on the canister
    let response_result: Result<token::Result_, String> = token_canister
        .reject_sale()
        .await
        .map_err(|e| format!("Failed to call reject_sale: {}", e));

    // Handle the response
    match response_result {
        Ok(response) => match response {
            token::Result_::Ok(success) => {
                if success {
                    log!("Sale rejected successfully.");
                    Ok(true)
                } else {
                    let error_msg = "reject_sale returned Ok(false)".to_string();
                    log!("{}", error_msg);
                    Err(error_msg)
                }
            }
            token::Result_::Err(err_msg) => {
                let error_msg = format!("reject_sale failed: {}", err_msg);
                log!("{}", error_msg);
                Err(error_msg)
            }
        },
        Err(e) => Err(e),
    }
}

// Updated get_sale_status function
pub async fn get_sale_status(
    canisters: &Canisters,
    token_canister_id: Principal,
) -> Result<SaleStatus, String> {
    let token_canister = canisters.token_canister(token_canister_id).await;
    // Call the get_sale_status method on the canister
    let response_result: Result<SaleStatus, String> = token_canister
        .get_sale_status()
        .await
        .map_err(|e| e.to_string());

    // Handle the response
    match response_result {
        Ok(status) => {
            log!("Retrieved sale status: {:?}", status);
            Ok(status)
        }
        Err(e) => {
            let error_msg = format!("Agent error during get_sale_status: {}", e);
            log!("{}", error_msg);
            Err(error_msg)
        }
    }
}
