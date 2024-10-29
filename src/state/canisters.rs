use std::sync::Arc;

use candid::Principal;
use ic_agent::{identity::BasicIdentity, Identity};
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::canister::backend::Backend;
use crate::canister::provision::Provision;
use crate::canister::token::Token;
use crate::utils::ic::AgentWrapper;

use crate::canister::{BACKEND_ID, PROVISION_ID, TOKEN_ID};

#[derive(Clone, Serialize, Deserialize)]
pub struct CanistersAuthWire {
    user_principal: Principal,
    expiry: u64,
    backend_principal: Principal,
    provision_principal: Principal,
    token_principal: Principal,
    // Add identity field to hold the user's identity
    #[serde(skip)]
    identity: Option<Arc<dyn Identity + Sync + Send>>,
}

impl std::fmt::Debug for CanistersAuthWire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CanistersAuthWire")
            .field("user_principal", &self.user_principal)
            .field("expiry", &self.expiry)
            .field("backend_principal", &self.backend_principal)
            .field("provision_principal", &self.provision_principal)
            .field("token_principal", &self.token_principal)
            .field("identity", &"<Identity omitted>")
            .finish()
    }
}

impl CanistersAuthWire {
    // Constructor to create a new instance with an optional identity
    pub fn new(
        user_principal: Principal,
        expiry: u64,
        backend_principal: Principal,
        provision_principal: Principal,
        token_principal: Principal,
        identity: Option<Arc<dyn Identity + Sync + Send>>,
    ) -> Self {
        Self {
            user_principal,
            expiry,
            backend_principal,
            provision_principal,
            token_principal,
            identity,
        }
    }

    pub fn canisters(self) -> Canisters {
        // Build the agent with the user's identity if available
        let agent = AgentWrapper::build(|builder| {
            if let Some(identity) = self.identity.clone() {
                builder.with_arc_identity(identity)
            } else {
                builder
            }
        });

        Canisters {
            agent,
            user_principal: self.user_principal,
            expiry: self.expiry,
            backend_principal: self.backend_principal,
            provision_principal: self.provision_principal,
            token_principal: self.token_principal,
        }
    }
}

#[derive(Clone)]
pub struct Canisters {
    agent: AgentWrapper,
    user_principal: Principal,
    expiry: u64,
    backend_principal: Principal,
    provision_principal: Principal,
    token_principal: Principal,
}

impl Default for Canisters {
    fn default() -> Self {
        Self {
            agent: AgentWrapper::build(|b| b),
            user_principal: Principal::anonymous(),
            expiry: 0,
            backend_principal: BACKEND_ID,
            provision_principal: PROVISION_ID,
            token_principal: TOKEN_ID,
        }
    }
}

impl Canisters {
    pub async fn backend_canister(&self) -> Backend<'_> {
        self.backend().await
    }

    pub async fn provision_canister(&self) -> Provision<'_> {
        let agent = self.agent.get_agent().await;
        Provision(self.provision_principal, agent)
    }

    pub async fn token_canister(&self, canister_id: Principal) -> Token<'_> {
        let agent = self.agent.get_agent().await;
        Token(canister_id, agent)
    }

    async fn backend(&self) -> Backend<'_> {
        let agent = self.agent.get_agent().await;
        Backend(self.backend_principal, agent)
    }
}

pub fn unauth_canisters() -> Canisters {
    Canisters::default()
}
