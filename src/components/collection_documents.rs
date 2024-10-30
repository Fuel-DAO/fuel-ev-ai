use leptos::*;

use crate::{canister::token::CollectionMetaData, components::info_title::InfoTitle};

#[component]
pub fn Documents( metadata: CollectionMetaData, asset_can_id: String) -> impl IntoView {
    // Function to construct document URLs
    let view_doc = move |path: &str| {
        let url = format!("https://{}.icp0.io{}", asset_can_id, path);
        window().location().set_href(&url).unwrap();
    };

    view! { 
        <div class="flex flex-col gap-8 pt-4 pb-32">
            <div class="rounded-2xl shadow-lg py-4 flex flex-col gap-5">
                {if  metadata.documents.is_empty() {
                    let documents = metadata.documents.clone();
                   view! {
                    <div>
                    {
                        let documents = documents.clone();
                         documents.into_iter().enumerate().map(|(i, (name, path))| {
                            let border_class = if i + 1 == metadata.documents.len() {
                                ""
                            } else {
                                "pb-4 border-black/10 border-b-[1px]"
                            };
    
                            view! { 
                                <div class=format!("flex items-center justify-between px-6 {}", border_class)>
                                    <InfoTitle classes="font-bold" title=name.clone() />
                                    <button
                                        on:click={
                                        let value = view_doc.clone();
                                        let path =format!("{}", path.clone());
                                        move |_| value(path.as_ref())
                                        }
                                        class="underline text-lg"
                                    >
                                        "View"
                                    </button>
                                </div>
                            }
                        }).collect_view()
                    }
                    </div>
                   }
                } else {
                    view! { 
                        <div class="flex items-center justify-between px-6">
                            <InfoTitle classes="font-light" title="No documents uploaded" />
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
