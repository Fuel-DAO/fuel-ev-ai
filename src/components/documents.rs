use leptos::*;
use web_sys::window;

use crate::canister::token::GetMetadataRet;

#[component]
pub fn DocumentList(metadata: GetMetadataRet) -> impl IntoView {
    let asset_can_id = metadata.asset_canister.to_text();
    let documents = metadata.documents.clone();

    let view_doc = move |path: String| {
        let url = format!("https://{}.icp0.io{}", asset_can_id, path);
        if let Some(win) = window() {
            let _ = win.open_with_url_and_target(&url, "_blank");
        }
    };

    view! {
        <div class="flex flex-col gap-8 pt-4 pb-32">
            <div class="rounded-2xl shadow-lg py-4 flex flex-col gap-5">
                {
                    if documents.is_empty() {
                        view! {
                            <div class="flex items-center justify-between px-6">
                                <div class="font-light">"No documents uploaded"</div>
                            </div>
                        }
                    } else {
                        view! {
                            <div>
                                {documents.iter().enumerate().map(|(i, (name, path))| {
                                    let is_last = i + 1 == documents.len();
                                    let path_clone = path.clone();
                                    view! {
                                        <div class={format!(
                                            "flex items-center justify-between px-6 {}",
                                            if is_last { "" } else { "pb-4 border-black/10 border-b-[1px]" }
                                        )}>
                                            <div class="font-bold">{name.clone()}</div>
                                            <button
                                                class="underline text-lg"
                                                on:click={
                                                let value = view_doc.clone();
                                                move |_| value(path_clone.clone())
                                                }
                                            >
                                                "View"
                                            </button>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }
                    }
                }
            </div>
        </div>
    }
}
