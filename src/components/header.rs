use crate::state::auth::AuthService;
use crate::{
    outbound::collection_canister_calls::fetch_collections_data, state::canisters::Canisters,
};
use codee::string::FromToStringCodec;
use futures::executor::block_on;
use ic_auth_client::AuthClient;
use leptos::ev::MouseEvent;
use leptos::*;
use leptos_dom::logging::{console_error, console_log};
use leptos_use::storage::use_local_storage;
use std::cell::RefCell;
use std::rc::Rc;
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="w-full fixed z-50 h-20 shadow-sm flex items-center justify-between px-8 font-light transition-all bg-white/90 backdrop-blur-md">
            // Logo Section
            <div class="flex items-center space-x-2">
                <a href="/">
                    <img src="/public/img/fueldao.svg" alt="Fuel DAO Logo" class="h-8" />
                </a>
            </div>

            // Collections and Profile
            <div class="absolute z-[1] lg:flex hidden right-8 items-center gap-8">
                <a href="/collections">

                    <span class="text-black font-medium">"Collections"</span>
                </a>
                <UserPrincipal />
            // <button class="bg-black text-white rounded-full p-2">
            // <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            // <path stroke-linecap="round" stroke-linejoin="round" d="M12 14l9-5-9-5-9 5 9 5zm0 7l9-5-9-5-9 5 9 5z"/>
            // </svg>
            // </button>
            </div>
        </div>
    }
}

#[component]
fn UserPrincipal() -> impl IntoView {
    // Consume the AuthService context as Rc<RefCell<AuthService>>
    let auth_service =
        use_context::<Rc<RefCell<AuthService>>>().expect("AuthService context must be provided");

    let (principal_id, set_principal_id, _) =
        use_local_storage::<String, FromToStringCodec>("user-principal-id");

    let (canisters, set_canisters) = create_signal::<Option<Rc<Canisters>>>(None);

    let handle_login = {
        let auth_service = Rc::clone(&auth_service);

        move |_: MouseEvent| {
            let auth_service = Rc::clone(&auth_service);

            // Spawn the asynchronous login process
            spawn_local(async move {
                let result = auth_service.borrow_mut().login().await;
                match result {
                    Ok(_) => {
                        log::info!("Login successful.");

                        if let Ok(principal) = auth_service.borrow().get_principal() {
                            set_principal_id(principal.to_text());
                        }

                        // Now create Canisters
                        match Canisters::new(auth_service.clone()) {
                            Ok(cans) => {
                                let cans_rc = Rc::new(cans);
                                set_canisters(Some(cans_rc.clone()));
                                provide_context(canisters);
                            }
                            Err(e) => {
                                log::error!("Failed to create Canisters: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to start login process: {:?}", e);
                    }
                }
            });
        }
    };

    view! {
        <Show
            when=move || !principal_id.get().is_empty()
            fallback=move || {
                let handle_login = handle_login.clone();
                view! {
                    // Define a new closure for the fallback to satisfy Fn
                    <button on:click=handle_login class="bg-black text-white rounded-full p-2">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class=""
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z"
                            />
                        </svg>
                    </button>
                }
            }
        >
            <div>{principal_id.get()}</div>
        </Show>
    }
}
