use crate::canister::backend::Backend;
// canisters.rs
use crate::canister::provision::Provision;
use crate::canister::token::Token;
use crate::canister::{ASSET_PROXY_ID, BACKEND_ID, PROVISION_ID};

use crate::state::asset_manager::AssetManager;
use crate::state::auth::AuthService;
use crate::utils::go_back_and_come_back::clear_localstorage;
use candid::Principal;
use gloo::utils::window;
use ic_agent::Agent;
use leptos::logging;
use leptos::prelude::{expect_context, provide_context, use_context, Get,  RwSignal, Set};
use std::sync::Arc;

#[derive(Clone)]
pub struct Canisters {
    pub agent: Arc<Agent>,
    provision_principal: Principal,
}

impl Canisters {
    pub async fn new(mut auth_service: AuthService) -> Result<Self, String> {
        let agent = { auth_service.get_agent().await? };
        let this = Self {
            // auth_service,
            agent,
            provision_principal: PROVISION_ID,
        };
        Ok(this)
    }

    pub async fn login() -> Result<(), String> {
        let mut auth_service = AuthService::new()?;
        auth_service.login().await?;
        let state = Self::new(auth_service).await?;
        Self::set_global(state);
        // Self::reload();
        Ok(())
    }

    pub async fn logout() -> Result<(), String> {
        clear_localstorage();
        Self::reload();
        Ok(())
    }

    fn reload() {
        let _ = window().location().reload().unwrap();
    }

    pub fn set_global(state: Self) {
        logging::log!("Set global called {:?}", state.agent.get_principal().map(|f| f.to_text()));
        let this =  use_context::<RwSignal<Option<Self>>>();
        match  this {
            Some(this) =>this.set(Some(state)), 
            None => provide_context(RwSignal::new(Some(state))),
        }
        ; 

        // provide_context(Some(RwSignal::new(state)));
        // let this:Option<RwSignal<Self>> = expect_context();
        // logging::log!("Get global called {:?}", this.get().agent.get_principal().map(|f| f.to_text()));
    }

    pub fn get() -> Option<Self> {
        logging::log!("Fetching canisters");
        let this = use_context::<RwSignal<Option<Canisters>>>();
        logging::log!("Get global called {:?}", this.map(|f| f.get().map(|f| f.agent.get_principal().map(|f| f.to_text()) )));
        this.map(|f| f.get()).flatten()
    }

    pub fn principal() -> Option<Principal> {
        Self::get_authenticated()
            .ok()
            .map(|f| f.agent.get_principal().ok())
            .flatten()
    }

    pub fn get_authenticated() -> Result<Self, String> {
        if Self::is_authenticated() {
            Self::get().ok_or("User is not authenticated".into())
        } else {
            Err("User is not authenticated".into())
        }
    }

    pub fn is_authenticated() -> bool {
        Self::get()
            .map(|f| {
                f.agent
                    .get_principal()
                    .map(|f| f != Principal::anonymous())
                    .ok()
            })
            .flatten()
            .unwrap_or(false)
    }
    pub async fn provision_canister(&self) -> Provision<'_> {
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
        let asset_proxy_canister_id = ASSET_PROXY_ID;

        AssetManager::new(asset_proxy_canister_id, &self.agent)
    }
}

impl PartialEq for Canisters {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.agent, &other.agent)
            && self.provision_principal == other.provision_principal
    }
}
