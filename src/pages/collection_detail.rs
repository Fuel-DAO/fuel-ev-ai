use candid::Principal;
use leptos::*;

use leptos_router::{use_params, Params};
use std::cmp::PartialEq;

use crate::{canister::token::CollectionMetaData, components::{collection_header::CollectionHeader, header::Header, invest_info::InvestInfo,}, outbound::collection_canister_calls::get_collection_metadata_from_token_canister};

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

    let collection_resource = create_resource(||(), move |_| { let token_canister_id = id.clone();  async {  get_collection_metadata_from_token_canister(Principal::from_text(token_canister_id).map_err(|e|e.to_string())?).await } } );

    view! {
        <Suspense>
        {
            move || collection_resource.get().map(|res| match res {
                Ok(metadata) => {
                    view! {
                        <div>
                        <Header />
                        <div class="w-full max-w-6xl pt-32 mx-auto px-8 lg:px-0">
                            <div class="w-full flex flex-col items-center justify-center  gap-4 pb-8">
                
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
                
                                // <h2>{format!("Collection ID: {}", id)}</h2>
                                // <p>"Details for the selected collection will be displayed here."</p>
                                <CarDetailPage metadata />
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
fn CarDetailPage(metadata: CollectionMetaData) -> impl IntoView {

    view! {
        <div class="w-full flex flex-col items-center gap-4 pb-8">
	<div class="flex flex-col lg:flex-row gap-8 pt-6 w-full max-w-6xl">
		<CollectionHeader metadata=metadata.clone() />
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

