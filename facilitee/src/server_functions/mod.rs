mod session;

use cfg_if::cfg_if;
use chrono::SubsecRound;
use leptos::prelude::*;
use libertee::{LiberteeError, LoginAuth, Session, SessionUuid, Timestamp};
pub use session::{PerformLogin, PerformLogout, Register, get_session_from_identity};
use tracing::debug;

use crate::structs::ContextExt;

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
