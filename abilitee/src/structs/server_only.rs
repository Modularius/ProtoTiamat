use clap::Args;
pub use libertee::Server;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::Expect;

/// Encapsulates all run-time settings which are only available to the server.
#[derive(Default, Clone)]
pub struct ServerSideData {
    pub server: Arc<Mutex<Server>>,
}

impl Expect for ServerSideData {
    const EXPECT: &'static str = "ServerSideData should be provided, this should never fail.";
}

/// Contains the settings defined in the CLI used as default values in the UI's inputs.
#[derive(Default, Clone, Debug, Serialize, Deserialize, Args)]
pub struct InitialUserData {
    #[clap(long)]
    pub initial_user_name: String,

    #[clap(long)]
    pub initial_user_username: String,

    #[clap(long)]
    pub initial_user_password: String,
}
