use leptos::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, FileList, HtmlInputElement}; // Import JsCast to use dyn_into

// Your component function and logic...
#[component]
pub fn DocumentsInfo(documents: RwSignal<Vec<String>>) -> impl IntoView {
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
        // Correctly cast EventTarget to HtmlInputElement using dyn_into()
        let input: HtmlInputElement = event
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let files = input.files().unwrap();
        if files.length() == 0 {
            return;
        }
        let file = files.get(0).unwrap();
        let file_name = file.name();
        documents.update(|d| d.push(file_name));
        input.set_value(""); // Clear the input to allow the same file to be reselected if necessary
    });

    view! {
        <div class="flex flex-col gap-2">
            <span>"Documents:"</span>
            <div class="h-[14rem] border rounded p-2 items-center w-full overflow-hidden overflow-x-auto flex gap-2">
                <For
                    each=move || documents.get()
                    key=|doc| doc.clone()
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
