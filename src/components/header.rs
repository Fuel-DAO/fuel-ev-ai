use crate::state::auth::AuthService;
use codee::string::FromToStringCodec;
use futures::executor::block_on;
use ic_auth_client::AuthClient;
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

    // Clone once for use in the `when` closure and once for the `fallback` closure
    let cloned_auth_service = auth_service.clone();
    let fallback_auth_service = auth_service.clone();
    let (principal_id, set_principal_id, _) =
        use_local_storage::<String, FromToStringCodec>("user-principal-id");

    // Reactive signal to store the principal

    view! {
        <Show
            when=move || {
                let principal_opt = cloned_auth_service.borrow().get_principal();
                log::info!(" {}", &format!("Principal fetched: {:?}", principal_opt));
                principal_opt.is_ok()
            }
            fallback=move || {
                let auth_service = fallback_auth_service.clone();
                view! {
                    <button
                        on:click=move |_| {
                            let auth_service = auth_service.clone();
                            spawn_local(async move {
                                let result = block_on(auth_service.borrow_mut().login());
                                match result {
                                    Ok(_) => log::info!("Started login process."),
                                    Err(e) => {
                                        log::info!(
                                            " {}",
                                            &format!("Failed to start login process: {:?}", e),
                                        )
                                    }
                                }
                            });
                        }
                        class="bg-black text-white rounded-full p-2"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M12 14l9-5-9-5-9 5 9 5zm0 7l9-5-9-5-9 5 9 5z"
                            />
                        </svg>
                    </button>
                }
            }
        >
            <div>

                {principal_id.get()}
            </div>
        </Show>
    }
}
