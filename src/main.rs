extern crate console_error_panic_hook;

use crate::stores::{agent::AgentProvider, auth_client::AuthClientProvider};
use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};
use pages::{collection_detail::CollectionDetail, collections::Collections, home::HomePage};

mod canister;
mod components;
mod pages;
mod stores;
#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/collections" view=Collections />
                    <Route path="/collections/:id" view=CollectionDetail />

                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Providers() -> impl IntoView {
    provide_meta_context();

    view! {
        <AuthClientProvider>
            <AgentProvider>
                <App />
            </AgentProvider>
        </AuthClientProvider>
    }
}

fn main() {
    if leptos_dom::is_dev() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    mount_to_body(|| view! { <Providers /> });
}

