use candid::Principal;
use leptos::*;
use crate::canister::token::{self, SaleStatus};
use crate::canister::token::GetMetadataRet;
use crate::components::admin::invest_info_admin::{AddFallbackPrincipalForAnnonymousInvestor, ConcludeSaleAdminComponent, RefundICPsToAnnonymous};
use crate::outbound::accept_or_reject_sale::get_sale_status;
use crate::state::admin::Admin;
use crate::state::canisters::Canisters;
use crate::{
    components::{
        collection_header::CollectionHeader, collection_images::CollectionImages, header::Header,
        invest_info::InvestInfo,
    },
    outbound::collection_canister_calls::get_collection_metadata_from_token_canister,
};
use leptos_router::{use_params, Params};
use std::cmp::PartialEq;

use std::rc::Rc;
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
    let asset_can_id = params
        .get()
        .map(|p| p.asset_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());
    let collection_id = id.clone();
    // let canisters = use_context::<Rc<Canisters>>();
    let canisters = use_context::<RwSignal<Option<Rc<Canisters>>>>()
        .expect("Canisters ReadWriteSignal must be provided");

    // Create a resource to fetch collection metadata
    let collection_resource = create_resource(
        move || collection_id.clone(), // Dependency: collection_id
        move |token_id| {
            let canisters = canisters.clone();
            async move {
                // Convert token_id string to Principal
                let principal = Principal::from_text(token_id).map_err(|e| e.to_string())?;

                // Ensure Canisters is available
                if let Some(cans_rc) = canisters.get() {
                    let metadata = get_collection_metadata_from_token_canister( &cans_rc, principal).await;
                    let status = get_sale_status( &cans_rc,principal).await.ok();

                    match metadata {
    Ok(meta) =>  {
        match meta {
    crate::canister::token::Result4::Ok(get_metadata_ret) => Ok((get_metadata_ret, status)),
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
            {
                let asset_can_id = asset_can_id.clone();
                move || {
                    collection_resource
                        .get()
                        .map(|res| match res {
                            Ok(metadata) => {
                                let asset_can_id = asset_can_id.clone();
                                view! {
                                    <div>
                                        <Header />
                                        <div class="w-full max-w-6xl pt-32 mx-auto px-8 lg:px-0">
                                            <CollectionImages metadata=metadata.0.clone() asset_can_id />
                                            <div class="w-full flex flex-col items-center justify-center  gap-4 pb-8">
                                                <CarDetailPage  metadata=metadata.0 status=metadata.1 />
                                                // <div>"check"</div>

                                            </div>
                                        </div>
                                    </div>
                                }
                            }
                            Err(e) => {
                                view! { <div>Failed to get details</div> }
                            }
                        })
                }
            }

        </Suspense>
    }
}

#[component]
fn CarDetailPage(metadata: GetMetadataRet, status: Option<SaleStatus>) -> impl IntoView {
    let params: Memo<Result<CollectionParams, leptos_router::ParamsError>> = use_params::<CollectionParams>();

    let collection_id = params
        .get()
        .map(|p| p.token_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());
    let asset_id = params
        .get()
        .map(|p| p.asset_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());

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
                    <Show when=move||(Admin::get().principal.get().is_some() && Admin::get().principal.get().unwrap() == metadata.collection_owner )>
                        <AddFallbackPrincipalForAnnonymousInvestor  token_canister_id />
                    </Show>
                    <Show when=move||(Admin::get().principal.get().is_some() && Admin::get().principal.get().unwrap() == metadata.collection_owner )>
                        <RefundICPsToAnnonymous  token_canister_id />
                    </Show>
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
