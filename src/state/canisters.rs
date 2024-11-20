// canisters.rs
use crate::canister::provision::Provision;
use crate::canister::token::Token;
use crate::canister::PROVISION_ID;
use crate::state::auth::AuthService;
use candid::Principal;
use ic_agent::Agent;
use leptos::*;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::rc::Rc;

#[derive(Clone)]
pub struct Canisters {
    pub auth_service: Rc<RefCell<AuthService>>,
    agent: Rc<Agent>,
    provision_principal: Principal,
}

impl Canisters {
    pub async fn new(auth_service: Rc<RefCell<AuthService>>) -> Result<Self, String> {
        let agent = {
            let mut auth_service_borrow = auth_service.borrow_mut();
            auth_service_borrow.get_agent().await?
        };
        Ok(Self {
            auth_service,
            agent,
            provision_principal: PROVISION_ID,
        })
    }

    pub async fn provision_canister(&self) -> Provision<'_> {
        let agent_ref: &Agent = &self.agent;
        Provision(self.provision_principal, agent_ref)
    }

    pub async fn token_canister(&self, canister_id: Principal) -> Token<'_> {
        let agent_ref: &Agent = &self.agent;
        Token(canister_id, agent_ref)
    }
}

impl PartialEq for Canisters {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.agent, &other.agent)
            && self.provision_principal == other.provision_principal
    }
}
