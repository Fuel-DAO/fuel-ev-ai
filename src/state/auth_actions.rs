// src/state/auth/auth_actions.rs
use crate::utils::go_back_and_come_back::go_to_home;
use leptos::{leptos_dom, prelude::*};
use leptos_dom::logging::{console_error, console_log};

use super::{auth::AuthService, canisters::Canisters};

/// Creates a login action.
pub fn create_login_action() -> Action<(), ()> {
    
    Action::new(move |_: &()| {
        let auth_service = AuthService::new().map(|f| f.login());
        async move {
            match  auth_service {
                Ok(ref mut f) => {
                    match f.await {
                        Ok(()) => {
                            window().location().reload().unwrap();
                            console_log("Login successful.")
                        },
                        Err(e) => console_error(&format!("Login failed: {:?}", e)),
                    }
                    
                }
                Err(e) => console_error(&format!("Login failed: {:?}", e)),
            }
        }
    })
}

/// Creates a logout action.
pub fn create_logout_action() -> Action<(), ()> {
    Action::new(move |_: &()| {
        let auth_service = Canisters::get_authenticated().unwrap().auth_service;
        async move {
            match auth_service.borrow_mut().logout().await {
                Ok(_) => {
                    console_log("Logout successful.");
                        go_to_home();
                        // clear_localstorage();

                    window().location().reload().unwrap();
                }
                Err(e) => console_error(&format!("Logout failed: {:?}", e)),
            }
        }
    })
}
