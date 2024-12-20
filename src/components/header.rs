use crate::{pages::admin::check_admin::AdminRoute, state::{
    auth::AuthService,
    auth_actions::{create_login_action, create_logout_action},
}};
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="w-full fixed z-50 h-20 shadow-sm flex items-center justify-between px-8 font-light transition-all bg-white/90 backdrop-blur-md">
            // Logo Section
            <div class="flex items-center space-x-2">
                <a href="/">
                    <img src="/public/img/app.svg" alt="Fuel DAO Logo" class="h-8" />
                </a>
            </div>

            // Collections and Profile
            <div class="absolute z-[1] lg:flex hidden right-8 items-center gap-8">
                <AdminRoute />
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
                        // on:click=move |_| handle_login.dispatch(())
                        href="/login"
                        class="bg-black text-white rounded-full p-2"
                        // target="_blank"
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
            // <div>{"user: "}{move || principal().map(|p| p.to_text()).unwrap_or_default()}</div>

            <div class="flex items-center space-x-2">

                <a
                    // on:click=move |_| handle_logout.dispatch(())
                    href="/login"
                    class="h-10 w-10 bg-black flex items-center text-xl select-none justify-center font-light text-white rounded-full uppercase"
                    // target="_blank"
                >

                    U
                </a>
            </div>
        </Show>
    }
}
