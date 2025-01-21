use candid::Principal;
use leptos::*;
use crate::canister::token::SaleStatus;
use crate::canister::token::GetMetadataRet;
use crate::components::admin::invest_info_admin::{AddFallbackPrincipalForAnnonymousInvestor, ConcludeSaleAdminComponent, RefundICPsToAnnonymous, TransferAmountFromAnnonymousToInvestor};
use crate::outbound::accept_or_reject_sale::get_sale_status;
use crate::state::admin::Admin;
use crate::state::canisters::Canisters;
use crate::state::sale_status::SaleStatusState;
use crate::{
    components::{
        collection_header::CollectionHeader, collection_images::CollectionImages, header::Header,
        invest_info::InvestInfo,
    },
    outbound::collection_canister_calls::get_collection_metadata_from_token_canister,
};
use leptos_router::{use_params, Params};
use std::cmp::PartialEq;

#[derive(Debug, Clone, Params, PartialEq)]
pub struct CollectionParams {
    pub  token_id: String,
    pub asset_id: String,
}

#[component]
pub fn CollectionDetail() -> impl IntoView {
    // Use the defined parameters struct
    let params = use_params::<CollectionParams>();

    // Get the ID from the params, default to "unknown" if it doesn't exist
    let id = params
        .get()
        .map(|p| p.token_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());
    // let asset_can_id = params
    //     .get()
    //     .map(|p| p.asset_id.clone())
    //     .unwrap_or_else(|_| "unknown".to_string());
    let collection_id = id.clone();
    let token_canister_id = Principal::from_text(collection_id.clone()).unwrap();
    // let asset_canister_id = Principal::from_text(asset_can_id.clone()).unwrap();
    let collection_resource = create_resource(
        move || collection_id.clone(), // Dependency: collection_id
        move |token_id| {
            async move {
                // Convert token_id string to Principal
                let principal = Principal::from_text(token_id).map_err(|e| e.to_string())?;

                // Ensure Canisters is available
                if let Some(cans_rc) = Canisters::get() {

                    let metadata = get_collection_metadata_from_token_canister( &cans_rc, principal).await;
                    let status = get_sale_status( &cans_rc,principal).await.ok();

                    match metadata {
    Ok(meta) =>  {
        match meta {

    crate::canister::token::Result4::Ok(get_metadata_ret) =>{
        SaleStatusState::set_listing_metadata(token_canister_id, get_metadata_ret.clone());
        SaleStatusState::set_listing_satatus(token_canister_id, status.clone().unwrap_or(SaleStatus::Rejected));
         Ok((get_metadata_ret, status))},
    crate::canister::token::Result4::Err(e) => Err(e),
}
    },
    Err(e) => Err(e),
}
                } else {
                    Err("Canisters not available. Please log in.".to_string())
                }
            }
        },
    );
    view! {
        <Suspense>
           <div>
           {
            move || collection_resource
                .get().map(|_|{})
           }
           </div>
           <CollectionDetailsInner />
        </Suspense>
    }
}

#[component]
fn CollectionDetailsInner() -> impl IntoView {

    let params = use_params::<CollectionParams>();

    // Get the ID from the params, default to "unknown" if it doesn't exist
    let id = params
        .get()
        .map(|p| p.token_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());
    let asset_can_id = params
        .get()
        .map(|p| p.asset_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());
    let collection_id = id.clone();
    
    let token_canister_id = Principal::from_text(collection_id.clone()).unwrap();
    let asset_canister_id = Principal::from_text(asset_can_id.clone()).unwrap();

    let metadata =move || SaleStatusState::get_listing_metadata(token_canister_id)();
    let status =move || SaleStatusState::get_listing_status(token_canister_id)();
    
        view! {
            <div>
                <Header />
                <Show when=move || SaleStatusState::get_listing_metadata(token_canister_id)().is_some()>
                <div class="w-full max-w-6xl pt-32 mx-auto px-8 lg:px-0">
                    <CollectionImages metadata=metadata().unwrap() asset_can_id=asset_canister_id.to_text() />
                    <div class="w-full flex flex-col items-center justify-center  gap-4 pb-8">
                        <CarDetailPage  metadata=metadata().unwrap() status=Some(status())  />
                    </div>
                </div>
                </Show>
            </div>
    }
}



#[component]
fn CarDetailPage(metadata: GetMetadataRet, status: Option<SaleStatus>) -> impl IntoView {
    let params: Memo<Result<CollectionParams, leptos_router::ParamsError>> = use_params::<CollectionParams>();

    let collection_id = params
        .get()
        .map(|p| p.token_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());
    // let asset_id = params
    //     .get()
    //     .map(|p| p.asset_id.clone())
    //     .unwrap_or_else(|_| "unknown".to_string());

    let token_canister_id = Principal::from_text(collection_id.clone()).unwrap();
    

    view! {
        <div class="w-full flex flex-col items-center gap-4 pb-8">
            <div class="flex flex-col lg:flex-row gap-8 pt-6 w-full max-w-6xl">
                <CollectionHeader metadata=metadata.clone() collection_id />
                <div class="flex flex-col gap-8">
                    <InvestInfo metadata=metadata.clone() token_canister_id />
                    <Show clone:metadata when=move||(Admin::get().principal.get().is_some() && Admin::get().principal.get().unwrap() == metadata.collection_owner )>
                        <ConcludeSaleAdminComponent metadata=metadata.clone() token_canister_id is_active=status.is_some() && status.clone().unwrap() == SaleStatus::Live />
                    </Show>
                    // <Show when=move||(Admin::get().principal.get().is_some() && Admin::get().principal.get().unwrap() == metadata.collection_owner )>
                    //     <RefundExcessAfterSale  token_canister_id />
                    // </Show>
                    // <Show when=move||(Admin::get().principal.get().is_some() && Admin::get().principal.get().unwrap() == metadata.collection_owner )>
                    //     <TransferAmountFromAnnonymousToInvestor  token_canister_id />
                    // </Show>
                    // <Show when=move||(Admin::get().principal.get().is_some() && Admin::get().principal.get().unwrap() == metadata.collection_owner )>
                    //     <AddFallbackPrincipalForAnnonymousInvestor  token_canister_id />
                    // </Show>
                    // <Show when=move||(Admin::get().principal.get().is_some() && Admin::get().principal.get().unwrap() == metadata.collection_owner )>
                    //     <RefundICPsToAnnonymous  token_canister_id />
                    // </Show>
                </div>
                // <div>"check"</div>
            // <div class="flex flex-col gap-8">
            // <SpecificationComponent />
            // </div>
            </div>
            // <div>"check"</div>

        </div>
    }
}
