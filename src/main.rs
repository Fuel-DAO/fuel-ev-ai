extern crate console_error_panic_hook;
use crate::state::auth::AuthService;
use crate::state::canisters::Canisters;
use crate::stores::{agent::AgentProvider, auth_client::AuthClientProvider};
use leptos::*;
use leptos_dom::logging::console_error;
use leptos_meta::*;

use leptos_router::{Route, Router, Routes, ProtectedRoute};
use pages::admin::check_admin::AdminProvider;
use pages::{
    admin::{
        auth::AdminComponent, collection_list::CollectionListPage,
        manage_collection::ManageCollectionPage, new_collection::NewCollectionForm,
    },
    collection_detail::CollectionDetail,
    collections::Collections,
    home::HomePage,
    login::Login,
};
pub mod constants;
pub use constants::TEMP_ASSET_CANISTER_ID;
use state::admin::Admin;
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
    // provide_context(Canisters::default());
    view! {
        <AdminProvider>
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/login" view=Login />

                    <Route path="/collections" view=Collections />
                    <Route path="/collections/:token_id/:asset_id" view=CollectionDetail />
                    // <ProtectedRoute 
                    //     path="/admin" 
                    //     redirect_path="/login"
                    //     condition=move || (Admin::get().is_admin)()
                    //     view=AdminComponent />
                    <Route path="/admin" view=move || view! {
                        <Show when=move || (Admin::get().is_admin)() fallback=Login>
                        <AdminComponent />
                        </Show>
                    } />
                    <Route path="/admin/new-collection" view=move || view! {
                        <Show when=move || (Admin::get().is_admin)() fallback=Login>
                        <NewCollectionForm />
                        </Show>
                    } />
                    <Route path="/admin/manage/list" view=move || view! {
                        <Show when=move || (Admin::get().is_admin)() fallback=Login>
                        <CollectionListPage />
                        </Show>
                    } />
                    <Route path="/admin/manage/:id" view=move || view! {
                        <Show when=move || (Admin::get().is_admin)() fallback=Login>
                        <ManageCollectionPage />
                        </Show>
                    } />
                    
                </Routes>
            </main>
        </Router>
        </AdminProvider>
    }
}
#[component]
fn AuthServiceProvider(children: Children) -> impl IntoView {
    set_up_auth_context();
    Admin::set_global();
    // Provide AuthService as a context
    children()
}

pub fn set_up_auth_context() {
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
}
#[component]
fn Providers() -> impl IntoView {
    provide_meta_context();
    Admin::set_global();
    // provide_context(Canisters::default());

    console_error_panic_hook::set_once();
    view! {
        <>
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
