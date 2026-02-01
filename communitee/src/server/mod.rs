use leptos::prelude::*;
use cfg_if::cfg_if;

use crate::structs::{LoginAuth, Session};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::ServerSideData;
    }
}

pub async fn require_login() -> Result<Option<Session>,ServerFnError> {
    if let Some(session) = perform_login(LoginAuth::default(), "".into()).await? {
        Ok(Some(session))
    } else {
        #[cfg(feature = "hydrate")]
        {
            use leptos_router::hooks::use_navigate;
            let nav = use_navigate();
            nav(&format!("/login"), Default::default());
        }
        
        Ok(None)
    }
}
/*
#[server]
pub async fn get_session() -> Result<Option<Session>,ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    
    let server = server_side_data.server.lock()?;
    if let Some(server) = server.get_session(auth) .as_ref() {
        Ok(server.get_session())
    } else {
        Ok(None)
    }
}
 */
#[server]
pub async fn perform_login(auth: LoginAuth, new_path: String) -> Result<Option<Session>,ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");

    let mut server = server_side_data.server.lock()?;
    Ok(server.get_session(&auth).cloned())
    //let nav = use_navigate();
    //nav(&new_path, Default::default());
    //Ok(())
}
