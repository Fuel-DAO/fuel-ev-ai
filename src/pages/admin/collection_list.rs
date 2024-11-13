use crate::components::header2::Header2;
use leptos::*;
use log;
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
    // ==== State Variables ====

    // Create a signal holding a vector of FormMetadataWithId with static sample data.
    let (collections, _) = create_signal(vec![
        FormMetadataWithId {
            metadata: Some(vec![Metadata {
                logo: Some("https://via.placeholder.com/48".to_string()), // Sample logo URL
                name: "First Collection".to_string(),
                description: "Description for the first collection.".to_string(),
            }]),
            collection_owner: "Alice Smith".to_string(),
            id: 101,
        },
        FormMetadataWithId {
            metadata: Some(vec![Metadata {
                logo: None, // No logo provided
                name: "Second Collection".to_string(),
                description: "Description for the second collection.".to_string(),
            }]),
            collection_owner: "Bob Johnson".to_string(),
            id: 102,
        },
        // Add more items as needed
    ]);

    // ==== Event Handlers ====

    // /// Function to navigate back in browser history.
    // let go_back = move |_| {
    //     if let Some(history) = window().and_then(|w| w.history().ok()) {
    //         let _ = history.back();
    //     }
    // };
    //
    // /// Function to handle form submission (simulated).
    // let submit_form = move |_| {
    //     // Implement form submission logic here.
    //     // For this example, we'll just log to the console.
    //     log::info!("Form submitted!");
    // };

    // ==== View Rendering ====

    view! {
        <Header2 />
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold mb-4">"Collection List"</h1>
            <ul role="list" class="divide-y divide-gray-100">
                <For each=collections key=|collection| collection.id let:collection>
                    {
                        let collection = collection.clone();
                        let metadata = collection.metadata.as_ref().and_then(|m| m.get(0));
                        let logo_view = if let Some(metadata) = metadata {
                            if let Some(logo_url) = &metadata.logo {
                                view! {
                                    // If needed, clone `collection` to avoid borrowing issues

                                    <div>
                                        <img
                                            class="h-12 w-12 flex-none rounded-full bg-gray-50 object-cover"
                                            src=logo_url.clone()
                                            alt=""
                                        />
                                    </div>
                                }
                            } else {
                                view! { <div class="h-12 w-12 bg-gray-200 rounded-full" /> }
                            }
                        } else {
                            view! { <div class="h-12 w-12 bg-gray-200 rounded-full" /> }
                        };
                        let name = metadata.map_or("Unknown Name".to_string(), |m| m.name.clone());
                        let description = metadata
                            .map_or(
                                "No description available.".to_string(),
                                |m| m.description.clone(),
                            );
                        view! {
                            // Get name and description

                            <li>
                                <a
                                    href=format!("/admin/manage/{}", collection.id)
                                    class="flex justify-between gap-x-6 py-5"
                                >
                                    <div class="flex min-w-0 gap-x-4">
                                        {logo_view} <div class="min-w-0 flex-auto">
                                            <p class="text-sm font-semibold leading-6 text-gray-900">
                                                {name}
                                            </p>
                                            <p class="mt-1 truncate line-clamp-2 text-xs leading-5 text-gray-500">
                                                {description}
                                            </p>
                                        </div>
                                    </div>
                                    <div class="hidden shrink-0 sm:flex sm:flex-col sm:items-end">
                                        <p class="text-sm leading-6 text-gray-900">
                                            {format!("Submitted by: {}", collection.collection_owner)}
                                        </p>
                                        <p class="mt-1 text-xs leading-5 text-gray-500">
                                            // Static date
                                            {format!("Created at {}", "Wed Sep 27 2023")}
                                        </p>
                                    </div>
                                </a>
                            </li>
                        }
                    }
                </For>
            </ul>
            // <button on:click=go_back class="px-4 py-2 bg-gray-300 rounded">
            // "Cancel"
            <div class="mt-4 flex justify-end gap-x-4">// </button>
            // <button on:click=submit_form class="px-4 py-2 bg-blue-500 text-white rounded">
            // "Save"
            // </button>
            </div>
        </div>
    }
}
