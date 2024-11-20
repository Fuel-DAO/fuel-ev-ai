use crate::components::header2::Header2;
use crate::{
    outbound::get_pending_collection_requests::fetch_pending_requests_data,
    state::canisters::Canisters,
};
use leptos::*;
use leptos::*;
use log;
use std::rc::Rc;
use web_sys::window;
/// Represents the metadata of a form.
#[derive(Clone)]
struct Metadata {
    logo: Option<String>, // Optional logo URL
    name: String,         // Name of the collection
    description: String,  // Description of the collection
}

/// Combines metadata with an ID and collection owner information.
#[derive(Clone)]
struct FormMetadataWithId {
    metadata: Option<Vec<Metadata>>, // Optional list of metadata entries
    collection_owner: String,        // Owner of the collection
    id: u32,                         // Unique identifier
}

/// A Leptos component representing the collection list page.
#[component]
pub fn CollectionListPage() -> impl IntoView {
    let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
        .expect("Canisters ReadWriteSignal must be provided");

    let pending_requests = create_resource(
        move || canisters_signal.get().clone(),
        move |cans_option| async move {
            if let Some(cans) = cans_option {
                log::info!("Fetching pending requests data.");
                match fetch_pending_requests_data(&cans).await {
                    Ok(data) => {
                        log::info!("Successfully fetched pending requests data.");
                        logging::log!("data: {:?}", data);
                        Ok(data)
                    }
                    Err(e) => {
                        log::error!("Error fetching pending requests data: {}", e);
                        Err(e)
                    }
                }
            } else {
                log::warn!("Canisters not available. User needs to log in.");
                Err("Canisters not available. Please log in.".to_string())
            }
        },
    );

    view! {
        <Header2 />
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold mb-4">"Pending Requests"</h1>
            <Suspense fallback=move || {
                view! { <div>"Loading pending requests..."</div> }
            }>
                {move || match pending_requests.get() {
                    Some(Ok(requests)) => {
                        logging::log!("requests: {:?}", requests);
                        let filtered_requests = requests
                            .iter()
                            .filter(|request| { true })
                            .collect::<Vec<_>>();
                        view! {
                            // Add your filtering logic here, if necessary.
                            <div>
                                <h1 class="text-2xl font-bold mb-4">"Pending Requests"</h1>

                                <ul role="list" class="divide-y divide-gray-100">
                                    {filtered_requests
                                        .into_iter()
                                        .map(|request| {
                                            logging::log!("request: {:?}", request);
                                            view! {
                                                <li class="py-4">
                                                    <div class="flex justify-between items-center">
                                                        <a href=format!("/admin/manage/{}", request.collection_id)>
                                                            <div>
                                                                <p class="text-sm font-semibold">{request.name.clone()}</p>
                                                                // {request.description.clone()}
                                                                <p class="text-xs text-gray-500"></p>
                                                            </div>
                                                        </a>
                                                    // <button
                                                    // class="px-4 py-2 bg-blue-500 text-white rounded"
                                                    // on:click=move |_| {}
                                                    // >
                                                    // "Approve"
                                                    // </button>
                                                    </div>
                                                </li>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </ul>
                            </div>
                        }
                    }
                    Some(Err(e)) => {
                        view! { <div>{format!("Error fetching pending requests: {}", e)}</div> }
                    }
                    None => {
                        view! {
                            <div>
                                <div class="container mx-auto p-4">
                                    <h1 class="text-2xl font-bold mb-4">"Pending Requests: "</h1>
                                </div>
                                <div>"Loading..."</div>
                            </div>
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}
