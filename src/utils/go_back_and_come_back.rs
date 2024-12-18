use leptos_router::use_navigate;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use leptos::*;
pub fn go_back_and_come_back() {
    if let Some(win) = web_sys::window() {

        let current_url = window().location().href().ok();
        if current_url.is_some() {

            let current_loc = current_url.unwrap();

        let navigator = use_navigate();

        // Go back to the previous page
        win.history().unwrap().back().unwrap();

        // Use setTimeout to navigate back to the current location after a short delay
        let closure = Closure::wrap(Box::new(move || {
            navigator(&current_loc, Default::default());
        }) as Box<dyn Fn()>);

        win.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 1000).unwrap();
        closure.forget(); 

        } 
    
    }
}
 
pub fn go_to_home() {
    let navigator = use_navigate();
        navigator("/", Default::default());
}
 
pub fn clear_localstorage() {
    let _ = window().local_storage().map(|s| s.map(|f| f.clear()));
}