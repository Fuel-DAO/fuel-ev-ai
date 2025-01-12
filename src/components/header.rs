use crate::{
    pages::admin::check_admin::AdminRoute,
    state::{
        auth::AuthService,
        auth_actions::{create_login_action, create_logout_action},
    },
};
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
#[component]
pub fn Header() -> impl IntoView {
    let (menu_open, set_menu_open) = create_signal(false);

    view! {
        <div class="w-full fixed z-50 h-20 shadow-sm flex items-center justify-between px-8 font-light transition-all bg-white/90 backdrop-blur-md">
            // Logo Section
            <div class="flex items-center justify-between space-x-2">
                <a href="/">
                    <img src="/public/img/app.svg" alt="Fuel DAO Logo" class="h-8" />
                </a>
            </div>

            // Hamburger Button
            <div class="lg:hidden flex  gap-2 items-center justify-end">
            <UserPrincipal />

            <button
                class=" text-black rounded-full h-8 "
                on:click=move |_| set_menu_open.update(|open| *open = !*open)
            >
                {move || {
                    if menu_open() {
                        view! {
                            // Close icon when the menu is open
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
                                    d="M6 18L18 6M6 6l12 12"
                                />
                            </svg>
                        }
                    } else {
                        view! {
                            // Hamburger icon when the menu is closed
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
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                            </svg>
                        }
                    }
                }}
            </button>
            </div>

            // Drawer Menu
            <div
                class=move || {
                    format!(
                        "fixed top-0 right-0 h-full w-64 bg-white shadow-lg z-40 transition-transform transform {}",
                        if menu_open.get() { "translate-x-0" } else { "translate-x-full" }
                    )
                }
            >
                <div class="flex flex-col h-full p-6 space-y-6 h-screen bg-white">
                    <button
                        class="self-end bg-gray-200 p-2 rounded-full"
                        on:click=move |_| set_menu_open.set(false)
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
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                    </button>
                    <div class="flex flex-col items-start space-y-4">
    <TrailingButton />
</div>
                </div>
            </div>

            // Overlay (optional)
            {move || if menu_open() {
                view! {
                    <div
                        class="fixed inset-0 bg-black bg-opacity-50 z-30"
                        on:click=move |_| set_menu_open.set(false)
                    ></div>
                }
            } else {
                view! { <div class="hidden lg:flex gap-8 items-center">
                    </div> }
            }}





            // Desktop Navigation
            <div class="hidden lg:flex gap-8 items-center">
                <TrailingButton />
                <UserPrincipal />

            </div>
        </div>
    }
}

#[component]
fn TrailingButton() -> impl IntoView {
    view! {
        <AdminRoute />

        <a href="https://fuelev.in" target="_blank">
            <span class="text-black font-medium">EV Rentals</span>
        </a>
        <a href="/collections">
            <span class="text-black font-medium">Fleet Investments</span>
        </a>
    }
}


#[component]
fn UserPrincipal() -> impl IntoView {
    // Obtain AuthService from the context
    let auth_service =
        use_context::<Rc<RefCell<AuthService>>>().expect("AuthService context must be provided");

    // Reactive signal for authentication state
    let is_authenticated = create_memo({
        let auth_service = Rc::clone(&auth_service);
        move |_| auth_service.borrow().is_authenticated()
    });

    // Reactive signal for principal
    let _ = create_memo({
        let auth_service = Rc::clone(&auth_service);
        move |_| {
            if is_authenticated() {
                auth_service.borrow().get_principal().ok()
            } else {
                None
            }
        }
    });

    // Use the reusable actions from auth_actions.rs
    let _handle_login = create_login_action(Rc::clone(&auth_service));
    let _handle_logout = create_logout_action(Rc::clone(&auth_service));
    view! {
        <Show
            when=move || is_authenticated()
            fallback=move || {
                view! {
                    <a
                        href="/login"
                        class="bg-black text-white rounded-full p-2"
                    >

                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="h-4 w-4"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z"
                            ></path>
                        </svg>
                    </a>
                }
            }
        >

            <div class="flex items-center space-x-2">

                <a
                    href="/login"
                    class="h-10 w-10 bg-black flex items-center text-xl select-none justify-center font-light text-white rounded-full uppercase"
                >

                    U
                </a>
            </div>
        </Show>
    }
}
