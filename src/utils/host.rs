use leptos::prelude::window;

pub fn get_host() -> String {
    window().location().host().unwrap().to_string()
}
