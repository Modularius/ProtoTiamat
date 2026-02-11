mod public_url;
mod libertee;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

pub use libertee::{GroupData, LoginAuth, Session, UserData};
pub use public_url::PublicUrl;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use clap::Args;
        use std::sync::{Arc, Mutex};
        pub use libertee::{User, Server, Post, Feed, Member, Group};

        /// Encapsulates all run-time settings which are only available to the server.
        #[derive(Default, Clone)]
        pub struct ServerSideData {
            pub server: Arc<Mutex<Server>>,
        }
    }
}

/// Contains the settings defined in the CLI used as default values in the UI's inputs.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Args))]
pub struct DefaultData {}

/// Encapsulates all run-time settings which are available to the client.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientSideData {
    pub default_data: DefaultData,
    pub public_url: PublicUrl,
}