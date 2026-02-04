mod session;
mod group;
mod user;

use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{Real, Timestamp, Uuid};

pub use session::Session;
pub use group::{GroupData, Member};
pub use user::UserData;

#[derive(Default, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct LoginAuth {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostData {
    pub id: Uuid,
    pub author: Uuid,
    pub posted_at: Timestamp,
    pub content: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        mod server;
        use rand::seq::IndexedRandom;
        pub use server::Server;
        pub use group::Group;
        pub use user::User;

        #[derive(Default, Clone, Debug)]
        pub struct Feed {
            pub(crate) posts: Vec<Post>
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Post {
            pub(crate) data: PostData,
            pub(crate) replies: Vec<Post>,
            pub(crate) promotions: Real,
        }

        impl Post {
            pub(crate) fn new_random(id: Uuid, author: Uuid) -> Self {
                let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
                Self {
                    data: PostData {
                        id,
                        author,
                        posted_at: Utc::now(),
                        content: (0..rand::random_range(4..10)).map(|_|
                            (0..rand::random_range(3..10)).map(|_|
                                alphabet.choose(&mut rand::rng())
                                    .to_owned()
                                    .unwrap()
                            ).collect::<String>()
                        )
                        .collect::<Vec<_>>()
                        .join(" ")
                    },
                    replies: Default::default(),
                    promotions: 0.0
                }
            }
        }
    }
}
