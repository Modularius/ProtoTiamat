mod session;

use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Real, Timestamp, Uuid};

pub use session::Session;

#[derive(Default, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct LoginAuth {
    pub username: String,
    pub password: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupData {
    name: String,
    members: HashMap<Uuid, Member>,
    adjacent_groups: Vec<(Uuid, Real)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    user: Uuid,
    joined: Timestamp,
    delegates: HashMap<Uuid, Real>,
}

impl Member {
    pub fn new(user: Uuid) -> Self {
        Self {
            user,
            joined: Utc::now(),
            delegates: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostData {
    pub author: Uuid,
    pub posted_at: Timestamp,
    pub content: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub name: String,
    pub properties: HashMap<String, String>,
    pub groups: Vec<Uuid>,
    pub friends: Vec<Uuid>,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        mod server;
        use rand::seq::{IndexedRandom, IteratorRandom};
        pub use server::Server;

        #[derive(Clone, Debug)]
        pub struct Group {
            data: GroupData,
            feed: Feed,
        }

        impl Group {
            pub(crate) fn new(data: GroupData) -> Self {
                Self {
                    data,
                    feed: Default::default(),
                }
            }
        }

        impl GroupData {
            pub(crate) fn new_random() -> Self {
                Self {
                    name: ["UK", "Music", "UK Music", "Science", "Space", "Wanking About", "Product of Inbreeding (Self Help)"]
                        .as_slice()
                        .choose(&mut rand::rng())
                        .expect("")
                        .to_string(),
                    ..Default::default()
                }
            }
        }

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
            pub(crate) fn new_random(author: Uuid) -> Self {
                let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
                Self {
                    data: PostData {
                        author: author,
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

        #[derive(Clone, Debug)]
        pub struct User {
            pub(crate) data: UserData,
            pub(crate) feed: Feed,
        }

        impl User {
            pub(crate) fn new(data: UserData) -> Self {
                Self {
                    data,
                    feed: Default::default(),
                }
            }
        }

        impl UserData {
            pub(crate) fn new_random() -> Self {
                Self {
                    name: ["Aaron", "April", "Abdul", "Bobby", "Beth", "Charlie", "Mike", "Laura", "Sandy", "Tamir", "Umar", "Zacahry"]
                        .as_slice()
                        .choose(&mut rand::rng())
                        .expect("")
                        .to_string(),
                    ..Default::default()
                }
            }
        }
    }
}
