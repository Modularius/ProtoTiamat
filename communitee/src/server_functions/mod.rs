use cfg_if::cfg_if;
use chrono::SubsecRound;
use leptos::prelude::*;

use libertee::{LoginAuth, Session, Timestamp};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::ServerSideData;
    }
}

pub(crate) fn format_datetime(datetime: &Timestamp) -> String {
    let date = datetime.date_naive();
    let time = datetime.time().trunc_subsecs(0);
    format!("{}, {}", date.to_string(), time.to_string())
}

pub async fn require_login() -> Result<Option<Session>, ServerFnError> {
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

#[server]
pub async fn perform_login(
    auth: LoginAuth,
    new_path: String,
) -> Result<Option<Session>, ServerFnError> {
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
