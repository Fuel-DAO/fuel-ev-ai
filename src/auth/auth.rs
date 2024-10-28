// use serde::{Deserialize, Serialize};
//
// // Define the AuthHelper structure
// #[derive(Serialize, Deserialize, Debug)]
// pub struct AuthHelper {
//     pub init: Option<bool>,
//     pub client: Option<String>, // Replace with the actual type for `authclient`
//     pub identity: Option<String>, // Replace with the actual type for `identity`
//     pub id_principal: Option<String>, // Replace with the actual type for `principal`
//     pub user_canister_principal: Option<String>, // Replace with the actual type for `principal`
// }
//
// // Define the AuthState structure
// #[derive(Serialize, Deserialize, Debug)]
// pub struct AuthState {
//     pub is_logged_in: bool,
//     pub id_string: Option<String>,
//     pub user_canister_id: Option<String>,
//     pub show_login: bool,
//     pub t: Option<bool>,
// }
//
// // Initialize default values for AuthState
// impl Default for AuthState {
//     fn default() -> Self {
//         Self {
//             is_logged_in: false,
//             id_string: None,
//             user_canister_id: None,
//             show_login: false,
//             t: None,
//         }
//     }
// }
//
// // Create instances of AuthHelper and AuthState
// pub fn create_auth_helper() -> AuthHelper {
//     AuthHelper {
//         init: None,
//         client: None,
//         identity: None,
//         id_principal: None,
//         user_canister_principal: None,
//     }
// }
//
// pub fn create_auth_state() -> AuthState {
//     AuthState::default()
// }
