use leptos::*;
use std::rc::Rc;

// Subcomponent for Documents Tab
#[component]
pub fn DocumentsInfo(documents: RwSignal<Vec<String>>) -> impl IntoView {
    // Access the loading state from context
    let loading = use_context::<ReadSignal<bool>>()
        .unwrap_or_else(|| create_rw_signal(false).read_only());

    // Handlers
    // Wrap the remove_document handler in Rc to allow cloning
    let remove_document = Rc::new(move |index: usize| {
        documents.update(|d| {
            d.remove(index);
        });
    });

    let upload_document = move |_| {
        if loading.get() {
            return;
        }
        // Simulate document upload
        let new_doc = format!("Document {}", documents.get().len() + 1);
        documents.update(|d| d.push(new_doc));
    };

    view! {
        <div class="flex flex-col gap-2">
            <span>"Documents:"</span>
            <div class="h-[14rem] border rounded p-2 items-center w-full overflow-hidden overflow-x-auto flex gap-2">
                <For
                    each=move || documents.get()
                    key=|doc| doc.clone()
                    // Use the children prop
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
                                    <span>{doc.clone()}</span>
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
                <button
                    on:click=upload_document
                    disabled=move || loading.get()
                    class="bg-primary shadow-md rounded-full flex items-center gap-2 font-semibold py-2 px-6 hover:bg-green-600 active:bg-green-500 transition-colors cursor-pointer text-nowrap text-sm text-white"
                >
                    {move || if loading.get() { "Uploading..." } else { "Upload Document" }}
                </button>
            </div>
        </div>
    }
}

