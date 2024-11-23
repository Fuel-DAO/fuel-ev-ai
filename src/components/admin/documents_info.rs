// documents_info.rs

use crate::state::asset_manager::AssetManager;
use crate::state::canisters::Canisters;
use candid::Principal;
use gloo::file::futures::read_as_bytes;
use gloo_file::Blob;
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

#[derive(Default, Debug, Clone)]
pub struct DocumentsInfoData {
    pub documents: Vec<(String, String)>, // (Document Name, URL)
}

#[component]
pub fn DocumentsInfo(documents: RwSignal<Vec<(String, String)>>) -> impl IntoView {
    // Access the Canisters context as RwSignal<Option<Rc<Canisters>>>
    let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
        .expect("Canisters ReadWriteSignal must be provided");

    // Local signals for upload state
    let uploading = create_rw_signal(false);
    let uploading_progress = create_rw_signal(0);
    let error_document = create_rw_signal(false);
    let error_message = create_rw_signal(String::new());

    // Determine if input elements should be disabled
    let disabled = move || uploading.get();

    // Handler for file selection (upload)
    let on_select = {
        let uploading = uploading.clone();
        let uploading_progress = uploading_progress.clone();
        let documents = documents.clone();
        let error_document = error_document.clone();
        let error_message = error_message.clone();
        let canisters_signal = canisters_signal.clone();

        Rc::new(move |event: Event| {
            // Reset error states
            error_document.set(false);
            error_message.set(String::new());

            // Extract the file input element
            let input = match event.target() {
                Some(target) => match target.dyn_into::<HtmlInputElement>() {
                    Ok(input) => input,
                    Err(_) => return,
                },
                None => return,
            };

            // Retrieve the selected file
            let files = input.files();
            if files.is_none() {
                uploading.set(false);
                return;
            }
            let files = files.unwrap();
            if files.length() == 0 {
                uploading.set(false);
                return;
            }
            let file = match files.get(0) {
                Some(file) => file,
                None => {
                    uploading.set(false);
                    return;
                }
            };

            uploading.set(true);
            uploading_progress.set(0);
            let blob = Blob::from(file.clone());

            // Clone necessary variables for async task
            let canisters_signal = canisters_signal.clone();
            let uploading = uploading.clone();
            let uploading_progress = uploading_progress.clone();
            let documents = documents.clone();
            let error_document = error_document.clone();
            let error_message = error_message.clone();

            // Perform the upload asynchronously
            spawn_local(async move {
                // Check if canisters are available
                let canisters_option = canisters_signal.get();
                if canisters_option.is_none() {
                    log::error!("Canisters not available.");
                    error_message.set("Canisters not available.".to_string());
                    error_document.set(true);
                    uploading.set(false);
                    return;
                }
                let canisters = canisters_option.unwrap();

                let agent = &canisters.agent;

                // Parse canister ID
                let upload_canister_id = "your_upload_canister_id_here"; // Replace with actual ID or pass as prop
                let upload_principal = match Principal::from_text(upload_canister_id) {
                    Ok(principal) => principal,
                    Err(_) => {
                        log::error!("Invalid upload canister ID");
                        error_message.set("Invalid upload canister ID.".to_string());
                        error_document.set(true);
                        uploading.set(false);
                        return;
                    }
                };

                // Initialize the AssetManager
                let manager = AssetManager::new(upload_principal.clone(), upload_principal, agent); // Adjust if different

                // Read the file data
                let file_name = file.name();
                let file_data = match read_as_bytes(&blob).await {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        log::error!("Failed to read file data: {:?}", e);
                        error_message.set("Failed to read file data.".to_string());
                        error_document.set(true);
                        uploading.set(false);
                        return;
                    }
                };

                // Upload the file
                match manager.store(file_data, file_name.clone()).await {
                    Ok(url) => {
                        documents.update(|docs| docs.push((file_name.clone(), url.clone())));
                        uploading_progress.set(100);
                    }
                    Err(e) => {
                        log::error!("Upload failed: {}", e);
                        error_message.set(format!("Upload failed: {}", e));
                        error_document.set(true);
                    }
                }

                uploading.set(false);
                input.set_value("");
            });
        }) as Rc<dyn Fn(Event) + 'static>
    };

    // Handler for removing documents
    let remove_document = {
        let documents = documents.clone();
        let canisters_signal = canisters_signal.clone();
        let error_message = error_message.clone();

        Rc::new(move |name: String| {
            let documents = documents.clone();
            let canisters_signal = canisters_signal.clone();
            let error_message = error_message.clone();

            spawn_local(async move {
                // Check if canisters are available
                let canisters_option = canisters_signal.get();
                if canisters_option.is_none() {
                    log::error!("Canisters not available.");
                    error_message.set("Canisters not available.".to_string());
                    return;
                }
                let canisters = canisters_option.unwrap();

                let agent = &canisters.agent;

                // Parse canister ID
                let upload_canister_id = "your_upload_canister_id_here"; // Replace with actual ID or pass as prop
                let upload_principal = match Principal::from_text(upload_canister_id) {
                    Ok(principal) => principal,
                    Err(_) => {
                        log::error!("Invalid upload canister ID");
                        error_message.set("Invalid upload canister ID.".to_string());
                        return;
                    }
                };

                // Initialize the AssetManager
                let manager = AssetManager::new(upload_principal.clone(), upload_principal, agent); // Adjust if different

                // Delete the document
                match manager.delete(name.clone()).await {
                    Ok(_) => {
                        documents.update(|docs| docs.retain(|(doc_name, _)| doc_name != &name));
                    }
                    Err(e) => {
                        log::error!("Deletion failed: {}", e);
                        error_message.set(format!("Deletion failed: {}", e));
                    }
                }
            });
        }) as Rc<dyn Fn(String) + 'static>
    };

    view! {
        <div class="flex flex-col -mt-8 gap-4">
            <span class="text-sm font-medium leading-6 text-gray-900 mt-8">"Documents:"</span>
            <div class="h-[14rem] border rounded p-2 items-center w-full overflow-hidden overflow-x-auto flex gap-2">
                <For each=move || documents.get().clone() key=|doc| doc.0.clone() let:doc>
                    {
                        let remove_document = remove_document.clone();
                        let doc_name = doc.0.clone();
                        view! {
                            <div class="p-1 shrink-0 border rounded-md w-52 h-52 relative transition-opacity">
                                <button
                                    on:click={
                                        let remove_document = remove_document.clone();
                                        let doc_name = doc_name.clone();
                                        move |_| { (remove_document)(doc_name.clone()) }
                                    }
                                    class="bg-white rounded-full flex items-center justify-center w-4 h-4 absolute top-2 right-2"
                                    aria-label=format!("Remove document {}", doc_name)
                                >
                                    "X"
                                </button>
                                <a
                                    href=doc.1.clone()
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="h-full w-full rounded-md object-contain flex items-center justify-center"
                                >
                                    <span class="text-sm text-blue-600 underline">
                                        {doc.0.clone()}
                                    </span>
                                </a>
                            </div>
                        }
                    }
                </For>
                <Show when=move || documents.get().is_empty() fallback=|| ()>
                    <div class="flex flex-1 text-sm items-center justify-center">
                        "No documents added yet"
                    </div>
                </Show>
            </div>

            <div class="mt-4 flex flex-col gap-2">
                <label class=move || {
                    format!(
                        "w-min transition-opacity {}",
                        if uploading.get() { "pointer-events-none opacity-50" } else { "" },
                    )
                }>
                    <div class="bg-primary shadow-md rounded-full flex items-center gap-2 font-semibold py-2 px-6 hover:bg-green-600 active:bg-green-500 transition-colors cursor-pointer text-nowrap text-sm text-white">
                        <div>
                            {move || {
                                if uploading.get() {
                                    format!("Uploading... ({}%)", uploading_progress.get())
                                } else {
                                    "Upload Document".to_string()
                                }
                            }}
                        </div>
                    </div>

                    <input
                        disabled=disabled()
                        on:change={
                            let on_select_clone = on_select.clone();
                            move |e| (on_select_clone)(e)
                        }
                        type="file"
                        accept=".pdf,.doc,.docx,.txt"
                        class="sr-only"
                    />
                </label>
            </div>

            <Show when=move || !error_message.get().is_empty()>
                <div class="text-red-500 text-sm mt-2">{error_message.get()}</div>
            </Show>
        </div>
    }
}
