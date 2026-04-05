mod session_store;

use cfg_if::cfg_if;
use chrono::SubsecRound;
use leptos::prelude::*;
use libertee::{LiberteeError, LoginAuth, Session, SessionUuid, Timestamp};
use tracing::debug;

pub use session_store::SessionStorage;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::ServerSideData;
        use actix_identity::Identity;
        use actix_web::{HttpMessage, HttpRequest};
        use leptos_actix::extract;
    }
}
