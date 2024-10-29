use candid::Principal;
use ic_agent::{identity::BasicIdentity, Identity};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::canister::backend::Backend;
use crate::canister::provision::Provision;
use crate::canister::token::Token;
use crate::canister::{BACKEND_ID, PROVISION_ID, TOKEN_ID};
use crate::utils::ic::AgentWrapper;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CanistersAuthWire {
    expiry: u64,
    backend_principal: Principal,
    provision_principal: Principal,
    token_principal: Principal,
    #[serde(skip)]
    identity: Option<Arc<BasicIdentity>>, // Using BasicIdentity as a placeholder
}

impl CanistersAuthWire {
    pub fn canisters(self) -> Result<Canisters<true>, String> {
        let unauth = unauth_canisters();

        let id = self.identity.clone().ok_or("Identity not found")?;
        let mut agent = unauth.agent.clone();
        agent.set_arc_id(id.clone());

        Ok(Canisters {
            agent,
            identity: Some(id),
            expiry: self.expiry,
            backend_principal: self.backend_principal,
            provision_principal: self.provision_principal,
            token_principal: self.token_principal,
        })
    }
}

#[derive(Clone)]
pub struct Canisters<const AUTH: bool> {
    agent: AgentWrapper,
    identity: Option<Arc<BasicIdentity>>,
    expiry: u64,
    backend_principal: Principal,
    provision_principal: Principal,
    token_principal: Principal,
}

impl Default for Canisters<false> {
    fn default() -> Self {
        Self {
            agent: AgentWrapper::build(|b| b),
            identity: None,
            expiry: 0,
            backend_principal: BACKEND_ID,
            provision_principal: PROVISION_ID,
            token_principal: TOKEN_ID,
        }
    }
}

impl Canisters<true> {
    pub fn authenticated(id: Arc<BasicIdentity>, expiry: u64) -> Canisters<true> {
        Canisters {
            agent: AgentWrapper::build(|b| b.with_arc_identity(id.clone())),
            identity: Some(id),
            expiry,
            backend_principal: BACKEND_ID,
            provision_principal: PROVISION_ID,
            token_principal: TOKEN_ID,
        }
    }

    pub fn expiry_ns(&self) -> u64 {
        self.expiry
    }

    pub fn identity(&self) -> &BasicIdentity {
        self.identity
            .as_ref()
            .expect("Authenticated canisters must have an identity")
    }

    pub fn user_principal(&self) -> Principal {
        self.identity()
            .sender()
            .expect("Expected principal to be present")
    }

    pub async fn backend_canister(&self) -> Backend<'_> {
        self.backend().await
    }
}

impl<const A: bool> Canisters<A> {
    pub async fn backend(&self) -> Backend<'_> {
        let agent = self.agent.get_agent().await;
        Backend(self.backend_principal, agent)
    }

    pub async fn provision_canister(&self) -> Provision<'_> {
        let agent = self.agent.get_agent().await;
        Provision(self.provision_principal, agent)
    }

    pub async fn token_canister(&self, canister_id: Principal) -> Token<'_> {
        let agent = self.agent.get_agent().await;
        Token(canister_id, agent)
    }
}

pub fn unauth_canisters() -> Canisters<false> {
    Canisters::default()
}

pub async fn do_canister_auth(identity: Arc<BasicIdentity>) -> Result<CanistersAuthWire, String> {
    let expiry = 1_640_000_000; // Example expiry time; replace with actual logic if needed.
    let canisters = Canisters::<true>::authenticated(identity.clone(), expiry);

    Ok(CanistersAuthWire {
        identity: Some(identity),
        expiry: canisters.expiry,
        backend_principal: BACKEND_ID,
        provision_principal: PROVISION_ID,
        token_principal: TOKEN_ID,
    })
}
