extern crate console_error_panic_hook;

use crate::{
    // components::account::{LoginButton, LogoutButton},
    stores::{agent::AgentProvider, auth_client::AuthClientProvider},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

mod components;
mod stores;
mod  canister;
mod pages;
use pages::{collections::Collections, home::HomePage};
#[component]
fn App() -> impl IntoView {
   

    view! {
        <Router >
            <main>
                <Routes>
                <Route path="/" view=HomePage />
                <Route path="/collections" view=Collections />

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