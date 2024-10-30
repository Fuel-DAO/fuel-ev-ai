extern crate console_error_panic_hook;
use crate::error_template::{AppError, ErrorTemplate};
use crate::state::canisters::Canisters;

use crate::stores::{agent::AgentProvider, auth_client::AuthClientProvider};
use ic_agent::identity::BasicIdentity;
use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};
use pages::{collection_detail::CollectionDetail, collections::Collections, home::HomePage};
mod auth;
mod canister;
mod components;
mod consts;
mod error_template;
mod pages;
mod state;
mod stores;
mod utils;
#[component]
fn App() -> impl IntoView {
    view! {
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
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
    provide_context(Canisters::default());

    console_error_panic_hook::set_once();
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
