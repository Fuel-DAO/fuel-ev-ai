use crate::components::header::Header;
// use crate::state::canisters::{fetch_collections_data, Canisters, CollectionData};
use crate::components::header2::Header2;
use crate::{
    outbound::collection_canister_calls::fetch_collections_data, state::canisters::Canisters,
};
use std::rc::Rc;

use candid::{Nat, Principal};
use leptos::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq)]
enum Tab {
    All,
    Available,
    Upcoming,
}
use num_bigint::BigUint;
// Define a structure for the token metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TokenMetadata {
    weight: f64,
    drive_type: String,
    purchase_price: BigUint, // Represents `nat`
    token: Principal,
    name: String,
    // documents: Vec<Document>,
    supply_cap: BigUint,
    displays: String,
    seating: String,
    cargo: f64,
    logo: String,
    overall_height: f64,
    description: String,
    overall_width: f64,
    track_front: f64,
    collection_owner: Principal,
    asset_canister: Principal,
    ground_clearance: f64,
    key_features: Vec<String>,
    range_per_charge: f64,
    track_rear: f64,
    acceleration: String,
    charging_speed: String,
    wheels: f64,
    brochure_url: String,
    index: Principal,
    price: BigUint,
    battery: String,
    overall_length: f64,
    total_supply: BigUint,
    symbol: String,
    treasury: Principal,
    images: Vec<String>,
}
#[component]
pub fn Collections() -> impl IntoView {
    let selected_tab = create_rw_signal(Tab::All);
    // let canisters = use_context::<Rc<Canisters>>().expect("Canisters context must be provided");
    let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
        .expect("Canisters ReadWriteSignal must be provided");

    // Create a resource to fetch collection data and token metadata
    let collection_data = create_resource(
        move || canisters_signal.get().clone(), // Access the signal value correctly
        move |cans_option| async move {
            if let Some(cans) = cans_option {
                log::info!("Fetching collections data.");
                match fetch_collections_data(&cans).await {
                    Ok(data) => {
                        log::info!("Successfully fetched collections data.");
                        Ok(data)
                    }
                    Err(e) => {
                        log::error!("Error fetching collections data: {}", e);
                        Err(e)
                    }
                }
            } else {
                log::warn!("Canisters not available. User needs to log in.");
                Err("Canisters not available. Please log in.".to_string())
            }
        },
    );
    view! {
        <Header2 />
        <section class="p-6 ">
            <div class="w-full max-w-6xl pt-32 px-8 mx-auto 6xl:px-0">

                // Top Filter Bar
                <div class="flex justify-between items-center mb-8">
                    <div class="flex space-x-4">
                        <Tabs selected_tab tab=Tab::All label="All".to_string() />
                        <Tabs selected_tab tab=Tab::Available label="Available".to_string() />
                        <Tabs selected_tab tab=Tab::Upcoming label="Upcoming".to_string() />
                    </div>

                    // Sort Button (For future sorting functionality)
                    <div>
                        <button class="flex items-center px-4 py-2 bg-white text-black rounded-full shadow-md font-medium">
                            "Sort"
                            <svg
                                class="w-4 h-4 ml-2"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M19 9l-7 7-7-7"
                                ></path>
                            </svg>
                        </button>
                    </div>
                </div>

                // Card Grid
                <Suspense fallback=move || {
                    view! { <div>"Loading collections..."</div> }
                }>
                    {move || match collection_data.get() {
                        Some(Ok(collections)) => {
                            let filtered_cars = collections
                                .iter()
                                .filter(|collection| {
                                    match selected_tab.get() {
                                        Tab::All => true,
                                        Tab::Available => collection.status == "Available",
                                        Tab::Upcoming => collection.status == "Upcoming",
                                    }
                                })
                                .collect::<Vec<_>>();
                            view! {
                                <div class="flex py-12 items-center gap-8 justify-normal mx-auto flex-wrap ">

                                    {filtered_cars
                                        .into_iter()
                                        .map(|collection| {
                                            let href = format!(
                                                "/collections/{}/{}",
                                                collection.id.token_canister.to_text(),
                                                collection.id.asset_canister.to_text(),
                                            );
                                            view! {
                                                <a
                                                    href=href
                                                    class="rounded-xl relative overflow-hidden shadow-lg w-[20rem]"
                                                >
                                                    <div class="bg-white rounded-lg shadow-md overflow-hidden">

                                                        <div class="relative">
                                                            {collection
                                                                .metadata
                                                                .as_ref()
                                                                .map(|meta| {
                                                                    view! {
                                                                        <img
                                                                            src=meta.logo.clone()
                                                                            alt=meta.name.clone()
                                                                            class="w-full h-64 z-[2] object-cover"
                                                                        />
                                                                    }
                                                                })
                                                                .unwrap_or_else(|| {
                                                                    view! {
                                                                        <img
                                                                            src="/public/img/default_logo.jpg"
                                                                            alt=collection.name.clone()
                                                                            class="w-full h-64 z-[2] object-cover"
                                                                        />
                                                                    }
                                                                })}
                                                            <span class="absolute top-2 left-2 bg-white text-black font-semibold text-xs px-2 py-1 rounded-full">
                                                                {collection.status.clone()}
                                                            </span>
                                                        </div>
                                                        <div class="p-4">
                                                            <h3 class="text-lg font-semibold">
                                                                {collection.name.clone()}
                                                            </h3>
                                                            <p class="text-sm text-gray-600">
                                                                {collection
                                                                    .metadata
                                                                    .as_ref()
                                                                    .map(|meta| {
                                                                        let words: Vec<&str> = meta
                                                                            .description
                                                                            .split_whitespace()
                                                                            .take(14)
                                                                            .collect();
                                                                        if meta.description.split_whitespace().count() > 14 {
                                                                            format!("{}...", words.join(" "))
                                                                        } else {
                                                                            words.join(" ")
                                                                        }
                                                                    })}
                                                            </p>
                                                        </div>
                                                    </div>
                                                </a>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            }
                        }
                        Some(Err(e)) => {
                            view! { <div>{format!("Error fetching collections: {}", e)}</div> }
                        }
                        None => view! { <div>"Loading..."</div> },
                    }}
                </Suspense>
            </div>
        </section>
    }
}

#[component]
fn Tabs(selected_tab: RwSignal<Tab>, tab: Tab, label: String) -> impl IntoView {
    let current_tab = tab.clone();
    view! {
        <button
            class=move || {
                format!(
                    "px-4 py-2 rounded-md shadow-md font-medium {}",
                    if selected_tab.get() == current_tab {
                        "bg-white text-black"
                    } else {
                        "text-gray-500"
                    },
                )
            }
            on:click=move |_| selected_tab.set(tab.clone())
        >
            {label}
        </button>
    }
}
