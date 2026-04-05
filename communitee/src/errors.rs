use leptos::{
    prelude::{FromServerFnError, ServerFnErrorErr},
    server_fn::codec::JsonEncoding,
};
use libertee::LiberteeError;
use serde::{Deserialize, Serialize};
use thiserror;

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum CommuniteeError {
    #[error("{0}")]
    Libertee(#[from] LiberteeError),
    #[error("Server Function Error: {0}")]
    ServerFn(#[from] ServerFnErrorErr),
}

impl FromServerFnError for CommuniteeError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self::from(value)
    }
}
