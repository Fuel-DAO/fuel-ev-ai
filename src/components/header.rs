use crate::state::auth::AuthService;
use leptos::*;
use leptos_dom::logging::{console_error, console_log};
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
    let auth_service =
        use_context::<Rc<RefCell<AuthService>>>().expect("AuthService context must be provided");

    let is_authenticated = create_memo({
        let auth_service = Rc::clone(&auth_service);
        move |_| auth_service.borrow().is_authenticated()
    });

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

    let handle_login = create_action({
        let auth_service = Rc::clone(&auth_service);
        move |_: &()| {
            let auth_service = Rc::clone(&auth_service);
            async move {
                match auth_service.borrow_mut().login().await {
                    Ok(_) => {
                        window().location().reload().unwrap();

                        console_log("Login successful.")
                    }
                    Err(e) => console_error(&format!("Login failed: {:?}", e)),
                }
            }
        }
    });
    let handle_logout = create_action({
        let auth_service = Rc::clone(&auth_service);
        move |_: &()| {
            let auth_service = Rc::clone(&auth_service);
            async move {
                match auth_service.borrow_mut().logout().await {
                    Ok(_) => console_log("Logout successful."),
                    Err(e) => console_error(&format!("Logout failed: {:?}", e)),
                }
            }
        }
    });
    view! {
        <Show
            when=move || is_authenticated()
            fallback=move || {
                view! {
                    <button
                        on:click=move |_| handle_login.dispatch(())
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
                    </button>
                }
            }
        >
            <div>{"user: "}{move || principal().map(|p| p.to_text()).unwrap_or_default()}</div>

            <div class="flex items-center space-x-2">

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
