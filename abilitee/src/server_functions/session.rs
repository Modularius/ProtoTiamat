use cfg_if::cfg_if;
use leptos::prelude::*;
use libertee::{LiberteeError, LoginAuth, Session, SessionUuid};
use tracing::debug;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos_actix::extract;
        use actix_identity::Identity;
        use actix_web::{HttpMessage, HttpRequest};
        use crate::{ServerSideData, structs::ContextExt};
    }
}

#[server]
#[tracing::instrument(level = "debug", err(level = "warn"))]
pub async fn get_session_from_identity() -> Result<Option<SessionUuid>, ServerFnError> {
    let identity = match extract::<Identity>().await {
        Ok(identity) => identity,
        Err(ServerFnErrorErr::ServerError(err_str)) => {
            if err_str == "There is no identity information attached to the current session" {
                return Ok(None);
            } else {
                return Err(ServerFnError::ServerError(err_str));
            }
        }
        Err(e) => Err(e)?,
    };
    match identity.id() {
        Ok(id) => {
            let server_mutex = use_context::<ServerSideData>().expect_context().server;
            let server = server_mutex.lock()?;
            Ok(Some(
                server
                    .get_session(&SessionUuid(id))
                    .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?
                    .uuid
                    .clone(),
            ))
        }
        Err(_e) => Ok(None),
    }
}

#[server]
#[tracing::instrument(level = "debug", err(level = "warn"))]
pub async fn perform_login(
    auth: LoginAuth,
    redirect_to: Option<String>,
) -> Result<SessionUuid, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();

    let mut server = server_side_data.server.lock()?;

    let session = server
        .create_new_session(&auth)
        .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;

    let request = extract::<HttpRequest>()
        .await
        .expect("Request should exist.");

    debug!("Beginning Login.");
    Identity::login(&request.extensions(), session.uuid.to_string())?;
    debug!("Login Successful.");

    if let Some(redirect_to) = redirect_to {
        debug!("Redirecting.");
        leptos_actix::redirect(&redirect_to);
    }
    Ok(session.uuid.clone())
}

#[server]
#[tracing::instrument(level = "debug", err(level = "warn"))]
pub async fn perform_logout(redirect_to: Option<String>) -> Result<bool, ServerFnError> {
    let result = match extract::<Identity>().await {
        Ok(identity) => {
            let id = identity.id()?;
            debug!("Identity found with id {id}");

            let server_side_data = use_context::<ServerSideData>().expect_context();

            identity.logout();
            debug!("Identity logged out.");

            let mut server = server_side_data.server.lock()?;
            if let Some(session) = server.remove_session(&SessionUuid(id)) {
                debug!("Successfully removed session {:?}.", session.uuid);
            }
            true
        }
        Err(ServerFnErrorErr::ServerError(err_str)) => {
            debug!("{err_str}");
            if err_str != "There is no identity information attached to the current session" {
                return Err(ServerFnError::ServerError(err_str));
            }
            false
        }
        Err(e) => Err(e)?,
    };

    if let Some(redirect_to) = redirect_to {
        debug!("Redirecting.");
        leptos_actix::redirect(&redirect_to);
    }

    Ok(result)
}

#[server]
pub async fn register(auth: LoginAuth, _new_path: String) -> Result<Session, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();

    let mut server = server_side_data.server.lock()?;
    let session = server
        .create_new_session(&auth)
        .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;
    Ok(session.clone())
}
