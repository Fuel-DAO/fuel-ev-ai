use leptos::*;

use crate::canister::token::CollectionMetaData;

#[component]
pub fn CollectionInfoCards( props: CollectionMetaData) -> impl IntoView {
    // Set default values for metadata fields
    let acceleration = props.acceleration.clone();
    let seating = props.seating.clone();
    let range = props.range_per_charge.clone();

    view! { 
        <div class="grid col-auto lg:flex items-center gap-2 pt-8">
            <div class="border border-black/20 flex-1 rounded-xl p-4 flex flex-col gap-1 text-black items-center">
                <div class="font-bold text-nowrap">"Acceleration"</div>
                <div class="">{acceleration}</div>
            </div>

            <div class="border border-black/20 flex-1 rounded-xl p-4 flex flex-col gap-1 text-black items-center">
                <div class="font-bold text-nowrap">"Range per charge"</div>
                <div class="">{range} "KM"</div>
            </div>

            <div class="border border-black/20 flex-1 rounded-xl p-4 flex flex-col gap-1 text-black items-center">
                <div class="font-bold text-nowrap">"Seating"</div>
                <div class="">{seating} " Seats"</div>
            </div>
        </div>
    }
}
