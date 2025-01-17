use candid::Principal;
use futures::executor::block_on;
use ic_agent::{identity::Identity, Agent};
use ic_auth_client::{AuthClient, AuthClientLoginOptions};
use leptos::logging;
use leptos::window;
use log::info;
use std::error::Error;
use std::rc::Rc;
use std::time::Duration;
use web_sys::Url;

use crate::canister::PROVISION_ID;
use crate::utils::go_back_and_come_back::*;
pub const TIMEOUT: Duration = Duration::from_secs(60 * 5);

#[derive(Clone)]
pub struct AuthService {
    auth_client: AuthClient,
    agent: Option<Rc<Agent>>, // Store agent in Rc for shared ownership
}

impl AuthService {
    pub fn new() -> Result<Self, String> {
        let auth_client = block_on(AuthClient::builder().build());
        Ok(AuthService {
            auth_client,
            agent: None,
        })
    }

    pub async fn login(&mut self) -> Result<(), String> {
        // dotenv::dotenv().ok();
        let  dfx_network = "LIVE".to_string();
        // if dfx_network.is_empty() {
        //     dfx_network = env::var("BACKEND").unwrap_or("LIVE".to_owned());
        // }

        let identity_provider: Option<Url> = match dfx_network.as_str() {
            "LOCAL" => Some({
                let port = 4943;
                let canister_id = PROVISION_ID;
                Url::new(&format!("http://{}.localhost:{}", canister_id, port)).unwrap()
            }),
            "LIVE" => Some(Url::new("https://identity.ic0.app/#authorize").unwrap()),
            _ => panic!("Unknown dfx network: {}", dfx_network),
        };

        let mut builder = AuthClientLoginOptions::builder()
            .max_time_to_live(7 * 24 * 60 * 60 * 1_000_000_000) // 7 days in nanoseconds
            .on_success(|_| {
                // Handle successful login
                info!("Login successful");
                window().location().reload().unwrap();
            })
            .on_error(|error| {
                // Handle login error
                logging::log!("Login failed: {:?}", error);
            });

        // Only set the identity_provider if it's Some
        if let Some(provider) = identity_provider {
            builder = builder.identity_provider(provider);
        }

        let options = builder.on_success(|_| {go_back_and_come_back();}).build();

        // Initiate the login process
        self.auth_client.login_with_options(options);

        // Verify authentication after login
        if self.auth_client.is_authenticated() {
            Ok(())
        } else {
            Err("Authentication failed".to_string())
        }
    }
    pub async fn get_agent(&mut self) -> Result<Rc<Agent>, String> {
        if self.agent.is_none() {
            self.agent = Some(Rc::new(create_agent(&self.auth_client).await?));
        }
        Ok(self.agent.as_ref().unwrap().clone())
    }

    /// Get the principal (identity's sender)
    pub fn get_principal(&self) -> Result<Principal, Box<dyn Error>> {
        self.auth_client
            .identity()
            .sender()
            .map_err(|_| "Unable to retrieve principal.".into())
    }
    pub fn is_authenticated(&self) -> bool {
        self.auth_client.is_authenticated()
    }

    pub async fn logout(&mut self) -> Result<(), String> {
        // Call the logout method on the AuthClient
        self.auth_client
            .logout(None)
            .await;

        // Clear the agent
        self.agent = None;


        // go_back_and_come_back();


        // Reload the page
        // web_sys::window()
        //     .unwrap()
        //     .location()
        //     .reload()
        //     .map_err(|_| "Failed to reload page".to_string())?;

        // Log the logout action
        info!("Logout successful");

        Ok(())
    }
}

async fn create_agent(auth_client: &AuthClient) -> Result<Agent, String> {
    let identity = auth_client.identity();
    // dotenv::dotenv().ok();
    let  dfx_network = "LIVE".to_string();
    // if dfx_network.is_empty() {
    //     dfx_network = env::var("BACKEND").unwrap_or("LIVE".to_owned());
    // }

    let url = match dfx_network.as_str() {
        "LOCAL" => "http://127.0.0.1:4943".to_string(),
        "LIVE" => "https://ic0.app".to_string(),
        _ => return Err(format!("Unknown DFX network: {}", dfx_network)),
    };

    let agent = Agent::builder()
        .with_url(url)
        .with_identity(identity)
        .with_ingress_expiry(Some(TIMEOUT))
        .build()
        .map_err(|e| format!("Failed to build agent: {}", e))?;

    if dfx_network == "LOCAL" {
        agent
            .fetch_root_key()
            .await
            .map_err(|e| format!("Failed to fetch root key: {}", e))?;
    }

    Ok(agent)
}
