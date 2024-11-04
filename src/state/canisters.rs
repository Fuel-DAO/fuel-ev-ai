use crate::canister::provision::Provision;
use crate::canister::token::Token;
use crate::canister::PROVISION_ID;
use crate::state::auth::AuthService; // Import the AuthService
use candid::Principal;

#[derive(Clone)]
pub struct Canisters {
    auth_service: AuthService,
    provision_principal: Principal,
}

impl Canisters {
    pub async fn new() -> Result<Self, String> {
        // Initialize AuthService, ensuring login before accessing the agent
        let auth_service = AuthService::new().await?;
        Ok(Self {
            auth_service,
            provision_principal: PROVISION_ID,
        })
    }

    // Function to log in with AuthService
    pub async fn login(&mut self) -> Result<(), String> {
        self.auth_service.login().await // Calls login on AuthService
    }

    // Access the Provision canister, borrowing the agent from AuthService
    pub async fn provision_canister(&self) -> Result<Provision<'_>, String> {
        let agent = self.auth_service.get_agent()?; // Unwraps agent or returns an error
        Ok(Provision(self.provision_principal, agent))
    }

    // Access the Token canister
    pub async fn token_canister(&self, canister_id: Principal) -> Result<Token<'_>, String> {
        let agent = self.auth_service.get_agent()?; // Calls get_agent on auth_service
        Ok(Token(canister_id, agent))
    }
}
