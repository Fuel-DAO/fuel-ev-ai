use crate::canister::provision::{AddCollectionRequestArg, Document};
use crate::components::header2::Header2;
use crate::outbound::add_collection_canister_calls::add_collection;
use crate::state::canisters::Canisters;
use candid::{Nat, Principal};
use leptos::logging::log;
use leptos::*;
use std::rc::Rc;
use web_sys::window;
use web_sys::MouseEvent;
// Import the subcomponents and ImagesInfoData
use crate::components::admin::{
    basic_info::BasicInfo,
    collection_info::CollectionInfo,
    documents_info::DocumentsInfo,
    images_info::{ImagesInfo, ImagesInfoData},
};
use gloo_timers::future::TimeoutFuture;
#[component]
pub fn NewCollectionForm() -> impl IntoView {
    // ==== Define All Required Signals ====

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
    // let success_message = create_signal(String::new());
    let success_message = create_rw_signal(String::new());

    // Documents Data Signal
    let documents = create_rw_signal(vec!["Document 1".to_string(), "Document 2".to_string()]);

    // Images Info Data Signal
    let images_info_data = create_rw_signal(ImagesInfoData {
        images: vec![
            "https://plus.unsplash.com/premium_photo-1664303847960-586318f59035?q=80&w=2874&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D".to_string(),
        ],
        logo: "https://plus.unsplash.com/premium_photo-1664303847960-586318f59035?q=80&w=2874&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D".to_string(),
    });

    // ==== Define Additional Signals for Form Handling ====
    let loading = create_rw_signal(false);
    let res = create_rw_signal::<Option<String>>(None);
    let selected_tab = create_rw_signal("basic".to_string());

    // ==== Event Handlers ====

    // Handler to navigate back in browser history
    // let go_back = move |_| {
    //     if let Some(history) = window().and_then(|w| w.history().ok()) {
    //         let _ = history.back();
    //     }
    // };

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
        // logging::log!("name: {:?}", name);
        // log!("Form Submission Initiated");
        // log!("name: {:?}", name.get());
        // log!("treasury: {:?}", treasury.get());
        // log!("price: {:?}", price.get());
        // log!("supply_cap: {:?}", supply_cap.get());
        // log!("symbol: {:?}", symbol.get());
        // log!("description: {:?}", description.get());
        // log!("purchase_price: {:?}", purchase_price.get());
        // log!("weight: {:?}", weight.get());
        // log!("drive_type: {:?}", drive_type.get());
        // log!("displays: {:?}", displays.get());
        // log!("seating: {:?}", seating.get());
        // log!("cargo: {:?}", cargo.get());
        // log!("overall_height: {:?}", overall_height.get());
        // log!("overall_width: {:?}", overall_width.get());
        // log!("overall_length: {:?}", overall_length.get());
        // log!("track_front: {:?}", track_front.get());
        // log!("track_rear: {:?}", track_rear.get());
        // log!("ground_clearance: {:?}", ground_clearance.get());
        // log!("key_features: {:?}", key_features.get());
        // log!("range_per_charge: {:?}", range_per_charge.get());
        // log!("acceleration: {:?}", acceleration.get());
        // log!("charging_speed: {:?}", charging_speed.get());
        // log!("wheels: {:?}", wheels.get());
        // log!("brochure_url: {:?}", brochure_url.get());
        // log!("battery: {:?}", battery.get());
        // log!("title: {:?}", title);
        // log!("documents: {:?}", documents.get());
        // log!("images_info_data: {:?}", images_info_data.get());
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
        let documents = documents.clone();
        let images_info_data = images_info_data.clone();
        // ... and so on for all signals

        move |_e: MouseEvent| {
            log!("Form Submission Initiated");

            // // Log the values of the signals
            // log!("name: {:?}", name.get());
            // log!("treasury: {:?}", treasury.get());
            // log!("price: {:?}", price.get());
            // log!("supply_cap: {:?}", supply_cap.get());
            // log!("symbol: {:?}", symbol.get());
            // log!("description: {:?}", description.get());
            // log!("purchase_price: {:?}", purchase_price.get());
            // log!("weight: {:?}", weight.get());
            // log!("drive_type: {:?}", drive_type.get());
            // log!("displays: {:?}", displays.get());
            // log!("seating: {:?}", seating.get());
            // log!("cargo: {:?}", cargo.get());
            // log!("overall_height: {:?}", overall_height.get());
            // log!("overall_width: {:?}", overall_width.get());
            // log!("overall_length: {:?}", overall_length.get());
            // log!("track_front: {:?}", track_front.get());
            // log!("track_rear: {:?}", track_rear.get());
            // log!("ground_clearance: {:?}", ground_clearance.get());
            // log!("key_features: {:?}", key_features.get());
            // log!("range_per_charge: {:?}", range_per_charge.get());
            // log!("acceleration: {:?}", acceleration.get());
            // log!("charging_speed: {:?}", charging_speed.get());
            // log!("wheels: {:?}", wheels.get());
            // log!("brochure_url: {:?}", brochure_url.get());
            // log!("battery: {:?}", battery.get());
            // log!("documents: {:?}", documents.get());
            // log!("images_info_data: {:?}", images_info_data.get());

            if loading.get() {
                return; // Prevent duplicate submissions
            }
            loading.set(true);
            let documents_transformed = vec![
                Document {
                    title: "Document 1".to_string(),
                    url: "https://pdfobject.com/pdf/sample.pdf".to_string(),
                },
                Document {
                    title: "Document 2".to_string(),
                    url: "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf"
                        .to_string(),
                },
            ];
            log!("documents: {:?}", documents_transformed);

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

                    let collection_data1 = AddCollectionRequestArg {
                        weight: weight.get(),
                        drive_type: drive_type.get(),
                        purchase_price: Nat::from(purchase_price.get() as u64),
                        token: Principal::from_text(&treasury_principal)
                            .expect("Invalid token principal"), // Adjust as necessary
                        documents: vec![],

                        // documents: documents_transformed, // Corrected to use transformed data
                        supply_cap: Nat::from(supply_cap.get() as u64),
                        displays: displays.get(),
                        seating: seating.get(),
                        cargo: cargo.get(),
                        logo: images_info_data.get().logo.clone(),
                        name: name.get(),
                        overall_height: overall_height.get(),
                        description: description.get(),
                        overall_width: overall_width.get(),
                        track_front: track_front.get(),
                        ground_clearance: ground_clearance.get(),
                        key_features: key_features
                            .get()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect(),
                        range_per_charge: range_per_charge.get(),
                        track_rear: track_rear.get(),
                        acceleration: acceleration.get(),
                        charging_speed: charging_speed.get(),
                        wheels: wheels.get() as f64,
                        brochure_url: brochure_url.get(),
                        index: Principal::from_text(&treasury_principal)
                            .expect("Invalid token principal"), // Adjust as necessary
                        price: Nat::from(price.get() as u64),
                        battery: battery.get(),
                        overall_length: overall_length.get(),
                        symbol: symbol.get(),
                        treasury: Principal::from_text(&treasury_principal)
                            .expect("Invalid token principal"),
                        images: images_info_data.get().images.clone(),
                    };
                    log!("collection_data1: {:?}", collection_data1);

                    let collection_data = AddCollectionRequestArg {
                        weight: 1.0,
                        drive_type: "type".to_string(),
                        purchase_price: Nat::from(1u64),
                        token: Principal::from_text(
                            "acred-4djfc-ledly-qhddc-eugbf-geqcc-w5g2i-dry2j-rce77-vorar-5qe",
                        )
                        .unwrap(),
                        documents: vec![],
                        supply_cap: Nat::from(1u64),
                        displays: "disdpayes".to_string(),
                        seating: "1".to_string(),
                        cargo: 1.0,
                        logo: "sdf".to_string(),
                        name: "sd".to_string(),
                        overall_height: 1.0,
                        description: "desc".to_string(),
                        overall_width: 1.0,
                        track_front: 1.0,
                        ground_clearance: 1.0,
                        key_features: vec!["dsfgdf".to_string()],
                        range_per_charge: f64::NAN,
                        track_rear: f64::NAN,
                        acceleration: "".to_string(),
                        charging_speed: "".to_string(),
                        wheels: f64::NAN,
                        brochure_url: "".to_string(),
                        index: Principal::from_text(
                            "acred-4djfc-ledly-qhddc-eugbf-geqcc-w5g2i-dry2j-rce77-vorar-5qe",
                        )
                        .unwrap(),
                        price: Nat::from(0u64),
                        battery: "".to_string(),
                        overall_length: f64::NAN,
                        symbol: "".to_string(),
                        treasury: Principal::from_text(
                            "acred-4djfc-ledly-qhddc-eugbf-geqcc-w5g2i-dry2j-rce77-vorar-5qe",
                        )
                        .unwrap(),
                        images: vec![],
                    };
                    log!("collection_data: {:?}", collection_data);
                    //
                    // Call the add_collection_request function
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
                            // ==== Render the Form ====
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

                                            class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 "
                                            on:click=on_cancel
                                            disabled=loading.get()
                                        >
                                            {"Cancel"}
                                        </button>
                                        <button

                                            class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 "
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
                                    {move || match selected_tab.get().as_str() {
                                        "basic" => {
                                            view! {
                                                // ==== Basic Info Tab Content ====
                                                <div>
                                                    <BasicInfo
                                                        name=name
                                                        treasury=treasury
                                                        price=price
                                                        supply_cap=supply_cap
                                                        symbol=symbol
                                                        description=description
                                                    />
                                                </div>
                                            }
                                        }
                                        "info" => {
                                            view! {
                                                // ==== Collection Info Tab Content ====
                                                <div>
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
                                                </div>
                                            }
                                        }
                                        "images" => {
                                            view! {
                                                <div>
                                                    <ImagesInfo
                                                        data=images_info_data
                                                        absolute_logo_path=false
                                                        upload_canister_id="your_upload_canister_id".to_string()
                                                        asset_canister_id="your_asset_canister_id".to_string()
                                                    />
                                                </div>
                                            }
                                        }
                                        "documents" => {
                                            view! {
                                                // ==== Documents Tab Content ====
                                                <div>
                                                    <DocumentsInfo documents=documents />
                                                </div>
                                            }
                                        }
                                        _ => {
                                            view! {
                                                // ==== Unknown Tab Content ====
                                                <div>"Unknown Tab"</div>
                                            }
                                        }
                                    }}
                                </div>
                            </div>
                        }
                    }
                }}
            </div>
        </div>
    }
}
