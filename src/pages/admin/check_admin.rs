use std::{cell::RefCell, rc::Rc};

use leptos::*;

use crate::{outbound::admin_check::is_admin, state::{admin::Admin, auth::AuthService, canisters::Canisters}};



#[component]
pub fn AdminProvider(children: Children) -> impl IntoView {

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

    spawn_local(async move {
        let is_authenticated = is_authenticated();
        let principal = principal();
        if let Some(rc_canisters) = canisters.get() {
            let admin =  is_admin(&rc_canisters, principal).await;
            match admin {
                Ok(is) => {
                    Admin::get().is_admin.set(is);
                    Admin::get().principal.set(principal);
                }, 
                Err(_) => {}
            }
            
            
        }
    });
    children()  
}


#[component]
pub fn AdminRoute() -> impl IntoView {

    view! {
        <AdminProvider>
        <Show when=move || (Admin::get().is_admin)()>
                <a href="/admin">
                    <span class="text-black font-medium">"Admin"</span>
                </a>
        </Show>
        </AdminProvider>
    }

}