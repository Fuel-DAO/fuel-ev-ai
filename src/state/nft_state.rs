use crate::state::canisters::Canisters;
use futures::future::Lazy;
use leptos::*;
use std::rc::Rc;
// Define this in a central location, like a `state.rs` file
static CANISTERS: Lazy<RwSignal<Option<Rc<Canisters>>>> = Lazy::new(|| create_rw_signal(None));

pub fn update_canisters_context(canisters: Option<Rc<Canisters>>) {
    CANISTERS.set(canisters);
}

pub fn use_canisters() -> ReadSignal<Option<Rc<Canisters>>> {
    CANISTERS.read_only()
}
