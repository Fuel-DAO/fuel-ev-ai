use crate::components::header2::Header2;
use crate::outbound::admin_check::is_admin;
use crate::state::canisters::Canisters;
use crate::state::{
    auth::AuthService,
    auth_actions::{create_login_action, create_logout_action},
};
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;

#[component]
pub fn AdminComponent() -> impl IntoView {
    // Obtain AuthService from the context
    let auth_service =
        use_context::<Rc<RefCell<AuthService>>>().expect("AuthService context must be provided");

    // Obtain Canisters from the context
    let canisters = use_context::<RwSignal<Option<Rc<Canisters>>>>()
        .expect("Canisters ReadWriteSignal must be provided");

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
    let handle_login = create_login_action();
    let handle_logout = create_logout_action();
    // Create a resource to fetch admin status
    let is_admin_resource = create_resource(
        move || {
            let maybe_canisters = canisters.get();
            let maybe_principal = principal();
            (maybe_canisters, maybe_principal)
        },
        move |(maybe_canisters, maybe_principal)| async move {
            if let Some(rc_canisters) = maybe_canisters {
                is_admin(&rc_canisters, maybe_principal).await
            } else {
                Err("Canisters are not available".to_string())
            }
        },
    );

    view! {
        <Header2 />
        <div class="flex flex-col items-center pt-32 gap-4 min-h-screen p-4">
            <div class="font-bold text-xl">"Admin"</div>

            // Conditional Rendering Based on Principal Availability
            {move || {
                if principal().is_some() {
                    match is_admin_resource.get() {
                        None => {
                            view! {
                                // Principal is available, proceed with admin content
                                // Resource is loading
                                <div>"Loading..."</div>
                            }
                                .into_view()
                        }
                        Some(Ok(true)) => {
                            view! {
                                // User is an admin
                                <>
                                    <div>"Welcome, Admin!"</div>
                                    <div class="text-sm">
                                        {"user: "}
                                        {principal().map(|p| p.to_text()).unwrap_or_default()}
                                        <div class="flex flex-col items-center justify-center mt-4 gap-4">
                                            <a
                                                href="/admin/new-collection"
                                                role="presentation"
                                                class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 "
                                            >
                                                <div class=" transition-opacity">
                                                Submit a new vehicle for tokenization
                                                </div>
                                            </a>

                                            <div class="text-sm text-gray-400">or</div>
                                            <a
                                                href="/admin/manage/list"
                                                role="presentation"
                                                class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 "
                                            >
                                                <div class=" transition-opacity">
                                                Manage submitted listings
                                                </div>
                                            </a>
                                        </div>
                                    </div>
                                </>
                            }
                                .into_view()
                        }
                        Some(Ok(false)) => {
                            view! {
                                // User is not authorized
                                <>
                                    <div>"You're not authorized"</div>
                                    <div class="text-sm">
                                        {"user: "}
                                        {principal().map(|p| p.to_text()).unwrap_or_default()}
                                        <div class="flex flex-col mt-4 items-center justify-center gap-4">
                                            <a
                                                href="/admin/new-collection"
                                                role="presentation"
                                                class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 "
                                            >
                                                <div class=" transition-opacity">
                                                Submit a new vehicle for tokenization
                                                </div>
                                            </a>

                                            <div class="text-sm text-gray-400">or</div>
                                            <a
                                                href="/admin/manage/list"
                                                role="presentation"
                                                class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 "
                                            >
                                                <div class=" transition-opacity">
                                                Manage submitted listings
                                                </div>
                                            </a>
                                        </div>

                                    </div>
                                </>
                            }
                                .into_view()
                        }
                        Some(Err(e)) => {
                            view! {
                                // An error occurred while fetching admin status
                                <div>{format!("Error: {}", e)}</div>
                            }
                                .into_view()
                        }
                    }
                } else {
                    view! {
                        // Principal is not available, prompt user to log in
                        <div class="text-center text-lg">"You need to log in first"</div>
                        <button
                            on:click=move |_| handle_login.dispatch(())
                            role="presentation"
                            class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30 "
                        >
                            <div class=" transition-opacity">Login</div>
                        </button>
                    }
                        .into_view()
                }
            }}
        </div>
    }
}
