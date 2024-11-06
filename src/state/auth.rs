use candid::Principal;
use futures::executor::block_on;
use ic_agent::{identity::Identity, Agent};
use ic_auth_client::{AuthClient, AuthClientLoginOptions};
use log::{error, info};
use std::error::Error;
use std::rc::Rc;

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
            .map_err(|e| format!("Failed to build agent: {}", e))?;
        self.agent = Some(Rc::new(agent)); // Wrap the agent in Rc

        Ok(())
    }

    pub fn get_agent(&self) -> Result<Rc<Agent>, String> {
        self.agent
            .as_ref()
            .cloned() // Clones the Rc<Agent>, increasing the reference count
            .ok_or_else(|| "Agent not available. Please login first.".to_string())
    }

    /// Get the principal (identity's sender)
    pub fn get_principal(&self) -> Result<Principal, Box<dyn Error>> {
        self.auth_client
            .identity()
            .sender()
            .map_err(|_| "Unable to retrieve principal.".into())
    }
}
