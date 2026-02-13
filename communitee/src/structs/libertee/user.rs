use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Timestamp, Uuid, structs::libertee::RandomGeneration};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserData {
    pub id: String,
    pub name: String,
    pub datetime_joined: Timestamp,
    pub properties: HashMap<String, String>,
    pub groups: Vec<Uuid>,
    pub friends: Vec<Friendship>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Friendship {
    pub user_id: Uuid,
    pub datetime_of_friendship: Timestamp,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use rand::prelude::IndexedRandom;
        use crate::structs::libertee::Feed;

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

        impl RandomGeneration for UserData {
            type Parameter = Uuid;

            fn new_random(id: Uuid) -> Self {
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
