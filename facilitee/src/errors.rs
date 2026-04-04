use leptos::{prelude::{FromServerFnError, ServerFnErrorErr}, server_fn::codec::JsonEncoding};
use serde::{Deserialize, Serialize};
use thiserror;

use libertee::{LiberteeError, GroupUuid, LoginAuth, PostUuid, SessionUuid, UserUuid};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum FaciliteeError {
    #[error("{0}")]
    Libertee(#[from] LiberteeError),
    #[error("Server Function Error: {0}")]
    ServerFn(#[from] ServerFnErrorErr),
}

impl FromServerFnError for FaciliteeError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self::from(value)
    }
}