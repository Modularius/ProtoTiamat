mod public_url;
#[cfg(feature = "ssr")]
mod server_only;

#[cfg(feature = "ssr")]
use clap::Args;
use serde::{Deserialize, Serialize};

pub use public_url::PublicUrl;
#[cfg(feature = "ssr")]
pub use server_only::{InitialUserData, Server, ServerSideData};

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
