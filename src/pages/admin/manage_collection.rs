use crate::canister::provision::CollectionRequest;
use crate::components::admin::manage_collection::{
    form_header::FormHeader, info_section::InfoSection, item_info::ItemInfo,
};
use crate::components::header2::Header2;
use crate::{
    outbound::get_pending_collection_requests::{
        approve_request,  get_request_info_by_id, reject_request,
    },
    state::canisters::Canisters,
};
use leptos::*;
use leptos_router::*;
use serde_json::Value;
use std::rc::Rc;

/// Converts Metadata to key-value pairs
fn metadata_to_key_value_pairs(metadata: &CollectionRequest) -> Vec<(String, String)> {
    // Serialize `Metadata` to a JSON-compatible Value
    let metadata_value = serde_json::to_value(metadata).unwrap();

    // Parse the JSON Value into a map of fields
    if let Value::Object(map) = metadata_value {
        map.into_iter()
            .map(|(key, value)| {
                let value_str = match value {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Array(arr) => format!(
                        "[{}]",
                        arr.into_iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    Value::Object(_) => "[Complex Value]".to_string(),
                    _ => "null".to_string(),
                };
                (key, value_str)
            })
            .collect()
    } else {
        vec![]
    }
}

#[derive(Params, PartialEq, Debug)]
struct ContactParams {
    id: u64, // Changed from usize to u128
}

#[derive(Params, PartialEq)]
struct ContactSearch {
    q: String,
}

#[component]
pub fn ManageCollectionPage() -> impl IntoView {
    // Signals for route parameters and state
    let params = use_params::<ContactParams>();
    let query = use_query::<ContactSearch>();

    // Define a closure to extract the id parameter
    let id =
        move || params.with(|params: &Result<ContactParams, ParamsError>| params.as_ref().map(|params| params.id).unwrap_or_default());
    let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
        .expect("Canisters ReadWriteSignal must be provided");
    let canisters_clone = canisters_signal.get().clone();

    // Create a resource to fetch collection details
    let collection_details = create_resource(
        move || canisters_signal.get().clone(),
        move |cans_option| async move {
            if let Some(cans) = cans_option {
                match get_request_info_by_id(cans.as_ref(), id()).await {
                    Ok(data) => {
                        logging::log!("data: {:?}", data);
                        Ok(data)
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err("Canisters not available. Please log in.".to_string())
            }
        },
    );
    // Function to handle approval
    let approve_action = move || {
        if let Some(canisters) = canisters_signal.get().clone() {
            let collection_id = id();
            spawn_local(async move {
                match approve_request(canisters.as_ref(), collection_id).await {
                    Ok((id, token_canister, asset_canister)) => {
                        logging::log!(
                            "Approval successful! ID: {}, Token Canister: {}, Asset Canister: {}",
                            id,
                            token_canister,
                            asset_canister
                        );
                    }
                    Err(err) => {
                        logging::log!("Error approving request: {}", err);
                    }
                }
            });
        } else {
            logging::log!("Canisters not available. Please log in.");
        }
    };
    let reject_action = move || {
        if let Some(canisters) = canisters_clone.clone() {
            let collection_id = id();
            spawn_local(async move {
                match reject_request(canisters.as_ref(), collection_id).await {
                    Ok(_) => {
                        logging::log!("Rejection successful for ID: {}", id());
                    }
                    Err(err) => {
                        logging::log!("Error rejecting request: {}", err);
                    }
                }
            });
        } else {
            logging::log!("Canisters not available. Please log in.");
        }
    };
    view! {
        <Header2 />
        <div class="w-full max-w-6xl pt-32 px-8 mx-auto 6xl:px-0">
            <div class="flex justify-between items-center">
                <h1 class="text-xl font-bold">"Manage Collection"</h1>

                <div class="flex space-x-2">
                    <button
                        class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 inline-flex items-center rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30"
                        on:click=move |_| approve_action()
                    >
                        "Approve"
                    </button>
                    <button
                        class="bg-primary hover:bg-red-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-black focus-visible:outline-red-300 ring-0 px-4 py-2 inline-flex items-center rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30"
                        on:click=move |_| reject_action()
                    >
                        "Decline"
                    </button>
                </div>
            </div>
            <div class="manage-collection-page flex flex-col gap-8 divide-y divide-gray-300 mx-32">
                <InfoSection>
                    <FormHeader
                        title="Basic Details".to_string()
                        subtitle=format!("Form ID: {}", id()).to_string()
                    />
                    <div class="flex flex-col gap-4 mt-4">
                        {move || match collection_details.get() {
                            Some(Ok(data)) => {
                                view! {
                                    {if let Some(metadata) = &data.metadata {
                                        logging::log!("metadata: {:?}", metadata);
                                        let metadata_items = metadata_to_key_value_pairs(metadata);
                                        view! {
                                            // Dynamically extract fields

                                            <div>
                                                {metadata_items
                                                    .iter()
                                                    .map(|(title, value)| {
                                                        view! {
                                                            <ItemInfo title=title.clone() value=value.clone() />
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        }
                                    } else {
                                        view! {
                                            <div class="text-gray-500">"No metadata available."</div>
                                        }
                                    }}
                                }
                            }
                            Some(Err(err)) => {
                                view! { <div class="text-red-500">{err.clone()}</div> }
                            }
                            None => {
                                view! { <div class="text-gray-500">"Loading..."</div> }
                            }
                        }}
                    </div>
                </InfoSection>

                <InfoSection>
                    <FormHeader title="Collection Assets".to_string() subtitle="".to_string() />
                    <div class="flex flex-wrap gap-4">
                        <div class="h-[14rem] w-[14rem] border rounded flex items-center justify-center">
                            <img
                                src="/static/logo.png"
                                class="h-full w-full rounded-md object-contain"
                                alt="Collection Logo"
                            />
                        </div>
                        <div class="h-[14rem] w-[14rem] border rounded flex items-center justify-center">
                            <img
                                src="/static/image1.png"
                                class="h-full w-full rounded-md object-contain"
                                alt="Collection Image"
                            />
                        </div>
                    </div>
                </InfoSection>

                <InfoSection>
                    <FormHeader title="Documents".to_string() subtitle="".to_string() />
                    <div class="flex flex-wrap gap-4">
                        <div class="h-[14rem] w-[14rem] border rounded flex items-center justify-center">
                            <a
                                href="/static/document1.pdf"
                                target="_blank"
                                class="text-blue-500 underline"
                            >
                                {"Document 1"}
                            </a>
                        </div>
                    </div>
                </InfoSection>
            </div>
        </div>
    }
}
