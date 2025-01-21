use std::future::Future;


use leptos::{leptos_dom, logging, prelude::*};

use super::canisters::Canisters;


/// Creates a login action.
pub fn create_login_action() -> Action<(), ()> {
    Action::new(move |_: &()| {
        send_wrap(async move {
            match Canisters::login().await {
                Ok(()) => {
                    logging::log!("Login successful.")
                }
                Err(e) => logging::log!("Login failed: {:?}", e),
            }
        })
    })
}

/// Creates a logout action.
pub fn create_logout_action() -> Action<(), Result<(), ()>> {
    Action::new(|_: &()| {
        send_wrap(async {
            let _ = Canisters::logout().await;
            Ok(())
        })
    })
}

// pub fn send_wrap<Fut: Future + Send>(
//     t: Fut,
// ) -> impl Future<Output = <Fut as Future>::Output> + Send {
//     t
// }

// Wraps a specific future that is not Send when hydrate feature is enabled
// the future must be Send when ssr is enabled
// use only when necessary (usually inside resources)
// if you get a Send related error inside an Action, it probably makes more
// sense to use Action::new_local or Action::new_unsync
pub fn send_wrap<Fut: Future>(t: Fut) -> impl Future<Output = <Fut as Future>::Output> + Send {
    Box::pin(send_wrapper::SendWrapper::new(t))
}
