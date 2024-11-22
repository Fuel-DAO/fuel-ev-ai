use crate::outbound::admin_check::is_admin;
use crate::state::canisters::Canisters;
use crate::state::{
    auth::AuthService,
    auth_actions::{create_login_action, create_logout_action},
};
use leptos::*;
use leptos_router::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::components::header2::Header2;

use leptos_router::*;

#[component]
pub fn ProtectedRoute() -> impl IntoView {
    // Consume the is_admin_resource from context
    let is_admin_resource = use_context::<Resource<(), Result<bool, String>>>()
        .expect("ProtectedRoute must be used within AdminComponent");

    view! {
        <Suspense fallback=move || {
            view! { <div>"Checking admin status..."</div> }
        }>
            {move || {
                match is_admin_resource.get() {
                    None => {
                        view! {
                            // Resource not yet loaded
                            <div>"Loading admin status..."</div>
                        }
                    }
                    Some(Ok(true)) => {
                        view! {
                            // Resource not yet loaded
                            // User is admin, render child routes
                            <div>
                                <Header2 />
                                <Outlet />
                            </div>
                        }
                    }
                    Some(Ok(false)) => {
                        view! {
                            // User is admin, render child routes
                            // Not admin
                            <div>"Access denied."</div>
                        }
                    }
                    Some(Err(e)) => {
                        view! {
                            // Not admin
                            // Error fetching admin status
                            <div>{format!("Error: {}", e)}</div>
                        }
                    }
                }
            }}
        </Suspense>
    }
}



#[component]
pub fn Admin() -> impl IntoView {
    // Obtain AuthService from the context
    let auth_service =
        use_context::<Rc<RefCell<AuthService>>>().expect("AuthService context must be provided");

    // Obtain Canisters from the context
    let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
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

    // Create a resource to fetch admin status
    let is_admin_resource = create_resource(
        move || {
            let maybe_canisters = canisters_signal.get();
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

    // Expose is_admin as a signal
    provide_context(is_admin_resource);

    // Optionally, render nothing or a placeholder
    view! {
        <>
            <Outlet />
        </>
    }
}
