use cfg_if::cfg_if;
use chrono::SubsecRound;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use libertee::{LoginAuth, Session, SessionUuid, Timestamp};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::ServerSideData;
        use actix_identity::Identity;
        use actix_web::{HttpMessage, HttpRequest};
        use leptos_actix::extract;
    }
}

pub(crate) fn format_datetime(datetime: &Timestamp) -> String {
    let date = datetime.date_naive();
    let time = datetime.time().trunc_subsecs(0);
    format!("{}, {}", date.to_string(), time.to_string())
}

#[server]
pub async fn get_session_from_identity() -> Result<Option<SessionUuid>, ServerFnError> {
    let identity = match extract::<Identity>().await {
        Ok(identity) => identity,
        Err(ServerFnErrorErr::ServerError(err_str)) => {
            if err_str == "There is no identity information attached to the current session" {
                return Ok(None)
            } else {
                return Err(ServerFnError::ServerError(err_str));
            }
        },
        Err(e) => Err(e)?,
    };
    match identity.id() {
        Ok(id) => {
            let server_mutex = use_context::<ServerSideData>()
                .expect("ServerSideData should be provided, this should never fail.")
                .server;
            let server = server_mutex.lock()?;
            Ok(server.get_session(&SessionUuid(id))
                .map(|session|session.uuid.clone())
            )
        },
        Err(_) => {Ok(None)},
    }
}

pub async fn require_login() -> Result<Option<Session>, ServerFnError> {
    if let Some(session) = perform_login(LoginAuth::default(), Default::default()).await? {
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

#[server]
pub async fn perform_login(
    auth: LoginAuth,
    redirect_to: Option<String>,
) -> Result<Option<Session>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");

    let mut server = server_side_data.server.lock()?;
    
    let session = server.create_new_session(&auth).cloned();
    if let Some(session) = &session {
        let request = extract::<HttpRequest>().await.expect("Request should exist.");
        Identity::login(&request.extensions(), session.uuid.to_string())?;
    }
    if let Some(redirect_to) = redirect_to {
        if session.is_some() {
            leptos_actix::redirect(&redirect_to);
        }
    }
    Ok(session)
}

#[server]
pub async fn register(auth: LoginAuth, new_path: String) -> Result<Option<Session>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");

    let mut server = server_side_data.server.lock()?;
    let session = server.create_new_session(&auth).cloned();
    Ok(session)
    //Ok(server.get_session(&auth).cloned())
    //let nav = use_navigate();
    //nav(&new_path, Default::default());
    //Ok(())
}
