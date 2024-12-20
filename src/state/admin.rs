use candid::Principal;
use leptos::{expect_context, provide_context, RwSignal};




#[derive(Default, Clone)]
pub struct Admin {
    pub is_admin: RwSignal<bool>, 
    pub principal: RwSignal<Option<Principal>>
}

impl Admin {
    pub fn set_global() {
        provide_context(Admin::default());
    }

    pub fn get() -> Self {
        expect_context()
    }
}