use candid::Principal;
use crate::canister::provision::Provision;
use crate::canister::token::Token;
use crate::canister::PROVISION_ID;
use crate::utils::ic::AgentWrapper;



#[derive(Clone)]
pub struct Canisters {
    agent: AgentWrapper,
    provision_principal: Principal,
}

impl Default for Canisters {
    fn default() -> Self {
        Self {
            agent: AgentWrapper::build(|b| b),
            provision_principal: PROVISION_ID,
        }
    }
}

impl Canisters {

    pub async fn provision_canister(&self) -> Provision<'_> {
        let agent = self.agent.get_agent().await;
        Provision(self.provision_principal, agent)
    }

    pub async fn token_canister(&self, canister_id: Principal) -> Token<'_> {
        let agent = self.agent.get_agent().await;
        Token(canister_id, agent)
    }
}