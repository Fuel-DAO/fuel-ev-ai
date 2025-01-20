use crate::canister::backend::Backend;
// canisters.rs
use crate::canister::provision::Provision;
use crate::canister::token::Token;
use crate::canister::{ASSET_PROXY_ID, BACKEND_ID, PROVISION_ID};

use crate::state::asset_manager::AssetManager;
use crate::state::auth::AuthService;
use candid::Principal;
use ic_agent::Agent;
use leptos::prelude::{expect_context, provide_context};
use std::cmp::PartialEq;
use std::sync::Arc;

#[derive(Clone)]
pub struct Canisters {
    // pub auth_service: Arc<AuthService>,
    pub agent: Arc<Agent>,
    provision_principal: Principal,
}

impl Canisters {
    pub async fn new(mut  auth_service: AuthService) -> Result<Self, String> {
        let agent =  {
            auth_service.get_agent().await?
        };
        Ok(Self {
            // auth_service,
            agent,
            provision_principal: PROVISION_ID,
        })
    }

    pub fn set_global(state: Self) {
        provide_context(state);
    }

    pub fn get() -> Option<Self> {
        expect_context()
    }

    pub fn principal() -> Option<Principal> {
        Self::get_authenticated().ok().map(|f| f.agent.get_principal().ok()).flatten()
    }

    pub fn get_authenticated() -> Result<Self, String> {
        if Self::is_authenticated() {
            Self::get().ok_or("User is not authenticated".into())
        } else {
            Err("User is not authenticated".into())
        }
    }

    pub fn is_authenticated() -> bool {
        Self::get().map(|f| f.agent.get_principal().ok().map(|f| f != Principal::anonymous())).flatten().unwrap_or(false)
    }
    pub async  fn provision_canister(&self) -> Provision<'_> {
        let agent_ref: &Agent = &self.agent;
        Provision(self.provision_principal, agent_ref)
    }

    pub async fn backend_canister(&self) -> Backend<'_> {
        let agent_ref: &Agent = &self.agent;
        Backend(BACKEND_ID, agent_ref)
    }

    pub async fn token_canister(&self, canister_id: Principal) -> Token<'_> {
        let agent_ref: &Agent = &self.agent;
        Token(canister_id, agent_ref)
    }

    pub fn asset_manager(&self) -> AssetManager<'_> {
        // let asset_canister_id = Principal::from_text(TEMP_ASSET_CANISTER_ID).unwrap();
        let asset_proxy_canister_id =
                ASSET_PROXY_ID;

        AssetManager::new(asset_proxy_canister_id, &self.agent)
    }
}

impl PartialEq for Canisters {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.agent, &other.agent)
            && self.provision_principal == other.provision_principal
    }
}
