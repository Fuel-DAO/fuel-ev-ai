use candid::Principal;
use futures::executor::block_on;
use ic_agent::{Agent, Identity};
use ic_auth_client::{AuthClient, AuthClientLoginOptions};
use leptos_use::storage::use_local_storage;
use log::{error, info};
use std::error::Error;

#[derive(Clone)]
pub struct AuthService {
    auth_client: AuthClient,
    agent: Option<Agent>, // Store agent after login
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
        let options = AuthClientLoginOptions::builder()
            .max_time_to_live(7 * 24 * 60 * 60 * 1_000_000_000)
            .on_success(|auth_success| {
                info!("Login successful: {:?}", auth_success);
            })
            .build();
        self.auth_client.login_with_options(options);

        let identity = self.auth_client.identity();
        let agent = Agent::builder()
            .with_url("https://ic0.app")
            .with_identity(identity)
            .build()
            .map_err(|e| format!("Failed to build Agent: {}", e))?;

        self.agent = Some(agent);
        Ok(())
    }

    /// Get the authenticated Agent instance
    pub fn get_agent(&self) -> Result<&Agent, String> {
        self.agent
            .as_ref()
            .ok_or_else(|| "Agent not available. Please login first.".to_string())
    }

    /// Get the principal (identity's sender)
    pub fn get_principal(&self) -> Result<Principal, Box<dyn Error>> {
        self.auth_client
            .identity()
            .sender()
            .map_err(|_| "Unable to retrieve principal.".to_string().into())
    }
}
