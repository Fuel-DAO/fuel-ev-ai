use crate::components::header2::Header2;
use crate::{
    outbound::get_pending_collection_requests::fetch_pending_requests_data,
    state::canisters::Canisters,
};
use leptos::*;
use log;
use std::rc::Rc;

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

    view! {
        <Header2 />
        <div class="container mx-auto p-12">
            <h1 class="text-2xl font-bold mb-4">"Pending Requests"</h1>
            <Suspense fallback=move || {
                view! { <div>"Loading pending requests..."</div> }
            }>
                {move || {
                    match pending_requests.get() {
                        Some(Ok(requests)) => {
                            log::debug!("requests: {:?}", requests);
                            let filtered_requests = requests
                                .iter()
                                .filter(|_request| true)
                                .collect::<Vec<_>>();
                            view! {
                                // Add filtering logic if needed
                                <div>
                                    <ul role="list" class="divide-y divide-gray-100">
                                        {if !filtered_requests.is_empty() {
                                            filtered_requests
                                                .into_iter()
                                                .map(|request| {
                                                    log::debug!("request: {:?}", request);
                                                    let href = format!(
                                                        "/admin/manage/{}",
                                                        request.collection_id,
                                                    );
                                                    let logo = request
                                                        .metadata
                                                        .as_ref()
                                                        .and_then(|meta| {
                                                            if !meta.logo.is_empty() {
                                                                Some(meta.logo.clone())
                                                            } else {
                                                                None
                                                            }
                                                        });
                                                    let name = request
                                                        .metadata
                                                        .as_ref()
                                                        .map(|meta| meta.name.clone())
                                                        .unwrap_or_else(|| "Unnamed Collection".to_string());
                                                    let description = request
                                                        .metadata
                                                        .as_ref()
                                                        .map(|meta| meta.description.clone())
                                                        .unwrap_or_else(|| "No description provided.".to_string());
                                                    view! {
                                                        // let collection_owner = request
                                                        // .metadata
                                                        // .as_ref()
                                                        // .map(|meta| meta.collection_owner.clone())
                                                        // .unwrap_or_else(|| "Unknown Owner".to_string());
                                                        <li>
                                                            <a href=href class="flex justify-between gap-x-6 py-5">
                                                                <div class="flex min-w-0 gap-x-4">
                                                                    {if let Some(logo_url) = logo {
                                                                        view! {
                                                                            <div>
                                                                                <img
                                                                                    class="h-12 w-12 flex-none rounded-full bg-gray-50 object-cover"
                                                                                    src=logo_url
                                                                                    alt=""
                                                                                />
                                                                            </div>
                                                                        }
                                                                    } else {
                                                                        view! { <div class="h-12 w-12 bg-gray-200 rounded-full" /> }
                                                                    }} <div class="min-w-0 flex-auto">
                                                                        <p class="text-sm font-semibold leading-6 text-gray-900">
                                                                            {name.clone()}
                                                                        </p>
                                                                        <p class="mt-1 truncate line-clamp-2 text-xs leading-5 text-gray-500">
                                                                            {description.clone()}
                                                                        </p>
                                                                    </div>
                                                                </div>
                                                                <div class="hidden shrink-0 sm:flex sm:flex-col sm:items-end">
                                                                    // {format!("Submitted by: {}", collection_owner)}
                                                                    <p class="text-sm leading-6 text-gray-900"></p>
                                                                    <p class="mt-1 text-xs leading-5 text-gray-500">
                                                                        {format!(
                                                                            "Created at {}",
                                                                            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                                                                        )}
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
                            view! { <div>{format!("Error fetching pending requests: {}", e)}</div> }
                        }
                        None => {
                            view! {
                                <div>
                                    <div class="container mx-auto p-4">
                                        <h1 class="text-2xl font-bold mb-4">
                                            "Pending Requests: "
                                        </h1>
                                    </div>
                                    <div>"Loading..."</div>
                                </div>
                            }
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}
