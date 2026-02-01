mod session;

use cfg_if::cfg_if;
use chrono::Utc;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
            posts: Vec<Post>
        }
        
        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Post {
            data: PostData,
            replies: Vec<Post>,
            promotions: Real,
        }
        
        #[derive(Clone, Debug)]
        pub struct User {
            data: UserData,
            feed: Feed,
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