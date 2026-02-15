use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    Timestamp, Uuid,
    structs::libertee::{GroupUuid, RandomGeneration},
};

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
    pub properties: HashMap<String, String>,
    pub groups: Vec<GroupUuid>,
    pub friends: Vec<Friendship>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Friendship {
    pub user_id: UserUuid,
    pub datetime_of_friendship: Timestamp,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use rand::prelude::IndexedRandom;
        use crate::structs::libertee::Store;

        #[derive(Clone, Debug)]
        pub struct User {
            pub(crate) data: UserData,
            pub(crate) store: Store,
        }

        impl User {
            pub(crate) fn new(data: UserData) -> Self {
                Self {
                    data,
                    store: Default::default(),
                }
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
                    properties: HashMap::from([("Personality".into(), "Bastard".into())]),
                    ..Default::default()
                }
            }
        }
    }
}
