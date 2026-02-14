mod group;
mod post;
mod session;
mod user;

use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use group::{GroupUuid, GroupData};
pub use session::{SessionUuid, Session};
pub use user::{UserUuid, UserData};
pub use post::PostUuid;

/// Used by instances of the website to refer to server-side sessions.
type Uuid = String;
/// The timestamp type with timezone.
pub type Timestamp = DateTime<Utc>;
pub type Real = f64;

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
        mod feed;
        mod server;
        mod store;
        pub use server::Server;
        pub use group::{Group, Member};
        pub use user::User;
        pub use post::Post;
        pub use feed::Feed;
        pub(crate) use store::Store;

        use rand::seq::IteratorRandom;
        pub trait Uuidlike {
            fn generate_random(size: usize) -> Self;
        }

        impl Uuidlike for Uuid {
            fn generate_random(size: usize) -> Self {
                let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
                (0..size).map(|_|
                    alphabet.iter()
                        .choose(&mut rand::rng())
                        .to_owned()
                        .unwrap()
                    ).collect::<Uuid>()
            }
        }
    }
}
