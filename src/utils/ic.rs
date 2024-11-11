use std::sync::Arc;

use candid::Principal;
use dotenv_codegen::dotenv;
use ic_agent::{ identity::AnonymousIdentity, Agent, Identity};
use ic_auth_client::AuthClient;
use leptos::{ use_context, ReadSignal, SignalGet};


#[derive(Clone)]
pub struct AgentWrapper(Agent);

impl AgentWrapper {
    pub fn build() -> Self {
        Self(Self::refresh_agent())
    }

 pub fn refresh_agent() -> Agent {
        let is_dev = dotenv!("BACKEND") == "LOCAL";

        let anonymouns_agent = Agent::builder().with_url(if is_dev {
            crate::consts::local::AGENT_URL
        } else {
            crate::consts::remote::AGENT_URL
        }).with_identity(AnonymousIdentity).build().unwrap();


        let auth_client = use_context::<ReadSignal<Option<AuthClient>>>().unwrap();
         let agent =  auth_client
            .get()
            .map(|f| match f.is_authenticated() {
                true => Agent::builder().with_url(if is_dev {
                    crate::consts::local::AGENT_URL
                } else {
                    crate::consts::remote::AGENT_URL
                })
                .with_identity(f.identity())
                .build().unwrap(),
                false => anonymouns_agent.clone(),
            }).unwrap_or(anonymouns_agent);
         agent
    }

    pub async fn get_agent(&self) -> &Agent {
        let agent = &self.0;
        agent
    }

    // pub fn set_arc_id(&mut self, id: Arc<impl Identity + 'static>) {
    //     self.0.set_arc_identity(id);
    // }

    // pub fn principal(&self) -> Result<Principal, String> {
    //     self.0.get_principal()
    // }
}
