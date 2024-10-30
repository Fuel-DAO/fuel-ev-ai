use leptos::*;
use crate::components::specifications::{CollectionMetadata, SpecificationComponent};
use crate::components::collection_info_cards::CollectionInfoCards;
use crate::components::tabs::Tabs;
use crate::utils::share::ShareButtonWithFallbackPopup;
#[component]
pub fn CollectionHeader() -> impl IntoView {
    let collection_id = "collection_id" /* get_collection_id() */;  // Placeholder for your collection ID function

    // Define tabs and selected tab state
    let tabs = vec!["specifications".to_string(), "documents".to_string()];
    let selected = create_rw_signal( "specifications".to_string());
    let share_link_s =|| { format!("/token/collections/id") };
    let share_message_s = || {format!("Model S Plaid- SAMPLE
https://fueldao.io/collection/ft37p-5aaaa-aaaam-acpgq-cai@fu2z3-qyaaa-aaaam-acpga-cai
Take a look at this car at FuelDAO!")};


    // Check if the user is logged in and is the collection owner
    let is_owner = move || false;

    view! { 
        <div class="flex flex-col grow gap-4">
            <div class="flex flex-col sm:flex-row gap-4 lg:justify-between lg:items-center">
                <div class="flex flex-col lg:flex-row gap-8 items-start lg:items-center">
                    <div class="text-2xl lg:text-5xl font-bold">{ "Model S Plaid- SAMPLE" }</div>
                    <div class="py-2 px-4 text-xs bg-black rounded-full text-white font-light flex h-min items-center justify-center">
                        Open
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <ShareButtonWithFallbackPopup  
                    share_link=share_link_s()
                    message=share_message_s()
                    style="w-12 h-12".into()
                    />
                </div>
                
                // Conditional rendering for the edit button or actions
                // {move || if is_owner() {
                //     view! {  
                //         <Button secondary=true href=format!("/admin/edit-collection/{}", collection_id)>
                //             "Edit"
                //         </Button>
                //     }
                // } else {
                //     view! {  
                //         <Actions title=props.metadata.name.clone() />
                //     }
                // }}
            </div>

            <CollectionInfoCards props=CollectionMetadata::default() />
            
            <Tabs tabs=tabs.clone() selected=selected />

            // Conditional rendering for Specifications or Documents based on selected tab
            {move || if selected() == "specifications" {
                view! {  
                    <div>
                    <SpecificationComponent  />
                    </div>
                }
            } /* else if selected() == "documents" {
                view! {  
                    <Documents metadata=props.metadata.clone() />
                }
            } */ else {
                view! {  
                    <div>{ "No content available." }</div>
                }
            }}
        </div>
    }
}
