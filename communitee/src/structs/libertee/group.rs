use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Real, Timestamp, Uuid, structs::UserData};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupData {
    pub id: String,
    pub name: String,
    pub members: HashMap<Uuid, Member>,
    pub adjacent_groups: Vec<(Uuid, Real)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub user: Uuid,
    pub joined: Timestamp,
    pub delegates: HashMap<Uuid, Real>,
}

impl Member {
    pub fn new(id: Uuid, user: Uuid) -> Self {
        Self {
            id,
            user,
            joined: Utc::now(),
            delegates: Default::default(),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use rand::seq::IndexedRandom;

        #[derive(Clone, Debug)]
        pub struct Group {
            pub(crate) data: GroupData,
            pub(crate) feed: super::Feed,
        }

        impl Group {
            pub(crate) fn new(data: GroupData) -> Self {
                Self {
                    data,
                    feed: Default::default(),
                }
            }

            pub(crate) fn add_member(&mut self, user_id: Uuid) {
                let member_id = format!("{}", self.data.members.len());
                self.data
                    .members
                    .insert(member_id.clone(), Member::new(member_id, user_id.clone()));
            }
        }

        impl GroupData {
            pub(crate) fn new_random(id: Uuid) -> Self {
                Self {
                    id,
                    name: ["UK", "Music", "UK Music", "Science", "Space", "Wanking About", "Product of Inbreeding (Self Help)"]
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
