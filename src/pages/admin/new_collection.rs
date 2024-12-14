use crate::canister::{provision::CollectionRequest, ASSET_PROXY_ID};
use crate::components::header2::Header2;
use crate::outbound::add_collection_canister_calls::add_collection;
use crate::state::canisters::Canisters;
use crate::TEMP_ASSET_CANISTER_ID;
use candid::{Nat, Principal};
use leptos::logging::log;
use leptos::*;
use std::rc::Rc;
use web_sys::MouseEvent;


// Import the subcomponents and ImagesInfoData
use crate::components::admin::{
    basic_info::BasicInfo,
    collection_info::CollectionInfo,
    documents_info::DocumentsInfo,
    images_info::{ImagesInfo, ImagesInfoData},
};

const TOKEN_PRINCIPAL: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const INDEX_PRINCIPAL: &str = "qhbym-qaaaa-aaaaa-aaafq-cai";

#[component]
pub fn NewCollectionForm() -> impl IntoView {
    // ==== Define All Required Signals ====
    // dotenv().ok();

    // Basic Info Data Signals
    let name = create_rw_signal("".to_string());
    let treasury = create_rw_signal("".to_string());
    let price = create_rw_signal(1.0);
    let supply_cap = create_rw_signal(1000);
    let symbol = create_rw_signal("ICP".to_string());
    let description = create_rw_signal("".to_string());

    // Collection Info Data Signals
    let purchase_price = create_rw_signal(0.0);
    let weight = create_rw_signal(0.0);
    let drive_type = create_rw_signal("".to_string());
    let displays = create_rw_signal("".to_string());
    let seating = create_rw_signal("".to_string());
    let cargo = create_rw_signal(0.0);
    let overall_height = create_rw_signal(0.0);
    let overall_width = create_rw_signal(0.0);
    let overall_length = create_rw_signal(0.0);
    let track_front = create_rw_signal(0.0);
    let track_rear = create_rw_signal(0.0);
    let ground_clearance = create_rw_signal(0.0);
    let key_features = create_rw_signal("".to_string());
    let range_per_charge = create_rw_signal(0.0);
    let acceleration = create_rw_signal("".to_string());
    let charging_speed = create_rw_signal("".to_string());
    let wheels = create_rw_signal(0u32);
    let brochure_url = create_rw_signal("".to_string());
    let battery = create_rw_signal("".to_string());
    let title = "New Collection".to_string();
    let success_message = create_rw_signal(String::new());

    // Documents Data Signal
    let documents = create_rw_signal(Vec::<(String, String)>::new());

    // Images Info Data Signal
    let images_info_data = create_rw_signal(ImagesInfoData::default());

    // ==== Define Additional Signals for Form Handling ====
    let loading = create_rw_signal(false);
    let res = create_rw_signal::<Option<String>>(None);
    let selected_tab = create_rw_signal("basic".to_string());

    // ==== Event Handlers ====
    let asset_proxy =ASSET_PROXY_ID;
    let asset_canister = TEMP_ASSET_CANISTER_ID;
    let asset_proxy_canister_id: RwSignal<String> = create_rw_signal(asset_proxy.to_string());
    let asset_canister_id: RwSignal<String> = create_rw_signal(asset_canister.to_string());
    log!("asset_canister_id: {:?}", asset_canister_id.get());
    log!(
        "asset_proxy_canister_id: {:?}",
        asset_proxy_canister_id.get()
    );
    log!("asset_canister_id: {:?} ", asset_canister_id);
    let on_cancel = move |_e: MouseEvent| {
        // Implement your cancel logic here
        log::info!("Cancel button clicked");
        // Example: Reset form fields
        name.set("".to_string());
        treasury.set("".to_string());
        price.set(1.0);
        supply_cap.set(1000);
        symbol.set("ICP".to_string());
        description.set("".to_string());
        // Reset other signals as needed
    };

    let on_submit = {
        // Clone all the signals used in the async block
        let loading = loading.clone();
        let success_message = success_message.clone();
        let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
            .expect("Canisters ReadWriteSignal must be provided");
        // Clone form data signals
        let name = name.clone();
        let treasury = treasury.clone();
        let price = price.clone();
        let supply_cap = supply_cap.clone();
        let symbol = symbol.clone();
        let description = description.clone();
        // Clone other signals...
        let purchase_price = purchase_price.clone();
        let weight = weight.clone();
        let drive_type = drive_type.clone();
        let displays = displays.clone();
        let seating = seating.clone();
        let cargo = cargo.clone();
        let overall_height = overall_height.clone();
        let overall_width = overall_width.clone();
        let overall_length = overall_length.clone();
        let track_front = track_front.clone();
        let track_rear = track_rear.clone();
        let ground_clearance = ground_clearance.clone();
        let key_features = key_features.clone();
        let range_per_charge = range_per_charge.clone();
        let acceleration = acceleration.clone();
        let charging_speed = charging_speed.clone();
        let wheels = wheels.clone();
        let brochure_url = brochure_url.clone();
        let battery = battery.clone();
        let documents = documents.clone();
        let images_info_data = images_info_data.clone();

        move |_e: MouseEvent| {
            log!("Form Submission Initiated");

            if loading.get() {
                return; // Prevent duplicate submissions
            }

            loading.set(true);

            spawn_local(async move {
                if let Some(canisters) = canisters_signal.get() {
                    // Parse necessary fields

                    let treasury_principal = match Principal::from_text(&treasury.get()) {
                        Ok(principal) => principal.to_text(),
                        Err(_) => {
                            loading.set(false);
                            success_message.set("Invalid treasury principal.".to_string());
                            return;
                        }
                    };

                    let collection_data1 = CollectionRequest {
                        weight: weight.get(),
                        drive_type: drive_type.get(),
                        purchase_price: Nat::from(purchase_price.get() as u64),
                        token: Principal::from_text(TOKEN_PRINCIPAL)
                            .expect("Invalid token principal"), // Adjust as necessary
                        documents: documents.get(),
                        supply_cap: Nat::from(supply_cap.get() as u64),
                        displays: displays.get(),
                        seating: seating.get(),
                        cargo: cargo.get(),
                        logo: images_info_data.get().logo.clone(),
                        name: name.get(),
                        overall_height: overall_height.get(),
                        description: description.get(),
                        overall_width: overall_width.get(),
                        overall_length: overall_length.get(),
                        track_front: track_front.get(),
                        track_rear: track_rear.get(),
                        ground_clearance: ground_clearance.get(),
                        key_features: key_features
                            .get()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect(),
                        range_per_charge: range_per_charge.get(),
                        acceleration: acceleration.get(),
                        charging_speed: charging_speed.get(),
                        wheels: wheels.get() as f64,
                        brochure_url: brochure_url.get(),
                        battery: battery.get(),
                        price: price.get() * 100_000_000.0 , // To e8s
                        symbol: symbol.get(),
                        treasury: Principal::from_text(&treasury_principal)
                            .expect("Invalid token principal"),
                        images: images_info_data.get().images.clone(),
                        index: Principal::from_text(INDEX_PRINCIPAL)
                            .expect("Invalid index principal"), // Adjust as necessary
                                                                // Note: Ensure all required fields are included
                                                                // If there are more fields in AddCollectionRequestArg, include them here
                    };
                    log!("collection_data1: {:?}", collection_data1);

                    // Call the add_collection function
                    match add_collection(&canisters, collection_data1).await {
                        Ok(_) => {
                            loading.set(false);
                            success_message.set("Collection created successfully!".to_string());
                            // Optionally, reset form fields
                        }
                        Err(e) => {
                            loading.set(false);
                            success_message.set(format!("Error: {}", e));
                        }
                    }
                } else {
                    loading.set(false);
                    success_message.set("Canisters not available. Please log in.".to_string());
                }
            });
        }
    };

    // Clone `asset_canister_id` to avoid moving it into the closure

    view! {
        <Header2 />
        <div class="flex flex-col mx-[10%] md:mx-[15%] lg:mx-[20%] pt-32 gap-4 min-h-screen p-4">
            <div class="flex flex-col gap-12 pb-16">
                {move || {
                    if let Some(result) = res.get() {
                        view! {
                            // ==== Render Result After Submission ====
                            <div>
                                <pre class="text-sm p-4 mt-8 bg-gray-100 rounded-xl">{result}</pre>
                                <a href="/admin/manage/list">"Approve/Deny on admin panel"</a>
                            </div>
                        }
                    } else {
                        view! {
                            <div>
                                <div class="flex items-start justify-between py-4 gap-4">
                                    <div class="flex flex-col">
                                        <h1 class="font-semibold text-4xl">{"New Collection"}</h1>
                                        <p>{"Submit data to create a new collection."}</p>
                                        <p>
                                            {"Submitted forms will be approved by an admin for listing."}
                                        </p>
                                    </div>
                                    <div class="mt-3 flex gap-3 items-center">
                                        <Show
                                            when=move || !success_message.get().is_empty()
                                            fallback=|| ()
                                        >
                                            <div class="text-xs text-green-500 font-medium pr-4">
                                                {success_message.get()}
                                            </div>
                                        </Show>
                                        <button
                                            class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30"
                                            on:click=on_cancel
                                            disabled=loading.get()
                                        >
                                            {"Cancel"}
                                        </button>
                                        <button
                                            class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30"
                                            on:click=on_submit
                                            disabled=loading.get()
                                        >
                                            {if loading.get() { "Submitting..." } else { "Submit" }}
                                        </button>
                                    </div>
                                </div>
                                <div class="flex space-x-4 border-b border-gray-200 my-8">
                                    // Basic Tab Button
                                    <button
                                        class=move || {
                                            if selected_tab.get() == "basic" {
                                                "pb-2 ml-4 border-b-2 border-gray-800 text-gray-800 font-semibold transition-colors duration-200"
                                            } else {
                                                "pb-2 ml-4 border-b-2 border-transparent text-gray-600 hover:border-gray-400 transition-colors duration-200"
                                            }
                                        }
                                        on:click=move |_| selected_tab.set("basic".to_string())
                                        aria-selected=move || selected_tab.get() == "basic"
                                        role="tab"
                                    >
                                        "Basic"
                                    </button>

                                    // Info Tab Button
                                    <button
                                        class=move || {
                                            if selected_tab.get() == "info" {
                                                "pb-2 ml-4 border-b-2 border-gray-800 text-gray-800 font-semibold transition-colors duration-200"
                                            } else {
                                                "pb-2 ml-4 border-b-2 border-transparent text-gray-600 hover:border-gray-400 transition-colors duration-200"
                                            }
                                        }
                                        on:click=move |_| selected_tab.set("info".to_string())
                                        aria-selected=move || selected_tab.get() == "info"
                                        role="tab"
                                    >
                                        "Info"
                                    </button>

                                    // Images Tab Button
                                    <button
                                        class=move || {
                                            if selected_tab.get() == "images" {
                                                "pb-2 ml-4 border-b-2 border-gray-800 text-gray-800 font-semibold transition-colors duration-200"
                                            } else {
                                                "pb-2 ml-4 border-b-2 border-transparent text-gray-600 hover:border-gray-400 transition-colors duration-200"
                                            }
                                        }
                                        on:click=move |_| selected_tab.set("images".to_string())
                                        aria-selected=move || selected_tab.get() == "images"
                                        role="tab"
                                    >
                                        "Images"
                                    </button>

                                    // Documents Tab Button
                                    <button
                                        class=move || {
                                            if selected_tab.get() == "documents" {
                                                "pb-2 ml-4 border-b-2 border-gray-800 text-gray-800 font-semibold transition-colors duration-200"
                                            } else {
                                                "pb-2 ml-4 border-b-2 border-transparent text-gray-600 hover:border-gray-400 transition-colors duration-200"
                                            }
                                        }
                                        on:click=move |_| selected_tab.set("documents".to_string())
                                        aria-selected=move || selected_tab.get() == "documents"
                                        role="tab"
                                    >
                                        "Documents"
                                    </button>
                                </div>
                                <div>
                                    // ==== Render Tab Content Exclusively ====
                                    <Show when=move || selected_tab.get() == "basic" fallback=|| ()>
                                        <BasicInfo
                                            name=name
                                            treasury=treasury
                                            price=price
                                            supply_cap=supply_cap
                                            symbol=symbol
                                            description=description
                                        />
                                    </Show>

                                    <Show when=move || selected_tab.get() == "info" fallback=|| ()>
                                        <CollectionInfo
                                            purchase_price=purchase_price
                                            weight=weight
                                            drive_type=drive_type
                                            displays=displays
                                            seating=seating
                                            cargo=cargo
                                            overall_height=overall_height
                                            overall_width=overall_width
                                            overall_length=overall_length
                                            track_front=track_front
                                            track_rear=track_rear
                                            ground_clearance=ground_clearance
                                            key_features=key_features
                                            range_per_charge=range_per_charge
                                            acceleration=acceleration
                                            charging_speed=charging_speed
                                            wheels=wheels
                                            brochure_url=brochure_url
                                            battery=battery
                                        />
                                    </Show>

                                    <Show
                                        when=move || selected_tab.get() == "images"
                                        fallback=|| ()
                                    >
                                        <ImagesInfo
                                            data=images_info_data
                                            absolute_logo_path=false
                                            // upload_canister_id=asset_proxy_canister_id.get()
                                            // asset_canister_id=asset_canister_id.get()
                                        />
                                    </Show>

                                    <Show
                                        when=move || selected_tab.get() == "documents"
                                        fallback=|| ()
                                    >
                                        <DocumentsInfo documents=documents />
                                    </Show>

                                    <Show
                                        when=move || {
                                            !(selected_tab.get() == "basic"
                                                || selected_tab.get() == "info"
                                                || selected_tab.get() == "images"
                                                || selected_tab.get() == "documents")
                                        }
                                        fallback=|| ()
                                    >
                                        <div>"Unknown Tab"</div>
                                    </Show>
                                </div>
                            </div>
                        }
                    }
                }}
            </div>
        </div>
    }
}
