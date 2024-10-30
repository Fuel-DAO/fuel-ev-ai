use crate::stores::auth_client::login;
use ic_auth_client::AuthClient;
use leptos::*;
use leptos_dom::logging::{console_error, console_log};

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
    let auth_client = use_context::<ReadSignal<Option<AuthClient>>>().unwrap();
    let principal = move || {
        auth_client
            .get()
            .map(|f| match f.is_authenticated() {
                true => f.identity().sender().ok(),
                false => None,
            })
            .flatten()
    };

    
    view! {
        <Show
            when=move || principal().is_some()
            fallback=move || {
                view! {
                    <button
                        on:click=move |_| {
                            match login() {
                                Ok(_) => console_log("Started login process."),
                                Err(e) => {
                                    console_error(
                                        &format!("Failed to start login process: {:?}", e),
                                    )
                                }
                            }
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
            <div>{principal().unwrap().to_text()}</div>
        </Show>
    }
}

