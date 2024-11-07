use candid::Principal;
use dotenv_codegen::dotenv;
use futures::executor::block_on;
use ic_agent::{identity::Identity, Agent};
use ic_auth_client::{AuthClient, AuthClientLoginOptions};
use log::{error, info};
use std::env;
use std::error::Error;
use std::rc::Rc;
use std::time::Duration;

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
        let options = AuthClientLoginOptions::builder()
            .max_time_to_live(7 * 24 * 60 * 60 * 1_000_000_000)
            .on_success(|auth_success| {
                info!("Login successful: {:?}", auth_success);
            })
            .build();
        self.auth_client.login_with_options(options);
        Ok(())
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
}

async fn create_agent(auth_client: &AuthClient) -> Result<Agent, String> {
    let identity = auth_client.identity();

    let mut dfx_network = dotenv!("BACKEND").to_string();
    if dfx_network.is_empty() {
        dfx_network = env::var("DFX_NETWORK").expect("DFX_NETWORK must be set");
    }

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
