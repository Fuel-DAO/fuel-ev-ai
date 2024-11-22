use leptos::logging::log;
use leptos::*;
use std::future::Future;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

// Mock upload function (replace with your actual upload logic)
async fn upload_document(file: web_sys::File) -> Result<String, String> {
    // Implement actual upload logic here.
    // For demonstration, we'll return a dummy URL.
    Ok(format!("https://your-storage-service.com/{}", file.name()))
}

#[component]
pub fn DocumentsInfo(documents: RwSignal<Vec<(String, String)>>) -> impl IntoView {
    let loading =
        use_context::<ReadSignal<bool>>().unwrap_or_else(|| create_rw_signal(false).read_only());

    let remove_document = Rc::new(move |index: usize| {
        documents.update(|d| {
            d.remove(index);
        });
    });

    let on_select_document = Rc::new(move |event: Event| {
        if loading.get() {
            return;
        }

        let input: HtmlInputElement = event
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        let files = input.files().clone();
        if files.is_none() || files.unwrap().length() == 0 {
            return;
        }

        let file = input.files().clone().unwrap().get(0).unwrap();
        let file_clone = file.clone();
        input.set_value(""); // Clear the input

        let documents = documents.clone();
        let loading = loading.clone();

        // Spawn an async task to handle the upload
        spawn_local(async move {
            match upload_document(file_clone).await {
                Ok(url) => {
                    // Determine the document type based on file extension
                    let doc_type = if url.ends_with(".pdf") {
                        "pdf".to_string()
                    } else if url.ends_with(".doc") || url.ends_with(".docx") {
                        "docx".to_string()
                    } else {
                        "unknown".to_string()
                    };
                    documents.update(|d| d.push((doc_type, url)));
                }
                Err(e) => {
                    // Handle upload error (e.g., show a notification)
                    log!("Error uploading document: {}", e);
                }
            }
        });
    });

    view! {
        <div class="flex flex-col gap-2">
            <span>"Documents:"</span>
            <div class="h-[14rem] border rounded p-2 items-center w-full overflow-hidden overflow-x-auto flex gap-2">
                <For
                    each=move || documents.get()
                    // Use URL as key
                    key=|doc| doc.1.clone()
                    children=move |doc| {
                        let remove_document = Rc::clone(&remove_document);
                        let doc_clone = doc.clone();
                        view! {
                            <div class="p-1 shrink-0 border rounded-md w-64 h-52 relative">
                                <button
                                    disabled=move || loading.get()
                                    on:click=move |_| {
                                        let index = documents
                                            .get()
                                            .iter()
                                            .position(|d| d == &doc_clone)
                                            .unwrap_or(0);
                                        remove_document(index);
                                    }
                                    class="bg-gray-200 z-[2] rounded-full flex items-center justify-center w-6 h-6 absolute top-2 right-2"
                                >
                                    "Remove"
                                </button>
                                <div class="flex items-center justify-center h-full">
                                    <a
                                        href=doc.1.clone()
                                        target="_blank"
                                        class="text-blue-500 underline"
                                    >
                                        {doc.1.clone()}
                                    </a>
                                </div>
                            </div>
                        }
                    }
                />
                <Show when=move || documents.get().is_empty() fallback=|| ()>
                    <div class="flex flex-1 items-center justify-center">
                        "No documents uploaded yet"
                    </div>
                </Show>
            </div>
            <div class="mt-4 flex flex-col gap-2">
                <label class="bg-primary shadow-md rounded-full flex items-center gap-2 font-semibold py-2 px-6 hover:bg-green-600 active:bg-green-500 transition-colors cursor-pointer text-nowrap text-sm text-white">
                    {move || if loading.get() { "Uploading..." } else { "Upload Document" }}
                    <input
                        on:change=move |e| on_select_document(e)
                        type="file"
                        accept=".pdf,.doc,.docx"
                        class="sr-only"
                    />
                </label>
            </div>
        </div>
    }
}
