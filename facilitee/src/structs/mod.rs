mod public_url;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

//pub use libertee::{GroupData, GroupUuid, LoginAuth, Session, SessionUuid, UserData, UserUuid, PostUuid, RandomGeneration};
pub use public_url::PublicUrl;

pub trait Expect: Sized {
    const EXPECT: &'static str;
}

pub trait ContextExt {
    type Inner: Expect;
    fn expect_context(self) -> Self::Inner;
}

impl<T> ContextExt for Option<T>
where
    T: Expect,
{
    type Inner = T;

    #[inline]
    #[track_caller]
    fn expect_context(self) -> Self::Inner {
        self.expect(Self::Inner::EXPECT)
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use clap::{Args};
        use std::sync::{Arc, Mutex};
        pub use libertee::{User, Server, Post, Member, Group};

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
            pub initial_user_password: String
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

impl Expect for ClientSideData {
    const EXPECT: &'static str = "ClientSideData should be provided, this should never fail.";
}
