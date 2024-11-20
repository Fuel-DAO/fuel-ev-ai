extern crate console_error_panic_hook;
use crate::state::auth::AuthService;
use crate::state::canisters::Canisters;
use crate::stores::{agent::AgentProvider, auth_client::AuthClientProvider};
use leptos::*;
use leptos_dom::logging::{console_error, console_log};
use leptos_meta::*;

use leptos_router::{Route, Router, Routes};
use pages::{
    admin::{
        auth::AdminComponent, collection_list::CollectionListPage,
        new_collection::NewCollectionForm,
        manage_collection::ManageCollectionPage
    },
    collection_detail::CollectionDetail,
    collections::Collections,
    home::HomePage,
};
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
                    <Route path="/admin" view=AdminComponent />
                    <Route path="/admin/new-collection" view=NewCollectionForm />
                    <Route path="/admin/manage/list" view=CollectionListPage />
                    <Route path="/admin/manage/:id" view=ManageCollectionPage />
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
    provide_context(auth_service.clone());

    let canisters_signal = create_rw_signal(None);
    provide_context(canisters_signal);

    spawn_local({
        let auth_service = auth_service.clone();
        async move {
            match Canisters::new(auth_service).await {
                Ok(canisters_instance) => {
                    canisters_signal.set(Some(Rc::new(canisters_instance)));
                }
                Err(e) => console_error(&format!("Failed to create Canisters: {:?}", e)),
            }
        }
    });

    // Provide AuthService as a context
    children()
}
#[component]
fn Providers() -> impl IntoView {
    provide_meta_context();
    // provide_context(Canisters::default());

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
