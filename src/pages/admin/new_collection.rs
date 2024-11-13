use leptos::*;
use web_sys::window;
use crate::components::header2::Header2;

// Import the subcomponents and ImagesInfoData
use crate::components::admin::{
    basic_info::BasicInfo,
    collection_info::CollectionInfo,
    documents_info::DocumentsInfo,
    images_info::{ImagesInfo, ImagesInfoData},
};

#[component]
pub fn NewCollectionForm() -> impl IntoView {
    // ==== State Variables ====

    // Selected tab state
    let (selected_tab, set_selected_tab) = create_signal("basic".to_string());

    // Loading state (provided via context)
    let (loading, set_loading) = create_signal(false);
    // provide_context(loading.read_only());

    // Result state after form submission
    let (res, set_res) = create_signal(None::<String>);

    // ==== Form Data Signals ====

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
    // ... (define other fields similarly)

    // Documents Data Signal
    let documents = create_rw_signal(vec!["Document 1".to_string(), "Document 2".to_string()]);

    // Images Info Data Signal
    let images_info_data = create_rw_signal(ImagesInfoData {
        images: vec![
            "https://plus.unsplash.com/premium_photo-1664303847960-586318f59035?q=80&w=2874&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D".to_string(),
        ],
        logo: "https://plus.unsplash.com/premium_photo-1664303847960-586318f59035?q=80&w=2874&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D".to_string(),
    });

    // ==== Event Handlers ====

    // Handler for form submission
    let submit_form = move |_| {
        if loading.get() {
            return; // Prevent duplicate submissions
        }
        set_loading(true); // Set loading state

        // Simulate an asynchronous operation (e.g., API call)
        spawn_local(async move {
            use gloo_timers::future::TimeoutFuture;
            TimeoutFuture::new(2000).await; // 2-second delay

            // Collect form data
            let basic_info = format!(
                "Name: {}\nTreasury: {}\nPrice: {}\nSupply Cap: {}\nSymbol: {}\nDescription: {}",
                name.get(),
                treasury.get(),
                price.get(),
                supply_cap.get(),
                symbol.get(),
                description.get()
            );

            let collection_info = format!(
                "Purchase Price: {}\nWeight: {}\nDrive Type: {}",
                purchase_price.get(),
                weight.get(),
                drive_type.get(),
                // ... include other fields as needed
            );

            let docs = documents.get().join(", ");

            // Extract images and logo
            let images_data = images_info_data.get();
            let images = images_data.images.join(", ");
            let logo = images_data.logo.clone();

            let form_data = format!(
                "Basic Info:\n{}\n\nCollection Info:\n{}\n\nDocuments:\n{}\n\nImages:\n{}\nLogo:\n{}",
                basic_info, collection_info, docs, images, logo
            );

            // After delay, set the result and reset loading state
            set_res(Some(format!(
                "Form submitted successfully!\n\n{}",
                form_data
            )));
            set_loading(false);
        });
    };

    // Handler to navigate back in browser history
    let go_back = move |_| {
        if let Some(history) = window().and_then(|w| w.history().ok()) {
            let _ = history.back();
        }
    };

    // ==== View Rendering ====

    view! {
        <Header2 />
        <div class="flex flex-col items-center pt-32 gap-4 min-h-screen p-4">
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
                                <h1 class="font-bold text-xl">"New Collection"</h1>
                                <div>
                                    <p>"Submit data to create a new collection."</p>
                                    <p>
                                        "Submitted form will be approved by an admin for listing."
                                    </p>
                                </div>
                                <div>
                                    <button on:click=move |_| set_selected_tab(
                                        "basic".to_string(),
                                    )>"Basic "</button>
                                    <button on:click=move |_| set_selected_tab(
                                        "info ".to_string(),
                                    )>"Info"</button>
                                    <button on:click=move |_| set_selected_tab(
                                        "images".to_string(),
                                    )>"Images"</button>
                                    <button on:click=move |_| set_selected_tab(
                                        "documents".to_string(),
                                    )>"Documents"</button>
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
                                <div>
                                    <button on:click=go_back disabled=loading.get()>
                                        "Cancel"
                                    </button>
                                    <button on:click=submit_form disabled=loading.get()>
                                        "Save"
                                    </button>
                                </div>
                            </div>
                        }
                    }
                }}
            </div>
        </div>
    }
}
