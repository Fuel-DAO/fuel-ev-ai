use crate::components::header2::Header2;
use crate::{
    outbound::get_pending_collection_requests::{
        fetch_pending_requests_data, CollectionData, CollectionId,
    },
    state::canisters::Canisters,
};
use leptos::*;
use leptos::*;
use log;
use std::rc::Rc;
use web_sys::window;
/// Represents the metadata of a form.

#[component]
pub fn CollectionListPage() -> impl IntoView {
    // Retrieve the canisters signal from the context
    let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
        .expect("Canisters ReadWriteSignal must be provided");

    // Create a resource to fetch pending requests data
    let pending_requests = create_resource(
        move || canisters_signal.get().clone(),
        move |cans_option| async move {
            if let Some(cans) = cans_option {
                log::info!("Fetching pending requests data.");
                match fetch_pending_requests_data(&cans).await {
                    Ok(data) => {
                        log::info!("Successfully fetched pending requests data.");
                        log::debug!("data: {:?}", data);
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

    // Render the component
    view! {
        <Header2 />
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold mb-4">"Pending Requests"</h1>
            <Suspense fallback=move || {
                view! { <div>"Loading pending requests..."</div> }
            }>
                {move || match pending_requests.get() {
                    Some(Ok(requests)) => {
                        log::debug!("requests: {:?}", requests);
                        let filtered_requests = requests
                            .iter()
                            .filter(|request| { true })
                            .collect::<Vec<_>>();
                        view! {
                            // Add your filtering logic here
                            <div>
                                <ul role="list" class="divide-y divide-gray-100 mx-12">
                                    {if !filtered_requests.is_empty() {
                                        filtered_requests
                                            .into_iter()
                                            .map(|request| {
                                                log::debug!("request: {:?}", request);
                                                let logo: Option<String> = request
                                                    .metadata
                                                    .as_ref()
                                                    .map(|meta| meta.logo.clone());
                                                let name = request.name.clone();
                                                let description = request
                                                    .metadata
                                                    .as_ref()
                                                    .map(|meta| meta.description.clone())
                                                    .unwrap_or_else(|| "No description provided.".to_string());
                                                let request_id_str = format!("{:?}", request.collection_id);
                                                view! {
                                                    // let collection_owner = request
                                                    // .metadata
                                                    // .as_ref()
                                                    // .map(|meta| meta.collection_owner.clone())
                                                    // .unwrap_or_else(|| "Unknown Owner".to_string());
                                                    // Returns Vec<HtmlElement<Li>>
                                                    // Extract collection_owner from metadata, assuming it's present
                                                    <li class="py-4">
                                                        <a
                                                            href=format!("/admin/manage/{}", request_id_str)
                                                            class="flex justify-between gap-x-6 py-5 hover:bg-gray-50"
                                                        >
                                                            <div class="flex min-w-0 gap-x-4">
                                                                <div>
                                                                    {if let Some(logo_url) = &logo {
                                                                        view! {
                                                                            // Render the logo image if available
                                                                            <div>
                                                                                <img
                                                                                    class="h-12 w-12 flex-none rounded-full bg-gray-50 object-cover"
                                                                                    src=logo_url.clone()
                                                                                    alt=name.clone()
                                                                                />
                                                                            </div>
                                                                        }
                                                                    } else {
                                                                        view! {
                                                                            // Render a placeholder if no logo is provided
                                                                            <div class="h-12 w-12 bg-gray-200 rounded-full" />
                                                                        }
                                                                    }}
                                                                </div>
                                                                <div class="min-w-0 flex-auto">
                                                                    <p class="text-sm font-semibold leading-6 text-gray-900">
                                                                        {name}
                                                                    </p>
                                                                    <p class="mt-1 truncate line-clamp-2 text-xs leading-5 text-gray-500">
                                                                        {description}
                                                                    </p>
                                                                </div>
                                                            </div>
                                                            <div class="hidden shrink-0 sm:flex sm:flex-col sm:items-end">
                                                                <p class="text-sm leading-6 text-gray-900">// {format!("Submitted by: {}", collection_owner)}
                                                                </p>
                                                            </div>
                                                        </a>
                                                    </li>
                                                }
                                            })
                                            .collect::<Vec<_>>()
                                    } else {
                                        vec![
                                            view! {
                                                // Return Vec<HtmlElement<Li>>
                                                <li class="py-5 text-center text-gray-500">
                                                    "No items to show"
                                                </li>
                                            },
                                        ]
                                    }}
                                </ul>
                            </div>
                        }
                    }
                    Some(Err(e)) => {
                        view! {
                            // Display error message within a <div>
                            <div>{format!("Error fetching pending requests: {}", e)}</div>
                        }
                    }
                    None => {
                        view! {
                            // Display loading state within a <div>
                            <div>
                                <div class="container mx-auto p-4">
                                    <h1 class="text-2xl font-bold mb-4">"Pending Requests"</h1>
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
