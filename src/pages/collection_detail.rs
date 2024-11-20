use candid::Principal;
use leptos::*;

use leptos_router::{use_params, Params};
use std::cmp::PartialEq;

use crate::state::canisters::Canisters;
use crate::{
    canister::token::CollectionMetaData,
    components::{collection_header::CollectionHeader, header::Header, invest_info::InvestInfo},
    outbound::collection_canister_calls::get_collection_metadata_from_token_canister,
};
use std::rc::Rc;
#[derive(Debug, Clone, Params, PartialEq)]
pub struct CollectionParams {
    token_id: String,
    asset_id: String,
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
                    get_collection_metadata_from_token_canister(cans_rc.as_ref(), principal).await
                } else {
                    Err("Canisters not available. Please log in.".to_string())
                }
            }
        },
    );
    view! {
        <Suspense fallback=|| {
            view! { <div>"Loading..."</div> }
        }>
            {move || match collection_resource.get() {
                Some(Ok(metadata)) => {
                    view! {
                        <div>
                            <Header />
                            <div class="w-full max-w-6xl pt-32 mx-auto px-8 lg:px-0">
                                <div class="w-full flex flex-col items-center justify-center gap-4 pb-8">
                                    <div class="flex flex-col lg:flex-row gap-2 lg:h-[40rem] w-full overflow-hidden overflow-x-auto relative">
                                        <div class="absolute h-16 lg:h-28 top-4 shadow-md z-[2] left-4 w-16 lg:w-28 rounded-full overflow-hidden">
                                            <img
                                                alt="Collection logo"
                                                class="h-full w-full object-cover object-center"
                                                src="https://fu2z3-qyaaa-aaaam-acpga-cai.icp0.io/Tesla_Model_S_2021-01@2x.jpg"
                                            />
                                        </div>
                                        <img
                                            alt="Collection"
                                            src="https://fu2z3-qyaaa-aaaam-acpga-cai.icp0.io/Tesla_Model_S_2021-03@2x.jpg"
                                            class="rounded-xl lg:h-full lg:grow object-cover"
                                        />
                                    </div>
                                    <CarDetailPage metadata=metadata.clone() />
                                </div>
                            </div>
                        </div>
                    }
                }
                Some(Err(e)) => {
                    view! { <div>{format!("Failed to get details: {}", e)}</div> }
                }
                None => view! { <div>"Loading..."</div> },
            }}
        </Suspense>
    }
}

#[component]
fn CarDetailPage(metadata: CollectionMetaData) -> impl IntoView {
    let params = use_params::<CollectionParams>();

    let collection_id = params
        .get()
        .map(|p| p.token_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());

    view! {
        <div class="w-full flex flex-col items-center gap-4 pb-8">
            <div class="flex flex-col lg:flex-row gap-8 pt-6 w-full max-w-6xl">
                <CollectionHeader metadata=metadata.clone() collection_id />
                <div class="flex flex-col gap-8">
                    <InvestInfo metadata />
                </div>
            // <div class="flex flex-col gap-8">
            // <SpecificationComponent />
            // </div>
            </div>
        </div>
    }
}
