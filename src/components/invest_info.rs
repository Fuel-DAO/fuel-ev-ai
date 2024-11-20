use html::meta;
use leptos::*;
use crate::canister::token::CollectionMetaData;


#[derive(Clone, Debug)]
pub struct InvestInfoMetaProps {
    pub metadata: CollectionMetaData,
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
pub fn InvestInfo( metadata: CollectionMetaData  ) -> impl IntoView {

    let props = InvestInfoMetaProps  {
        metadata: metadata,
        booked_tokens: 5,
        status: Status::Live
    };


    // Function to calculate invested percentage
    let invested_percentage = move || {
        if props.booked_tokens > 0 && props.metadata.supply_cap.0.to_string().parse::<f64>().unwrap_or_default() > 0.0 {
            ((props.booked_tokens as f64 / props.metadata.supply_cap.to_string().parse::<f64>().unwrap_or_default()) * 100.0).to_string()
        } else {
            "0".to_string()
        }
    };

    let invested_percentage_clone = invested_percentage.clone();

    // Function to calculate invested amount in ICP
    let invested_icp = move || {
        let price = from_e8s(props.metadata.price.0.to_string().parse::<u64>().unwrap_or_default() );
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
                    style=move || format!("width: {}%;", invested_percentage().clone())
                    class="absolute transition-all bg-white rounded-full left-0 h-full"
                />
            </div>

            <div class="text-md font-light">
                "Funded " {invested_percentage_clone()} "%"
                {if props.status == Status::Live { "till now" } else { "" }}
            </div>
            <button
                role="presentation"
                type="button"
                disabled=move || props.status != Status::Live
                on:click=move |_| {}
                class="bg-white ring-1 ring-inset ring-gray-100 hover:bg-gray-50 outline-none active:bg-gray-200 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 "
            >
                <div class=" transition-opacity">Invest</div>
            </button>
        // <button
        // class="btn-secondary"
        // disabled=move || props.status != Status::Live
        // on:click=move |_| {}
        // >
        // "Invest"
        // </button>
        </div>
    }
}

// Helper function to convert e8s to ICP
fn from_e8s(e8s: u64) -> f64 {
    e8s as f64 / 1e8
}
