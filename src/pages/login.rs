// src/components/login.rs

use crate::{pages::admin::check_admin::AdminRoute, state::{
    auth::AuthService,
    auth_actions::{create_login_action, create_logout_action},
}};
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;

/// The Login component handles user authentication.
/// It displays login options when the user is not authenticated
/// and shows user information along with logout options when authenticated.
#[component]
pub fn Login() -> impl IntoView {
    // Obtain AuthService from the context
    let auth_service =
        use_context::<Rc<RefCell<AuthService>>>().expect("AuthService context must be provided");

    // Reactive signal to track authentication state
    let is_authenticated = create_memo({
        let auth_service = Rc::clone(&auth_service);
        move |_| auth_service.borrow().is_authenticated()
    });

    // Reactive signal to track the user's principal ID
    let principal = create_memo({
        let auth_service = Rc::clone(&auth_service);
        move |_| {
            if is_authenticated() {
                auth_service.borrow().get_principal().ok()
            } else {
                None
            }
        }
    });

    // Create login and logout actions
    let handle_login = create_login_action(Rc::clone(&auth_service));
    let handle_logout = create_logout_action(Rc::clone(&auth_service));

    view! {
        <div class="flex flex-col overflow-hidden h-screen w-full items-center justify-center pb-20 gap-4 relative">
            <div class="flex z-3 min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
                <div class="bg-white/75 backdrop-blur-xl flex flex-col items-center gap-8 px-6 py-12 shadow-md rounded-lg sm:px-12 ">
                    <a href="/" class="flex justify-center items-center ">
                        <img src="/public/img/app.svg" alt="Fuel DAO Logo" />
                    // class="h-full w-full"
                    </a>

                    <Show
                        when=move || is_authenticated()
                        fallback=move || {
                            view! {
                                <>
                                    <h2 class="text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
                                        "Sign in or join"
                                    </h2>

                                    <button
                                        role="presentation"
                                        type="button"
                                        on:click=move |_| { handle_login.dispatch(()) }
                                        class="bg-white ring-1 ring-inset ring-gray-100 hover:bg-gray-50 outline-none active:bg-gray-200 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 w-min"
                                    >
                                        <div class="flex items-center justify-center gap-2">
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="none"
                                                viewBox="0 0 197 97"
                                                class="h-4 w-4"
                                            >
                                                <path
                                                    fill="#F15A24"
                                                    d="M148.7.4c-10.9 0-22.8 5.8-35.4 17.3-6 5.4-11.2 11.2-15 15.9l12.8 14.3c3.6-4.5 8.9-10.6 14.9-16.1 11.2-10.2 18.5-12.3 22.7-12.3a28.9 28.9 0 1 1 0 57.8L146 77a35 35 0 0 0 14.2 3.6c28.8 0 34.4-19.5 34.8-20.9A48 48 0 0 0 148.7.4Z"
                                                ></path>
                                                <path
                                                    fill="#ED1E79"
                                                    d="M48.1 96.3c10.9 0 22.8-5.8 35.4-17.3 6-5.4 11.2-11.2 15-15.9L85.7 48.8c-3.6 4.5-8.9 10.6-14.9 16.1-11.2 10.2-18.5 12.3-22.7 12.3a28.9 28.9 0 1 1 0-57.8l2.7.3a35 35 0 0 0-14.2-3.6C7.8 16.1 2.2 35.6 1.8 37a48 48 0 0 0 46.3 59.3Z"
                                                ></path>
                                                <path
                                                    fill="#29ABE2"
                                                    d="M70 32.2c-3.1-3-18.5-15.2-33.3-15.6-26.3-.6-34 18-34.6 20.5C7.1 16.1 25.9.5 48.2.4c18.2 0 36.5 17.4 50.1 33.2l.1-.1 12.8 14.3s7.6 8.8 15.7 16.6c3.1 3 18.5 15.1 33.2 15.5 27 .7 34.4-19 34.9-20.5a47.8 47.8 0 0 1-46.1 36.9c-18.2 0-36.5-17.5-50.2-33.2l-.1.1-12.8-14.3c-.1-.1-7.7-8.9-15.8-16.7Zm-68 5V37c.1.1.1.2 0 .2Z"
                                                ></path>
                                            </svg>
                                            <span class="whitespace-nowrap">
                                                "Login using internet identity"
                                            </span>
                                        </div>
                                    </button>

                                    <p class="text-center text-sm text-gray-500 max-w-xs">
                                        "By creating an account you agree to our "
                                        <a
                                            href="/tc"
                                            class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500"
                                        >
                                            "Terms and conditions"
                                        </a>
                                    </p>
                                </>
                            }
                        }
                    >
                        <>
                            <h2 class="text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
                                "You are logged in!"
                            </h2>
                            <div class="text-center text-sm text-gray-500">
                                <div class="pb-2 text-base">Your principal ID:</div>
                                <div class=" font-mono bg-gray-200 p-2 rounded-md max-w-sm select-all leading-relaxed text-pretty">

                                    {move || principal().map(|p| p.to_text()).unwrap_or_default()}
                                </div>
                            </div>

                            <a href="/collections" class="w-min">
                                <button
                                    role="presentation"
                                    type="button"
                                    class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 w-min"
                                >
                                    "View all collections"
                                </button>
                            </a>
                            <div class="flex items-center justify-evenly gap-2">
                            <button
                                role="presentation"
                                type="button"
                                on:click=move |_| { handle_logout.dispatch(()) }
                                class="bg-white ring-1 ring-inset ring-gray-100 hover:bg-gray-50 outline-none active:bg-gray-200 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 w-min mt-2"
                            >
                                <div class="flex items-center justify-center gap-2">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="2"
                                        stroke="currentColor"
                                        class="h-4 w-4"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15M12 9l-3 3m0 0l3 3m-3-3h12.75"
                                        />
                                    </svg>
                                    <span class="whitespace-nowrap">"Logout"</span>
                                    
                                </div>
                            </button>
                            
                                // <div class="flex items-center justify-center gap-2">
                                //     <AdminRoute />
                                // </div>
                            </div>
                        </>
                    </Show>
                </div>
            </div>
        </div>
    }
}
