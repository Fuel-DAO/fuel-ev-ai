use candid::Principal;
use leptos::*;

use leptos_router::{use_params, Params};
use std::cmp::PartialEq;

use crate::{canister::token::CollectionMetaData, components::{collection_header::CollectionHeader, collection_images::CollectionImages, header::Header, invest_info::InvestInfo}, outbound::collection_canister_calls::get_collection_metadata_from_token_canister};

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
    let asset_can_id = params
        .get()
        .map(|p| p.asset_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());

    let collection_id = id.clone();
    let collection_resource = create_resource(||(), move |_| { let token_canister_id = collection_id.clone();  async {  get_collection_metadata_from_token_canister(Principal::from_text(token_canister_id).map_err(|e|e.to_string())?).await } } );
    view! {
        <Suspense>
        {   
            let asset_can_id = asset_can_id.clone();

            move || collection_resource.get().map(|res| match res {
                Ok(metadata) => {
                    let asset_can_id = asset_can_id.clone();
                    view! {
                        <div>
                        <Header />
                        <div class="w-full max-w-6xl pt-32 mx-auto px-8 lg:px-0">
                            <CollectionImages metadata=metadata.clone() asset_can_id />
                            <div class="w-full flex flex-col items-center justify-center  gap-4 pb-8">
                                <CarDetailPage metadata/>
                            </div>
                        </div>
                        </div>
                    }
                }, 
                Err(e) => {
                    view! {
                        <div>
                        Failed to get details
                        </div>
                    }
                }
            })
        }

        

        </Suspense>
    }

    
}


#[component]
fn CarDetailPage(metadata: CollectionMetaData,) -> impl IntoView {
    let params = use_params::<CollectionParams>();

    let collection_id = params
        .get()
        .map(|p| p.token_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());
    let asset_id = params
        .get()
        .map(|p| p.asset_id.clone())
        .unwrap_or_else(|_| "unknown".to_string());

    view! {
        <div class="w-full flex flex-col items-center gap-4 pb-8">
	<div class="flex flex-col lg:flex-row gap-8 pt-6 w-full max-w-6xl">
		<CollectionHeader metadata=metadata.clone() collection_id />
        <div class="flex flex-col gap-8">
			<InvestInfo metadata />
		</div>
		// <div class="flex flex-col gap-8">
        //     <SpecificationComponent />
		// </div>
	</div>
</div>
    }
}

