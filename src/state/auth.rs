use ic_agent::{Agent, Identity};
use ic_auth_client::{AuthClient, AuthClientLoginOptions};

pub struct AuthService {
    auth_client: AuthClient,
    agent: Option<Agent>, // Store agent after login
}

impl AuthService {
    // Initialize AuthService with a new AuthClient
    pub async fn new() -> Result<Self, String> {
        let auth_client = AuthClient::builder().build().await;
        Ok(AuthService {
            auth_client,
            agent: None, // Initialize without an agent
        })
    }

    // Login function using Identity with custom options
    pub async fn login(&mut self) -> Result<(), String> {
        let options = AuthClientLoginOptions::builder()
            .max_time_to_live(7 * 24 * 60 * 60 * 1_000_000_000) // Example: 7 days in nanoseconds
            .on_success(|auth_success| {
                println!("Login successful: {:?}", auth_success);
            })
            .build();

        // Perform login and check if successful
        self.auth_client.login_with_options(options);

        // Retrieve the identity from the auth client and create an agent with it
        let identity = self.auth_client.identity();
        let agent = Agent::builder()
            .with_identity(identity) // Use the retrieved identity for the agent
            .build()
            .map_err(|e| e.to_string())?;

        self.agent = Some(agent); // Store the authenticated agent

        Ok(())
    }

    // Get the authenticated Agent instance
    pub fn get_agent(&self) -> Result<&Agent, String> {
        self.agent
            .as_ref()
            .ok_or("Agent not available. Please login first.".to_string())
    }
}
