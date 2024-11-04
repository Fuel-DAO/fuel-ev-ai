extern crate console_error_panic_hook;
use crate::state::auth::AuthService;
use crate::state::canisters::Canisters;
use crate::stores::{agent::AgentProvider, auth_client::AuthClientProvider};
use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};
use pages::{collection_detail::CollectionDetail, collections::Collections, home::HomePage};
use std::cell::RefCell;
use std::rc::Rc;
mod canister;
mod components;
mod consts;
mod outbound;
mod pages;
mod state;
mod stores;
mod utils;
#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/collections" view=Collections />
                    <Route path="/collections/:token_id/:asset_id" view=CollectionDetail />

                </Routes>
            </main>
        </Router>
    }
}
#[component]
fn AuthServiceProvider(children: Children) -> impl IntoView {
    // Initialize AuthService and handle potential errors
    let auth_service = AuthService::new().expect("Failed to initialize AuthService");

    // Wrap AuthService in Rc<RefCell<>> for shared ownership and interior mutability
    let auth_service_rc = Rc::new(RefCell::new(auth_service));

    // Provide the Rc<RefCell<AuthService>> to the context
    provide_context(auth_service_rc.clone());

    // Render child components
    children()
}
#[component]
fn Providers() -> impl IntoView {
    provide_meta_context();
    provide_context(Canisters::default());
    console_error_panic_hook::set_once();
    view! {
        <AuthClientProvider>
            <AgentProvider>
                <AuthServiceProvider>
                    <App />
                </AuthServiceProvider>
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
