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
    let auth_service = Rc::new(RefCell::new(
        AuthService::new().expect("Failed to create AuthService"),
    ));

    // Provide AuthService as a context
    provide_context(auth_service.clone());
    children()
}
#[component]
fn Providers() -> impl IntoView {
    provide_meta_context();
    // provide_context(Canisters::default());
    let (canisters_signal, set_canisters) = create_signal::<Option<Rc<Canisters>>>(None);

    // Provide the ReadSignal and WriteSignal as contexts
    provide_context(canisters_signal); // ReadSignal<Option<Rc<Canisters>>>
    provide_context(set_canisters);
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
