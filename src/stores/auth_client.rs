use ic_auth_client::{AuthClient, AuthClientLoginOptions};
use leptos::{leptos_dom::logging::console_warn, logging, prelude::*, task::spawn_local};
use send_wrapper::SendWrapper;
use web_sys::Url;

use crate::{
    canister::BACKEND_ID,
    state::{auth::AuthService, canisters::Canisters},
};

/// Component that provides the AuthClient to the children components
#[component]
pub fn AuthClientProvider(children: Children) -> impl IntoView {
    let auth_client= RwSignal::new(send_wrapper::SendWrapper::new(None));

    spawn_local(async move {
        auth_client.set(SendWrapper::new(Some(
            AuthClient::builder()
                .on_idle(|| {
                    spawn_local(async move {
                        logout().await.unwrap();
                        window().location().reload().unwrap();
                    });
                })
                .idle_timeout(20 * 60 * 1000) // 20 minutes
                .capture_scroll(true)
                .build()
                .await
                ,
        )));
    });

    provide_context(auth_client);

    children()
}

 async fn auth_client() -> Result<AuthClient, AuthClientError> {
    logging::log!("Getting auth client");
    let auth_client: RwSignal<SendWrapper<Option<AuthClient>>> = match use_context() {
        Some(auth_client) => auth_client,
        None => return  AuthClient::builder()
        .on_idle(|| {
            spawn_local(async move {
                logout().await.unwrap();
                window().location().reload().unwrap();
            });
        })
        .idle_timeout(20 * 60 * 1000) // 20 minutes
        .capture_scroll(true)
        .build()
        .await
        .map_err(|_|AuthClientError::AuthClientNotInitialized),
    };
    logging::log!("Got auth client");
    if let Some(auth_client) = &*auth_client.get_untracked().clone() {
        logging::log!("Got Selected auth client");
        Ok(auth_client.clone())
    } else {
        Err(AuthClientError::AuthClientNotInitialized)
    }
}

// pub fn get_identity() -> Arc<dyn Identity> {
//     match auth_client() {
//         Ok(auth_client()) => auth_client.identity(),
//         Err(_) => Arc::new(AnonymousIdentity),
//     }
// }

pub async fn login() -> Result<(), AuthClientError> {
    let dfx_network = "ic".to_string();

    let identity_provider = match dfx_network.as_str() {
        "local" => Some({
            let port = 4943;
            let canister_id = BACKEND_ID.to_text();
            Url::new(&format!("http://{}.localhost:{}", canister_id, port)).unwrap()
        }),
        "ic" => None,
        _ => panic!("Unknown dfx network: {}", dfx_network),
    };

    let mut auth_client = auth_client().await?;

    let on_success = move |_| {
        // window().location().reload().unwrap();
    };
    let on_error = |e| {
        if let Some(e) = e {
            console_warn(&format!("Failed to login: {:?}", e));
        } else {
            console_warn("Failed to login");
        }
    };

    let options = match identity_provider {
        Some(identity_provider) => {
            AuthClientLoginOptions::builder().identity_provider(identity_provider)
        }
        None => AuthClientLoginOptions::builder(),
    };
    let options = options.on_success(on_success).on_error(on_error).build();

    auth_client.login_with_options(options);

    if auth_client.is_authenticated() {
        let _ = Canisters::new(AuthService::from_client(auth_client.clone())).await;
        Ok(())
    } else {
        Err(AuthClientError::AuthClientContextError)
    }
}

pub async fn logout() -> Result<(), AuthClientError> {
    auth_client().await?.logout(Some(window().location())).await;
    Ok(())
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum AuthClientError {
    #[error("Out of auth client context")]
    AuthClientContextError,
    #[error("Auth client not initialized")]
    AuthClientNotInitialized,
}
