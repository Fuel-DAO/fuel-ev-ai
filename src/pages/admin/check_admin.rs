
use leptos::*;

use crate::{outbound::admin_check::is_admin, state::{admin::Admin, canisters::Canisters}};



#[component]
pub fn AdminProvider(children: Children) -> impl IntoView {


    spawn_local(async move {
        let principal =Canisters::principal();
        if let Some(rc_canisters) = Canisters::get() {
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