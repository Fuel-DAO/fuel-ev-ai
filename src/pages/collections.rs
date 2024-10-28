use crate::state::canisters::{authenticated_canisters, CanistersAuthWire};
use candid::{Nat, Principal};
use leptos::*; // Adjust the path based on your project structure
use serde::{Deserialize, Serialize}; // Import serde traits

#[derive(Clone, PartialEq)]
enum Tab {
    All,
    Available,
    Upcoming,
}

// Define a structure for the token metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TokenMetadata {
    weight: f64,
    drive_type: String,
    purchase_price: Nat,
    token: Principal,
    // Add other fields based on the metadata record
    // ...
}

// Function to fetch metadata from the token canister
async fn fetch_token_metadata(cans: CanistersAuthWire) -> Result<TokenMetadata, String> {
    let backend = cans.canisters().map_err(|e| format!("Error: {:?}", e))?;
    let token_canister = backend.token_canister().await; // Now `token()` should exist

    // Call the get_metadata method
    let metadata_result = token_canister.get_metadata().await; // Adjust call syntax as needed
    match metadata_result {
        Ok(metadata) => {
            // Map the response to the TokenMetadata struct
            Ok(TokenMetadata {
                weight: metadata.weight,
                drive_type: metadata.drive_type,
                purchase_price: metadata.purchase_price,
                token: metadata.token,
                // Map other fields as needed
                // ...
            })
        }
        Err(e) => Err(format!("Error fetching token metadata: {:?}", e)),
    }
}

#[component]
pub fn Collections() -> impl IntoView {
    // Adjusted to retrieve `CanistersAuthWire` directly
    let cans_wire = use_context::<CanistersAuthWire>().expect("CanistersAuthWire not found");

    // Signal to track the selected tab
    let selected_tab = create_rw_signal(Tab::All);

    // Example data: You can replace this with your actual car data
    let cars = vec![
        (
            "Model S Plaid - SAMPLE",
            "The Tesla Model S Plaid is the epitome of electric luxury and performance...",
            "Live",
            "Available",
        ),
        ("Test EV Vehicle", "asdfasdsads", "Live", "Upcoming"),
        (
            "Tesla Electric Sample 1",
            "This is a test listing for Dfinity R&D Demo.",
            "Live",
            "Available",
        ),
    ];

    // Filter cars based on the selected tab
    let filtered_cars = move || match selected_tab.get() {
        Tab::All => cars.clone(),
        Tab::Available => cars
            .iter()
            .filter(|(_, _, _, status)| *status == "Available")
            .cloned()
            .collect(),
        Tab::Upcoming => cars
            .iter()
            .filter(|(_, _, _, status)| *status == "Upcoming")
            .cloned()
            .collect(),
    };

    // Create a resource to fetch the token metadata
    let token_metadata = create_resource(
        move || (), // Dependency: none in this case
        move |_| {
            let cans_wire = cans_wire.clone();
            async move { fetch_token_metadata(cans_wire).await }
        },
    );

    view! {
        <section class="p-6 bg-gray-100">
            // Top Filter Bar
            <div class="flex justify-between items-center mb-8">
                <div class="flex space-x-4">
                    <Tabs selected_tab tab=Tab::All label="All".to_string() />
                    <Tabs selected_tab tab=Tab::Available label="Available".to_string() />
                    <Tabs selected_tab tab=Tab::Upcoming label="Upcoming".to_string() />
                </div>

                // Sort Button (For future sorting functionality)
                <div>
                    <button class="flex items-center px-4 py-2 bg-white text-black rounded-full shadow-md font-medium">
                        "Sort"
                        <svg
                            class="w-4 h-4 ml-2"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M19 9l-7 7-7-7"
                            ></path>
                        </svg>
                    </button>
                </div>
            </div>

            // Card Grid
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                {move || {
                    filtered_cars()
                        .into_iter()
                        .map(|(title, description, status, _)| {
                            view! {
                                <a href=format!("/collections/{}", title) class="block">
                                    <div class="bg-white rounded-lg shadow-md overflow-hidden">
                                        <div class="relative">
                                            <img
                                                src="/public/img/car_image.jpg"
                                                alt=title
                                                class="w-full h-48 object-cover"
                                            />
                                            <span class="absolute top-2 left-2 bg-white text-black font-semibold text-xs px-2 py-1 rounded-full">
                                                {status}
                                            </span>
                                        </div>
                                        <div class="p-4">
                                            <h3 class="text-lg font-semibold">{title}</h3>
                                            <p class="text-sm text-gray-600">{description}</p>
                                        </div>
                                    </div>
                                </a>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>

            // Display Token Metadata
            <div class="p-4 bg-white rounded-lg shadow-md mt-8">
                <h2 class="text-xl font-semibold">"Token Metadata"</h2>
                <Suspense fallback=move || {
                    view! { <div>"Loading..."</div> }
                }>
                    {move || match token_metadata.get() {
                        Some(Ok(metadata)) => {
                            let metadata = metadata.clone();
                            view! {
                                <div>
                                    <p>
                                        <strong>"Weight: "</strong>
                                        {metadata.weight}
                                    </p>
                                    <p>
                                        <strong>"Drive Type: "</strong>
                                        {metadata.drive_type.clone()}
                                    </p>
                                    <p>
                                        <strong>"Purchase Price: "</strong>
                                        {metadata.purchase_price.to_string()}
                                    </p>
                                    <p>
                                        <strong>"Token: "</strong>
                                        {metadata.token.to_text()}
                                    </p>
                                </div>
                            }
                        }
                        Some(Err(e)) => {
                            view! {
                                <div>
                                    <p>{format!("Error fetching metadata: {}", e)}</p>
                                </div>
                            }
                        }
                        None => {
                            view! {
                                <div>
                                    <p>"Loading..."</p>
                                </div>
                            }
                        }
                    }}
                </Suspense>
            </div>
        </section>
    }
}

#[component]
fn Tabs(selected_tab: RwSignal<Tab>, tab: Tab, label: String) -> impl IntoView {
    let current = selected_tab.clone();
    let current_tab = tab.clone();
    view! {
        <button
            class=move || {
                format!(
                    "px-4 py-2 rounded-md shadow-md font-medium {}",
                    if selected_tab.get() == current_tab {
                        "bg-white text-black"
                    } else {
                        "text-gray-500"
                    },
                )
            }
            on:click=move |_| current.set(tab.clone())
        >
            {label}
        </button>
    }
}
