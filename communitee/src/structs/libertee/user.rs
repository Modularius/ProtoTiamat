use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Uuid;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    pub name: String,
    pub properties: HashMap<String, String>,
    pub groups: Vec<Uuid>,
    pub friends: Vec<Uuid>,
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

        impl UserData {
            pub(crate) fn new_random(id: Uuid) -> Self {
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