use std::{collections::HashMap, fmt::Display};

use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};

use crate::{Timestamp, UserUuid, Uuid};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SessionUuid(pub Uuid);

impl From<String> for SessionUuid {
    fn from(val: String) -> Self {
        Self(val)
    }
}

impl Display for SessionUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub uuid: SessionUuid,
    pub user: UserUuid,
    //pub user_data: UserData,
    started: Timestamp,
    ttl: TimeDelta,
    state: HashMap<String, String>,
}

impl Session {
    pub fn new(
        uuid: SessionUuid,
        user: UserUuid,
        state: HashMap<String, String>,
        //user_data: UserData,
    ) -> Self {
        Self {
            uuid,
            user,
            //user_data,
            started: Utc::now(),
            ttl: TimeDelta::days(7),
            state,
        }
    }

    pub fn get_state(&self) -> HashMap<String, String> {
        self.state.clone()
    }

    pub fn set_state(&mut self, state: HashMap<String, String>) {
        self.state = state;
    }
}
