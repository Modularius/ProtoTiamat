pub mod components;
pub mod generic_components;
pub mod guards;

use leptos::prelude::*;
use libertee::SessionUuid;
use tracing::Span;
use crate::{server_functions::{PerformLogin, PerformLogout}, structs::{ClientSideData, Expect}};

pub use components::SubmitPost;

/// This struct enable a degree of type-checking for the [use_context]/[use_context] functions.
/// Any component making use of the following fields should call `use_context::<TopLevelContext>()`
/// and select the desired field.
#[derive(Clone)]
pub struct TopLevelContext {
    pub client_side_data: ClientSideData,
    pub session_id_res: Resource<Result<Option<SessionUuid>, ServerFnError>>,
    //pub session_actions: Box<dyn SessionActions>,
    pub session_id: Signal<Option<SessionUuid>>,
    pub login: ServerAction<PerformLogin>,
    pub logout: ServerAction<PerformLogout>,
    pub span: Span,
}

impl TopLevelContext {
    #[inline]
    #[track_caller]
    pub fn session_id_expect(&self) -> SessionUuid {
        self.session_id_res
            .get()
            .unwrap()
            .expect("session_id_expect should only be called inside <SessionGuard>, this should never fail.")
            .expect("session_id_expect should only be called inside <IsLoggedIn>, this should never fail.")
    }

    #[inline]
    #[track_caller]
    pub fn login_expect(&self) -> SessionUuid {
        self.login
            .value()
            .get()
            .expect("session_id_expect should only be called inside <SessionGuard>, this should never fail.")
            .expect("session_id_expect should only be called inside <IsLoggedIn>, this should never fail.")
    }
}

impl Expect for TopLevelContext {
    const EXPECT: &'static str = "`TopLevelContext` should be provided, this should never fail.";
}
