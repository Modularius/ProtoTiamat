use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{GroupUuid, RandomGeneration, Timestamp, Uuid};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserUuid(pub Uuid);

impl Into<UserUuid> for String {
    fn into(self) -> UserUuid {
        UserUuid(self)
    }
}

impl ToString for UserUuid {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserData {
    pub id: UserUuid,
    pub name: String,
    pub datetime_joined: Timestamp,
    pub properties: Option<HashMap<String, String>>,    // Optional until can figure out how to make jsonserde parse empty containers
    pub groups: Option<Vec<GroupUuid>>,
    pub friends: Option<Vec<Friendship>>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Friendship {
    pub user_id: UserUuid,
    pub datetime_of_friendship: Timestamp,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use rand::prelude::IndexedRandom;
        use crate::Store;

        #[derive(Clone, Debug)]
        pub struct User {
            pub data: UserData,
            pub store: Store,
        }

        impl User {
            pub fn new(data: UserData) -> Self {
                Self {
                    data,
                    store: Default::default(),
                }
            }

            pub fn add_friendship(&mut self, friendship: Friendship) {
                self.data.friends
                    .get_or_insert_default()
                    .push(friendship);
            }

            pub fn add_group(&mut self, group_id: GroupUuid) {
                self.data.groups
                    .get_or_insert_default()
                    .push(group_id);
            }
        }

        impl RandomGeneration for UserData {
            type Parameter = UserUuid;

            fn new_random(id: UserUuid) -> Self {
                let first = *["Aaron", "April", "Abdul", "Bobby", "Beth", "Charlie", "Mike", "Laura", "Sandy", "Tamir", "Umar", "Zacahry"]
                    .as_slice()
                    .choose(&mut rand::rng())
                    .expect("");

                let last = *["Appleton", "Cooby", "Faisal", "Genty", "Harris", "Landau", "Gupta", "O'Malley", "Rabbiter", "Singh", "Tellers", "Vivvy", "Waxford", "Xanthys"]
                    .as_slice()
                    .choose(&mut rand::rng())
                    .expect("");

                let name = [first, last].join(" ");

                Self {
                    id,
                    name,
                    ..Default::default()
                }
            }
        }
    }
}
