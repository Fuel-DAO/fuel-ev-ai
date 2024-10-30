use leptos::*;

use super::specifications::CollectionMetadata;

#[derive(Clone, Debug)]
pub struct InvestInfoMetaProps {
    pub metadata: CollectionMetadata,
    pub booked_tokens: u64,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Live,
    Rejected,
    Accepted,
}

#[component]
pub fn InvestInfo(   ) -> impl IntoView {

    let props = InvestInfoMetaProps  {
        metadata: CollectionMetadata::default(),
        booked_tokens: 5,
        status: Status::Live
    };

    
    // Function to calculate invested percentage
    let invested_percentage = move || {
        if props.booked_tokens > 0 && props.metadata.supply_cap.unwrap_or_default() > 0 {
            ((props.booked_tokens as f64 / props.metadata.supply_cap.unwrap_or_default() as f64) * 100.0).to_string()
        } else {
            "0".to_string()
        }
    };

    // Function to calculate invested amount in ICP
    let invested_icp = move || {
        let price = from_e8s(props.metadata.price.unwrap_or_default());
        (props.booked_tokens as f64 * price).to_string()
    };

    view! { 
        <div class="shrink-0 bg-primary rounded-xl flex flex-col text-white gap-3 p-6 shadow-xl h-fit">
            <div class="font-bold text-5xl">
                {if props.status == Status::Live { "Open" } else { "Closed" }}
            </div>

            <div class="text-2xl">
                "Invested " <span class="font-bold">{invested_icp()} " ICP"</span>
            </div>

            <div class="w-72 bg-black/20 h-4 rounded-full relative overflow-hidden">
                <div
                    style=move || format!("width: {}%;", invested_percentage())
                    class="absolute transition-all bg-white rounded-full left-0 h-full"
                />
            </div>

            <div class="text-md font-light">
                "Funded " {invested_percentage()} "%" {if props.status == Status::Live { "till now" } else { "" }}
            </div>

            <button 
                class="btn-secondary"
                disabled=move || props.status != Status::Live
                on:click=move |_| {
                    // Handle Invest button click logic here
                }
            >
                "Invest"
            </button>
        </div>
    }
}

// Helper function to convert e8s to ICP
fn from_e8s(e8s: u64) -> f64 {
    e8s as f64 / 1e8
}
