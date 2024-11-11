pub fn get_host() -> String {
        use leptos::window;
        window().location().host().unwrap().to_string()
}
