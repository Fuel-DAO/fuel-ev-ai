use crate::state::{
    auth::AuthService,
    auth_actions::{create_login_action, create_logout_action},
};
use leptos::*;
use leptos_dom::logging::{console_error, console_log};
use std::cell::RefCell;
use std::rc::Rc;

// Assuming UserPrincipal is defined in the same module or appropriately imported
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

    // Use the reusable actions from auth_actions.rs
    let handle_login = create_login_action(Rc::clone(&auth_service));
    let handle_logout = create_logout_action(Rc::clone(&auth_service));

    view! {
        <Show
            when=move || is_authenticated()
            fallback=move || {
                view! {
                    <button
                        on:click=move |_| handle_login.dispatch(())
                        class="h-10 w-10 bg-black flex items-center text-xl select-none justify-center font-light text-white rounded-full uppercase"
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
                    </button>
                }
            }
        >
            <div class="flex items-center space-x-2">
                <div>{"user: "}{move || principal().map(|p| p.to_text()).unwrap_or_default()}</div>

                <button
                    on:click=move |_| handle_logout.dispatch(())
                    class="bg-red-500 text-white rounded-full p-2"
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
                            d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15M12 9l-3 3m0 0l3 3m-3-3h12.75"
                        />
                    </svg>
                </button>
            </div>
        </Show>
    }
}

#[component]
pub fn Header2() -> impl IntoView {
    view! {
        <div class="w-full fixed z-50 h-20 shadow-sm flex items-center justify-between px-8 font-light transition-all bg-white/90 backdrop-blur-md">

            <button class="z-[1] hidden lg:block text-gray-700 hover:text-gray-900">
                "← Go back"
            </button>

            <button class="z-[1] lg:hidden pr-4 text-gray-700 hover:text-gray-900">"←"</button>

            <div class="absolute z-0 inset-x-0 flex items-center pl-8 justify-center">
                <a href="/">
                    <img src="/public/img/fueldao.svg" alt="FuelDAO" class="lg:h-8 h-5" />
                </a>
            </div>

            <div class="absolute z-[1] lg:flex hidden right-8 items-center gap-8">
                <a href="/collections">"Collections"</a>
                <UserPrincipal />
            </div>
        </div>
    }
}
