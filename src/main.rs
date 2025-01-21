extern crate console_error_panic_hook;
use crate::state::auth::AuthService;
use crate::state::canisters::Canisters;
mod time;
use crate::stores::auth_client::AuthClientProvider;
use leptoaster::*;
use leptos::task::spawn_local;
use leptos::{logging, prelude::*};
use leptos_meta::*;

use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;
use pages::admin::auth::AdminComponent;
use pages::admin::check_admin::AdminProvider;
use pages::admin::collection_list::CollectionListPage;
use pages::admin::manage_collection::ManageCollectionPage;
use pages::admin::new_collection::NewCollectionForm;
use pages::collection_detail::CollectionDetail;
use pages::collections::Collections;
use pages::investors_business::InvestorsBookingDashboard;
use pages::{home::HomePage, login::Login};
pub mod constants;
pub use constants::TEMP_ASSET_CANISTER_ID;
use state::admin::Admin;
use state::auth_actions::send_wrap;
use state::sale_status::SaleStatusState;
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
            <Routes fallback=|| "Page not found.".into_view()>
            // <ParentRoute path=path!("") view=AuthServiceProvider/ >
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/business") view=InvestorsBookingDashboard />
                    <Route path=path!("/login") view=Login />

                    <Route path=path!("/collections") view=Collections />
                    <Route path=path!("/collections/:token_id/:asset_id") view=CollectionDetail />
                    // <ProtectedRoute
                    //     path="/admin"
                    //     redirect_path="/login"
                    //     condition=move || (Admin::get().is_admin)()
                    //     view=AdminComponent />
                    <Route path=path!("/admin") view=move || view! {
                        <Show when=move || (Admin::get().is_admin)() fallback=Login>
                        <AdminComponent />
                        </Show>
                    } />
                    <Route path=path!("/admin/new-collection") view=move || view! {
                        <Show when=move || (Admin::get().is_admin)() fallback=Login>
                        <NewCollectionForm />
                        </Show>
                    } />
                    <Route path=path!("/admin/manage/list") view=move || view! {
                        <Show when=move || (Admin::get().is_admin)() fallback=Login>
                        <CollectionListPage />
                        </Show>
                    } />
                    <Route path=path!("/admin/manage/:id") view=move || view! {
                        <Show when=move || (Admin::get().is_admin)() fallback=Login>
                        <ManageCollectionPage />
                        </Show>
                    } />

                    // </ ParentRoute>
                </Routes>
            </main>
        </Router>
        </AdminProvider>
    }
}
#[component]
fn AuthServiceProvider(children: Children) -> impl IntoView {
    provide_context(RwSignal::new(None::<Canisters>));


    let state = LocalResource::new( ||{
        
        send_wrap(async move {
            let auth_service = AuthService::new().expect("Failed to create AuthService");

            match Canisters::new(auth_service).await {
                Ok(canisters_instance) => {
                    // Canisters::set_global(canisters_instance);
                Canisters::set_global(canisters_instance);

                }
                Err(e) => logging::log!("Failed to create Canisters: {:?}", e),
            }
        })
    });
    Admin::set_global();
    SaleStatusState::set_global();
    // Provide AuthService as a context
    view! {
    <Suspense>
    {state.get().map(|_|{})}
    {children()}
    </Suspense>
    }
}

pub fn set_up_auth_context() {
    logging::log!("Setting up auth context");
    logging::log!("Got  auth service");
    provide_context(None::<RwSignal<Canisters>>);


    Resource::new_blocking(|| (), move|()|{
        
        send_wrap(async move {
            let auth_service = AuthService::new().expect("Failed to create AuthService");

            match Canisters::new(auth_service).await {
                Ok(canisters_instance) => {
                    // Canisters::set_global(canisters_instance);
                Canisters::set_global(canisters_instance);

                }
                Err(e) => logging::log!("Failed to create Canisters: {:?}", e),
            }
        })
    });
}
#[component]
fn Providers() -> impl IntoView {
    provide_meta_context();
    Admin::set_global();
    // provide_context(Canisters::default());

    console_error_panic_hook::set_once();
    view! {
        <AuthClientProvider>
                <AuthServiceProvider>
                    <App />
                </AuthServiceProvider>
        </AuthClientProvider>
    }
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    mount_to_body(|| view! { <Providers /> });
}
