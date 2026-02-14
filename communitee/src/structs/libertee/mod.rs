mod feed;
mod group;
mod post;
mod session;
mod user;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

pub use group::{GroupUuid, GroupData};
pub use session::{SessionUuid, Session};
pub use user::{UserUuid, UserData};

pub trait RandomGeneration {
    type Parameter;

    fn new_random(param: Self::Parameter) -> Self;
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct LoginAuth {
    pub username: String,
    pub password: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        mod server;
        mod store;
        pub use server::Server;
        pub use group::{Group, Member};
        pub use user::User;
        pub use post::{Post, PostUuid};
        pub use feed::Feed;
        pub(crate) use store::Store;
    }
}
