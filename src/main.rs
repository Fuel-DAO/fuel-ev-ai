extern crate console_error_panic_hook;
use crate::state::auth::AuthService;
use crate::state::canisters::Canisters;
mod time;
use crate::stores::{agent::AgentProvider, auth_client::AuthClientProvider};
use leptos::leptos_dom::logging::console_error;
use leptos::prelude::*;
use leptoaster::*;
use leptos::task::spawn_local;
use leptos_meta::*;

use leptos_router::components::{Route, Router, Routes};
use pages::admin::check_admin::AdminProvider;
use pages::investors_business::InvestorsBookingDashboard;
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
use state::sale_status::SaleStatusState;
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
    provide_toaster();
    // provide_context(Canisters::default());
    view! {
        <Toaster />
        <AdminProvider>
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/business" view=InvestorsBookingDashboard />
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
    SaleStatusState::set_global();
    // Provide AuthService as a context
    children()
}

pub fn set_up_auth_context() {
    let auth_service = AuthService::new().expect("Failed to create AuthService");


    spawn_local({
        let auth_service = auth_service.clone();
        async move {
            match Canisters::new(auth_service).await {
                Ok(canisters_instance) => {
                    Canisters::set_global(canisters_instance);
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
